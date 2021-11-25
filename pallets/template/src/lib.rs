#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use sp_runtime::traits::Hash;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type WeightInfo: crate::WeightInfo;
	}

	pub trait WeightInfo {
		fn store_something(_s: u32, ) -> Weight;
		fn benign_repeat_hashing(i: u32, ) -> Weight;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	pub type Sum<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
		/// The resulting hash of all our work
		TheHash(T::Hash),
		/// The calculated sum
		TheSum(u32),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// An overflow occured while calculating.
		Overflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(T::WeightInfo::store_something(*something))]
		pub fn store_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Removes the `Something` storage item.
		#[pallet::weight(10_000_000 + T::DbWeight::get().writes(1))]
		pub fn remove_something(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			<Something<T>>::take();
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::Overflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		/// Does repeated hashing based on the given parameter. Can take a very very long time for
		/// big enough numbers and create overweight blocks.
		#[pallet::weight(10_000_000)]
		pub fn dangerous_repeat_hashing(origin: OriginFor<T>, times: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_repeat_hashing(times, &who);
			Ok(())
		}

		/// Does repeated hashing based on the given parameter. Can only take up to the maximum
		/// block weight in computation because it is benchmarked and weighted.
		#[pallet::weight(T::WeightInfo::benign_repeat_hashing(*times))]
		pub fn benign_repeat_hashing(origin: OriginFor<T>, times: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::do_repeat_hashing(times, &who);
			Ok(())
		}

		/// Triggers an overflow if the given value is big enough.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn overflow(origin: OriginFor<T>, added: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let prev = Something::<T>::get().unwrap_or(0);

			let val= prev + added;
			Something::<T>::put(val);

			Self::deposit_event(Event::SomethingStored(val, who));
			Ok(())
		}

		/// Does not overflow because of checked addition.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn no_overflow(origin: OriginFor<T>, added: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;

			let prev = Something::<T>::get().unwrap_or(0);

			let val = prev.checked_add(added).ok_or(Error::<T>::Overflow)?;
			Something::<T>::put(val);

			Self::deposit_event(Event::SomethingStored(val, who));
			Ok(())
		}

		/// Unwrap is BAD.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 1))]
		pub fn unwrap_is_bad(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			// `.unwrap()` is BAD, DON'T USE IT in the runtime.
			let prev = Something::<T>::get().unwrap();

			let val = prev.checked_add(1).ok_or(Error::<T>::Overflow)?;
			Something::<T>::put(val);

			Self::deposit_event(Event::SomethingStored(val, who));
			Ok(())
		}

		/// Does not update storage transactionally.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn non_transactional_sum(origin: OriginFor<T>, val: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let prev = Something::<T>::get().unwrap_or(0);
			Something::<T>::put(val);

			let sum = prev.checked_add(val).ok_or(Error::<T>::Overflow)?;
			Sum::<T>::put(sum);

			Self::deposit_event(Event::TheSum(sum));
			Ok(())
		}

		/// Updates either both or none of the storage items -> transactional.
		#[pallet::weight(10_000_000 + T::DbWeight::get().reads_writes(1, 2))]
		pub fn transactional_sum(origin: OriginFor<T>, val: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let prev = Something::<T>::get().unwrap_or(0);
			let sum = prev.checked_add(val).ok_or(Error::<T>::Overflow)?;

			Something::<T>::put(val);
			Sum::<T>::put(sum);

			Self::deposit_event(Event::TheSum(sum));
			Ok(())
		}
	}
}
impl<T: Config> Pallet<T> {
	fn do_repeat_hashing(times: u32, acc: &T::AccountId) {
		let mut hashed = T::Hashing::hash_of(acc);
		for _i in 0..times {
			hashed = T::Hashing::hash(&hashed.as_ref());
		}

		Self::deposit_event(Event::TheHash(hashed));
	}
}
