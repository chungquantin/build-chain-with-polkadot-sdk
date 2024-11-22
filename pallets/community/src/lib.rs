//! A shell pallet built with [`frame`].
//!
//! To get started with this pallet, try implementing the guide in
//! <https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
use frame::traits::{BuildGenesisConfig, EnsureOrigin, Get, IsType};
use polkadot_sdk::polkadot_sdk_frame as frame;

// Re-export all pallet parts, this is needed to properly import the pallet into the runtime.
pub use pallet::*;

#[frame::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: polkadot_sdk::frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as polkadot_sdk::frame_system::Config>::RuntimeEvent>;

        /// Required origin for adding a member (though can always be Root).
        type AddOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Required origin for removing a member (though can always be Root).
        type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The maximum number of members that this membership can have.
        type MaxMembers: Get<u32>;
    }

    #[pallet::storage]
    pub type Members<T: Config> =
        StorageValue<_, BoundedVec<T::AccountId, T::MaxMembers>, ValueQuery>;

    #[pallet::genesis_config]
    #[derive(frame::deps::frame_support::DefaultNoBound)]
    pub struct GenesisConfig<T: Config> {
        pub members: BoundedVec<T::AccountId, T::MaxMembers>,
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            use sp_std::collections::btree_set::BTreeSet;
            let members_set: BTreeSet<_> = self.members.iter().collect();
            assert_eq!(
                members_set.len(),
                self.members.len(),
                "Members cannot contain duplicate accounts."
            );

            let mut members = self.members.clone();
            members.sort();
            Members::<T>::put(members);
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// The given member was added; see the transaction for who.
        MemberAdded,
        /// The given member was removed; see the transaction for who.
        MemberRemoved,
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Already a member.
        AlreadyMember,
        /// Not a member.
        NotMember,
        /// Too many members.
        TooManyMembers,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::call]
    impl<T: Config> Pallet<T> {}
}
