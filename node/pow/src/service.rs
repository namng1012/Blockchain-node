//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use pow_runtime::{self, opaque::Block, RuntimeApi};
// use sc_client_api::BlockBackend;
pub use sc_executor::NativeElseWasmExecutor;
// use sc_finality_grandpa::SharedVoterState;
use sc_keystore::LocalKeystore;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager, PartialComponents};
use sc_telemetry::{Telemetry, TelemetryWorker};
// use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sha3pow::*;
use std::{sync::Arc, time::Duration};
use sp_core::{
	Encode, U256,
};
use std::thread;

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
	/// Only enable the benchmarking host functions when we actually want to benchmark.
	#[cfg(feature = "runtime-benchmarks")]
	type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
	/// Otherwise we only use the default Substrate host functions.
	#[cfg(not(feature = "runtime-benchmarks"))]
	type ExtendHostFunctions = ();

	fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
		pow_runtime::api::dispatch(method, data)
	}

	fn native_version() -> sc_executor::NativeVersion {
		pow_runtime::native_version()
	}
}

pub(crate) type FullClient =
	sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

type POWBlockImport = sc_consensus_pow::PowBlockImport<
	Block,
	Arc<FullClient>,
	FullClient,
	FullSelectChain,
	MinimalSha3Algorithm,
	Box<
		dyn sp_inherents::CreateInherentDataProviders<
			Block,
			(),
			InherentDataProviders=sp_timestamp::InherentDataProvider,
		>,
	>,
>;

pub fn new_partial(
	config: &Configuration,
) -> Result<
	PartialComponents<
		FullClient,
		FullBackend,
		FullSelectChain,
		sc_consensus::DefaultImportQueue<Block, FullClient>,
		sc_transaction_pool::FullPool<Block, FullClient>,
		(
			POWBlockImport,
			Option<Telemetry>,
		),
	>,
	ServiceError,
> {
	if config.keystore_remote.is_some() {
		return Err(ServiceError::Other("Remote Keystores are not supported.".into()))
	}

	let telemetry = config
		.telemetry_endpoints
		.clone()
		.filter(|x| !x.is_empty())
		.map(|endpoints| -> Result<_, sc_telemetry::Error> {
			let worker = TelemetryWorker::new(16)?;
			let telemetry = worker.handle().new_telemetry(endpoints);
			Ok((worker, telemetry))
		})
		.transpose()?;

	let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
		config.wasm_method,
		config.default_heap_pages,
		config.max_runtime_instances,
		config.runtime_cache_size,
	);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;
	let client = Arc::new(client);

	let telemetry = telemetry.map(|(worker, telemetry)| {
		task_manager.spawn_handle().spawn("telemetry", None, worker.run());
		telemetry
	});

	let select_chain = sc_consensus::LongestChain::new(backend.clone());

	let transaction_pool = sc_transaction_pool::BasicPool::new_full(
		config.transaction_pool.clone(),
		config.role.is_authority().into(),
		config.prometheus_registry(),
		task_manager.spawn_essential_handle(),
		client.clone(),
	);

	let inherent_data_providers = Box::new(move |_, ()| async move {
		let provider = sp_timestamp::InherentDataProvider::from_system_time();
		Ok(provider)
	})
		as Box<dyn sp_inherents::CreateInherentDataProviders<Block,(),InherentDataProviders=sp_timestamp::InherentDataProvider>>;

	let pow_block_import = sc_consensus_pow::PowBlockImport::new(
		client.clone(),
		client.clone(),
		sha3pow::MinimalSha3Algorithm,
		0, // check inherents starting at block 0
		select_chain.clone(),
		inherent_data_providers
	);

	let import_queue = sc_consensus_pow::import_queue(
		Box::new(pow_block_import.clone()),
		None,
		sha3pow::MinimalSha3Algorithm,
		&task_manager.spawn_essential_handle(),
		config.prometheus_registry(),
	)?;

	Ok(sc_service::PartialComponents {
		client,
		backend,
		task_manager,
		import_queue,
		keystore_container,
		select_chain,
		transaction_pool,
		other: (pow_block_import, telemetry),
	})
}

fn remote_keystore(_url: &String) -> Result<Arc<LocalKeystore>, &'static str> {
	// FIXME: here would the concrete keystore be built,
	//        must return a concrete type (NOT `LocalKeystore`) that
	//        implements `CryptoStore` and `SyncCryptoStore`
	Err("Remote Keystore not supported.")
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
	let PartialComponents {
		client,
		backend,
		mut task_manager,
		import_queue,
		mut keystore_container,
		select_chain,
		transaction_pool,
		other: (pow_block_import, mut telemetry),
	} = new_partial(&config)?;

	if let Some(url) = &config.keystore_remote {
		match remote_keystore(url) {
			Ok(k) => keystore_container.set_remote_keystore(k),
			Err(e) =>
				return Err(ServiceError::Other(format!(
					"Error hooking up remote keystore for {}: {}",
					url, e
				))),
		};
	}

	let (network, system_rpc_tx, tx_handler_controller, network_starter) =
		sc_service::build_network(sc_service::BuildNetworkParams {
			config: &config,
			client: client.clone(),
			transaction_pool: transaction_pool.clone(),
			spawn_handle: task_manager.spawn_handle(),
			import_queue,
			block_announce_validator_builder: None,
			warp_sync: None,
		})?;

	if config.offchain_worker.enabled {
		sc_service::build_offchain_workers(
			&config,
			task_manager.spawn_handle(),
			client.clone(),
			network.clone(),
		);
	}

	let role = config.role.clone();
	let prometheus_registry = config.prometheus_registry().cloned();

	let rpc_extensions_builder = {
		let client = client.clone();
		let pool = transaction_pool.clone();

		Box::new(move |deny_unsafe, _| {
			let deps =
				crate::rpc::FullDeps { client: client.clone(), pool: pool.clone(), deny_unsafe };
			crate::rpc::create_full(deps).map_err(Into::into)
		})
	};

	let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
		network: network.clone(),
		client: client.clone(),
		keystore: keystore_container.sync_keystore(),
		task_manager: &mut task_manager,
		transaction_pool: transaction_pool.clone(),
		rpc_builder: rpc_extensions_builder,
		backend,
		system_rpc_tx,
		tx_handler_controller,
		config,
		telemetry: telemetry.as_mut(),
	})?;

	if role.is_authority() {
		let proposer_factory = sc_basic_authorship::ProposerFactory::new(
			task_manager.spawn_handle(),
			client.clone(),
			transaction_pool,
			prometheus_registry.as_ref(),
			telemetry.as_ref().map(|x| x.handle()),
		);

		let (_worker, worker_task) = sc_consensus_pow::start_mining_worker(
			Box::new(pow_block_import),
			client,
			select_chain,
			MinimalSha3Algorithm,
			proposer_factory, 
			network.clone(), 
			network.clone(),
			None,
			move |_, ()| async move {
				let provider = sp_timestamp::InherentDataProvider::from_system_time();
				Ok(provider)
			},
			// Time to wait for a new block before starting to mine a new one
			Duration::new(10, 0),
			// How long to take to actually build the block (i.e. executing extrinsic)
			Duration::new(10, 0),
		);		


		task_manager
			.spawn_essential_handle()
			.spawn_blocking("pow", Some("block-authoring"), worker_task);

			// Start Mining
		// (from recipes a bit modified)
		let mut nonce: U256 = U256::from(0);
		thread::spawn(move || loop {
			let worker = _worker.clone();
			let metadata = worker.metadata();
			if let Some(metadata) = metadata {
				let compute = Compute {
					difficulty: metadata.difficulty,
					pre_hash: metadata.pre_hash,
					nonce,
				};
				let seal = compute.compute();
				if hash_meets_difficulty(&seal.work, seal.difficulty) {
					nonce = U256::from(0);
					let _ = futures::executor::block_on(worker.submit(seal.encode()));
				} else {
					nonce = nonce.saturating_add(U256::from(1));
					if nonce == U256::MAX {
						nonce = U256::from(0);
					}
				}
			} else {
				thread::sleep(Duration::new(1, 0));
			}
		});
	}

	network_starter.start_network();
	Ok(task_manager)
}
