#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		inherent::Vec, 
		pallet_prelude::*, 
		sp_runtime::traits::Hash, 
		traits::Randomness
	};
	use frame_system::pallet_prelude::*;
	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		
		type HealthRandomness: Randomness<Self::Hash, Self::BlockNumber>;

		type WeightInfo: WeightInfo;
	}

	pub type VecString = Vec<u8>;
	pub type BigNumberType = u64;
	pub type SmallNumberType = u8;
	pub type Bool = bool;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum Gender {
		Male,
		Female,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum RateScale {
		Zero,
		One,
		Two,
		Three,
		Four,
		Five,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum MaritalStatus {
		Single,
		Married,
		Divorced,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum BussinessTravel {
		TravelRarely,
		TravelFrequently,
		NonTravel,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum DepartmentName {
		Cardiology,
		Maternity,
		Neurology,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum FieldName {
		Medical,
		TechnicalDegree,
		Marketing,
		LifeSciences,
		HumanResources,
		Other,
	}

	#[derive(Clone, Encode, Decode, PartialEq, Copy, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	pub enum JobRole {
		Nurse,
		Administrative,
		Therapist,
		Other,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct AccountInfo<T: Config> {
		pub owner: T::AccountId,
		pub employeeid: [SmallNumberType; 7],
		pub hashid: T::Hash,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct IndividualInfo {
		pub age: SmallNumberType,
		pub gender: Gender,
		pub education: RateScale,
		pub education_field: FieldName,
		pub over18: Bool,
		pub department: DepartmentName,
		pub marital_status: MaritalStatus,
		pub distance_from_home: SmallNumberType,
		pub no_companies_worked: SmallNumberType,
		pub monthly_income: BigNumberType,
		pub percent_salary_hike: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct WorkingInfo {
		pub job_involvement: SmallNumberType,
		pub job_level: RateScale,
		pub job_role: JobRole,
		pub hourly_rate: SmallNumberType,
		pub daily_rate: BigNumberType,
		pub monthly_rate: BigNumberType,
		pub standard_hours: SmallNumberType,
		pub overtime: Bool,
		pub bussiness_travel: BussinessTravel,
		pub training_times_lastyear: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct WorkingYearInfo {
		pub total_working_years: SmallNumberType,
		pub year_atcompany: SmallNumberType,
		pub year_incurrent_role: SmallNumberType,
		pub year_inlast_promotion: SmallNumberType,
		pub year_withcurrent_manager: SmallNumberType,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct RatingInfo {
		pub attrition: Bool,
		pub environment_satisfaction: RateScale,
		pub job_satisfaction: RateScale,
		pub performance_rating: RateScale,
		pub relationship_satisfaction: RateScale,
		pub shift: RateScale,
		pub work_life_balance: RateScale,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct HealthWorkerInfo<T: Config> {
		pub account: AccountInfo<T>,
		pub individual_info: IndividualInfo,
		pub working_info: WorkingInfo,
		pub working_year_info: WorkingYearInfo,
		pub rating: RatingInfo,
	}

	#[pallet::storage]
	#[pallet::getter(fn employee_count)]
	pub(super) type EmployeeCount<T> = StorageValue<_, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn account_info_storage)]
	pub(super) type AccountInfoStorage<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, AccountInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn individual_info_storage)]
	pub(super) type IndividualInfoStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AccountInfo<T>,
		IndividualInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn working_info_storage)]
	pub(super) type WorkingInfoStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AccountInfo<T>,
		WorkingInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn working_year_info_storage)]
	pub(super) type WorkingYearStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AccountInfo<T>,
		WorkingYearInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn rating_info_storage)]
	pub(super) type RatingInfoStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AccountInfo<T>,
		RatingInfo,
	>;

	#[pallet::storage]
	#[pallet::getter(fn health_worker_info_storage)]
	pub(super) type HealthWorkerInfoStorage<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		AccountInfo<T>,
		HealthWorkerInfo<T>,
	>;

	#[pallet::error]
	pub enum Error<T> {
		SeedWeightTooHigh
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddAccountInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddIndividualInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddWorkingInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddWorkingYearInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddRatingInfo(T::AccountId, T::Hash, T::BlockNumber),
		AddHealthWorkerInfo(T::AccountId, T::Hash, T::BlockNumber),
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_account_info(
			origin: OriginFor<T>,
			item: [SmallNumberType; 7],
			seed: u32
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let hash = Self::generate_hash(item);
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let account_info =
				AccountInfo::<T> { owner: sender.clone(), employeeid: item, hashid: hash.clone() };
			<AccountInfoStorage<T>>::insert(sender.clone(), account_info);
			Self::deposit_event(Event::AddAccountInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_individual_info(origin: OriginFor<T>, item: IndividualInfo, seed: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let individual_info = IndividualInfo {
				age: item.age,
				gender: item.gender,
				education: item.education,
				education_field: item.education_field,
				over18: item.over18,
				department: item.department,
				marital_status: item.marital_status,
				distance_from_home: item.distance_from_home,
				no_companies_worked: item.distance_from_home,
				monthly_income: item.monthly_income,
				percent_salary_hike: item.percent_salary_hike,
			};
			let hash = Self::generate_hash(&individual_info);
			let account_info = Self::account_info_storage(&sender);
			<IndividualInfoStorage<T>>::insert(
				sender.clone(),
				account_info.unwrap(),
				individual_info,
			);
			Self::deposit_event(Event::AddIndividualInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_working_info(origin: OriginFor<T>, item: WorkingInfo, seed: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let working_info = WorkingInfo {
				job_involvement: item.job_involvement,
				job_level: item.job_level,
				job_role: item.job_role,
				hourly_rate: item.hourly_rate,
				daily_rate: item.daily_rate,
				monthly_rate: item.monthly_rate,
				standard_hours: item.standard_hours,
				overtime: item.overtime,
				bussiness_travel: item.bussiness_travel,
				training_times_lastyear: item.training_times_lastyear,
			};
			let hash = Self::generate_hash(&working_info);
			let account_info = Self::account_info_storage(&sender);
			<WorkingInfoStorage<T>>::insert(sender.clone(), account_info.unwrap(), working_info);
			Self::deposit_event(Event::AddWorkingInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_working_year_info(
			origin: OriginFor<T>,
			item: WorkingYearInfo,
			seed: u32
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let working_year_info = WorkingYearInfo {
				total_working_years: item.total_working_years,
				year_atcompany: item.year_atcompany,
				year_incurrent_role: item.year_incurrent_role,
				year_inlast_promotion: item.year_inlast_promotion,
				year_withcurrent_manager: item.year_withcurrent_manager,
			};
			let hash = Self::generate_hash(&working_year_info);
			let account_info = Self::account_info_storage(&sender);
			<WorkingYearStorage<T>>::insert(
				sender.clone(),
				account_info.unwrap(),
				working_year_info,
			);
			Self::deposit_event(Event::AddWorkingYearInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_rating_info(origin: OriginFor<T>, item: RatingInfo, seed: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let rating_info = RatingInfo {
				attrition: item.attrition,
				environment_satisfaction: item.environment_satisfaction,
				job_satisfaction: item.job_satisfaction,
				performance_rating: item.performance_rating,
				relationship_satisfaction: item.relationship_satisfaction,
				shift: item.shift,
				work_life_balance: item.work_life_balance,
			};
			let hash = Self::generate_hash(&rating_info);
			let account_info = Self::account_info_storage(&sender);
			<RatingInfoStorage<T>>::insert(sender.clone(), account_info.unwrap(), rating_info);
			Self::deposit_event(Event::AddRatingInfo(sender, hash, blocknumber));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::add_info(*seed))]
		pub fn add_health_worker_info(origin: OriginFor<T>, seed: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!( seed <= 10000 as u32, Error::<T>::SeedWeightTooHigh); 
			let blocknumber = <frame_system::Pallet<T>>::block_number();
			let account_info = Self::account_info_storage(&sender).unwrap();
			let patient_info = HealthWorkerInfo::<T> {
				account: account_info.clone(),
				individual_info: Self::individual_info_storage(&sender, &account_info).unwrap(),
				working_info: Self::working_info_storage(&sender, &account_info).unwrap(),
				working_year_info: Self::working_year_info_storage(&sender, &account_info).unwrap(),
				rating: Self::rating_info_storage(&sender, &account_info).unwrap(),
			};
			let hash = Self::generate_hash(&patient_info);
			let new_cnt = Self::employee_count().checked_add(1).unwrap();
			<HealthWorkerInfoStorage<T>>::insert(sender.clone(), account_info, patient_info);
			<EmployeeCount<T>>::put(new_cnt);
			Self::deposit_event(Event::AddHealthWorkerInfo(sender, hash, blocknumber));
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn generate_hash<V>(item: V) -> T::Hash
		where
			V: Encode,
		{
			let tmp = (
				T::HealthRandomness::random(&b"2023/02/16_PhD_Nguyen_Quoc_Duy_Nam"[..]).0,
				item,
				<frame_system::Pallet<T>>::block_number(),
			);
			let hash = T::Hashing::hash_of(&tmp.encode());
			hash
		}
	}
}
