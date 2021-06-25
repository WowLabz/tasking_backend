//! RPC interface for the pallet tasking module.

use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::{generic::BlockId, traits::Block as BlockT};
use std::sync::Arc;
use pallet_tasking_runtime_api::PalletTaskingApi as PalletTaskingRuntimeApi;

#[rpc]
pub trait PalletTaskingApi<BlockHash> {
	#[rpc(name = "palletTasking_getOne")]
	fn get_one(&self, at: Option<BlockHash>) -> Result<u128>;
}

/// A struct that implements the `PalletTaskingApi`.
pub struct TaskStore<C, M> {
	// If you have more generics, no need to TaskStore<C, M, N, P, ...>
	// just use a tuple like TaskStore<C, (M, N, P, ...)>
	client: Arc<C>,
	_marker: std::marker::PhantomData<M>,
}

impl<C, M> TaskStore<C, M> {
	/// Create new `TaskStore` instance with the given reference to the client.
	pub fn new(client: Arc<C>) -> Self {
		Self {
			client,
			_marker: Default::default(),
		}
	}
}

/// Error type of this RPC api.
// pub enum Error {
// 	/// The transaction was not decodable.
// 	DecodeError,
// 	/// The call to runtime failed.
// 	RuntimeError,
// }
//
// impl From<Error> for i64 {
// 	fn from(e: Error) -> i64 {
// 		match e {
// 			Error::RuntimeError => 1,
// 			Error::DecodeError => 2,
// 		}
// 	}
// }

impl<C, Block> PalletTaskingApi<<Block as BlockT>::Hash> for TaskStore<C, Block>
where
	Block: BlockT,
	C: Send + Sync + 'static,
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block>,
	C::Api: PalletTaskingRuntimeApi<Block>,
{
	fn get_one(&self, at: Option<<Block as BlockT>::Hash>) -> Result<u128> {
		let api = self.client.runtime_api();
		let at = BlockId::hash(at.unwrap_or_else(||
			// If the block hash is not supplied assume the best block.
			self.client.info().best_hash));

		let runtime_api_result = api.get_one(&at);
		runtime_api_result.map_err(|e| RpcError {
			code: ErrorCode::ServerError(9876), // No real reason for this value
			message: "Something wrong".into(),
			data: Some(format!("{:?}", e).into()),
		})
	}
}
