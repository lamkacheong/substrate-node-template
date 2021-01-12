#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, StorageValue, StorageMap, dispatch, traits::Get};
use sp_io::hashing::blake2_128;
use frame_system::ensure_signed;
use sp_runtime::DispatchError;

type KittyIndex = u32;
#[derive(Encode, Decode)]
pub struct Kitty(pub [u8; 16]);

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
	type Randomness: Randomness<Self::Hash>;
	// Because this pallet emits events, it depends on the runtime's definition of an event.
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait>as Kitties {
		pub Kitties get(fn kitties):map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
		pub KittiesCount get(fn kitties_count): KittyIndex;
		pub KittyOwner get(fn kitty_owner): map hasher(blake2_128_concat) KittyIndex => Option<Kitty>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::AccountId, {
		KittyCreated(AccountId, KittyIndex)
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait>{
		KittiesCountOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin{
		#[weight = 0]
		pub fn create(origin) {
			let sender = ensure_signed(origin)?
			let kitty_id = Self::next_kitty_id()?
			let dna = Self::random_value(&sender);
			let kitty = Kitty(dna);
			Self::insert_kitty(&sender, kitty_id, kitty);
			Self::deposit_event(RawEvent::KittyCreated(sender, kitty_id));

		}
	}
}

impl<T: Trait> Module<T> {
	fn insert_kitty(owner: &T::AccountId, kitty_id: KittyIndex, kitty: Kitty) {
		Kitties::insert(kitty_id, kitty);
		KittiesCount::put(kitty_id + 1);
		<KittyOwners<T>>::insert(kitty_id, owner);
	}
	fn next_kitty_id() -> sp_std::result::Result<KittyIndex, DispatchError> {
		let kitty_id = Self::kitties_count();
		if kitty_id == KittyIndex::max_value() {
			return Err(Error::<T>::KittiesCountOverflow.into());
		}
		Ok(kitty_id)
	}

	fn random_vale(sender: &T::AccountId) -> [u8;16] {
		let payload = (
			T::Randomness::random_seed(),
			&sender,
			<frame_system::Module<T>::extrinsic_index(),
		);
		payload.using_encoded(blake2_128)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
}
