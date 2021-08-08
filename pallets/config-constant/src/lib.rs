#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::sp_runtime::traits::Zero;
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type MaxAddend: Get<u32>;
        type ClearFrequency: Get<Self::BlockNumber>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn single_value)]
    pub type SingleValue<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        Added(u32, u32, u32),
        Cleared(u32),
    }

    #[pallet::error]
    pub enum Error<T> {
        Overflow,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(n: T::BlockNumber) {
            if (n % T::ClearFrequency::get()).is_zero() {
                let current_value = <SingleValue<T>>::get();
                <SingleValue<T>>::put(0u32);
                Self::deposit_event(Event::Cleared(current_value));
            }
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn add_value(origin: OriginFor<T>, val_to_add: u32) -> DispatchResultWithPostInfo {
            let _ = ensure_signed(origin)?;
            ensure!(
                val_to_add <= T::MaxAddend::get(),
                "value must be <= maximum add amount constant"
            );

            let c_val = SingleValue::<T>::get();

            let result = c_val.checked_add(val_to_add).ok_or(Error::<T>::Overflow)?;

            <SingleValue<T>>::put(result);
            Self::deposit_event(Event::Added(c_val, val_to_add, result));
            Ok(().into())
        }
    }
}
