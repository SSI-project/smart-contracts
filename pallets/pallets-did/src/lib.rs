#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        DIDCreated { did: [u8; 32] },
        IssuerRegistered { issuer: T::AccountId },
        CredentialRevoked { credential_id: [u8; 32] },
    }

    #[pallet::error]
    pub enum Error<T> {
        DuplicateDID,
        UnauthorizedIssuer,
        InvalidCredential,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn create_did(_origin: OriginFor<T>, _did: [u8; 32]) -> DispatchResult {
            Ok(())
        }
    }
}