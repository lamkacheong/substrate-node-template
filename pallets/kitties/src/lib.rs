#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, StorageValue, StorageMap,
	traits::Randomness,
};
use sp_runtime::{
	DispatchError, traits::{
		AtLeast32Bit, Member, MaybeSerialize, MaybeDisplay,
	},
};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_std::prelude::*;

type KittyIndex = u32;
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
	type KittyIndex: Clone + Eq + Member + MaybeSerialize + Default + MaybeDisplay + AtLeast32Bit
	+ Copy;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as Kitties {
		pub Kitties get(fn kitties):map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
		pub KittiesCount get(fn kitties_count): KittyIndex;
		pub KittyOwners get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<T::AccountId>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::AccountId, {
		KittyCreated(AccountId, KittyIndex),
		Transferred(AccountId, AccountId, KittyIndex),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait>{
		KittiesCountOverflow,
		InvalidKittyId,
		RequireDifferentParent,
	}
}

impl<T: Trait> Module<T> {
	pub fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndex, kitty: Kitty) {
		Kitties::insert(kitty_id, kitty);
		KittiesCount::put(kitty_id + 1);
		<KittyOwners<T>>::insert(kitty_id, owner);
	}
	fn next_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
		let kitty_id = KittiesCount::get();
		if kitty_id == KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

	fn random_value(sender: &T::AccountId) -> [u8;16] {
		let payload = (T::Randomness::random_seed(),
					   &sender, <frame_system::Module<T>>::extrinsic_index());
		payload.using_encoded(blake2_128)
	}

	fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
		(selector & dna1) | (!selector & dna2)
	}

	fn do_breed(sender: &T::AccountId, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) -> sp_std::result::Result<KittyIndex, DispatchError>{
		let kitty1 = Self::kitties(kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
		let kitty2 = Self::kitties(kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

		ensure!(kitty_id_1 != kitty_id_2, Error::<T>::RequireDifferentParent);

		let kitty_id = Self::next_kitty_id()?;

		let kitty1_dna = kitty1.0;
		let kitty2_dna = kitty2.0;
		let selector = Self::random_value(&sender);
		let mut new_dna = [0u8; 16];

		for i in 0..kitty1_dna.len() {
			new_dna[i] = Self::combine_dna(kitty1_dna[i], kitty2_dna[i], selector[i]);
		}
		Self::insert_kitty(sender, kitty_id, Kitty(new_dna));
		Ok(kitty_id)
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{
		type Error = Error<T>;
		fn deposit_event() = default;

		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?;
			let kitty_id = Self::next_kitty_id()?;
			let dna = Self::random_value(&sender);
			let kitty = Kitty(dna);
			Self::insert_kitty(&sender, kitty_id, kitty);
			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id));
		}

		#[weight = 0]
		pub fn transfer(origin, to: T::AccountId, kitty_id: KittyIndex) {
			let sender = ensure_signed(origin)?;
			Self::kitties(kitty_id).ok_or(Error::<T>::InvalidKittyId)?;
			<KittyOwners<T>>::insert(kitty_id, to.clone());
			Self::deposit_event(RawEvent::Transferred(sender, to, kitty_id));
		}

		#[weight = 0]
		pub fn breed(origin, kitty_id_1: KittyIndex, kitty_id_2: KittyIndex) {
			let sender = ensure_signed(origin)?;
			let new_kitty_id = Self::do_breed(&sender, kitty_id_1, kitty_id_2)?;
			Self::deposit_event(RawEvent::KittyCreated(sender, new_kitty_id));
		}
	}
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod mock;
