#![cfg_attr(not(feature = "std"), no_std)]

sp_api::decl_runtime_apis! {
	/// The API to get one
	pub trait PalletTaskingApi {
		/// Verify get one
		fn get_one() -> u128;
	}
}