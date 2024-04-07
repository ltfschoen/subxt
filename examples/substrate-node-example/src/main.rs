#![allow(missing_docs)]

use reconnecting_jsonrpsee_ws_client::{Client, ExponentialBackoff, PingConfig};
use std::time::Duration;
// use subxt::error::{Error, RpcError};
use subxt::backend::{legacy::LegacyRpcMethods, rpc::RpcClient};
use subxt::backend::rpc::RpcClientT;
use subxt::config::DefaultExtrinsicParamsBuilder as Params;
use subxt::{OnlineClient, PolkadotConfig};
use subxt_signer::sr25519::dev;
// use pallet_identity::{legacy::IdentityInfo, Data};
// use sp_runtime::BoundedVec;
// use sp_core::bounded_vec::BoundedVec;
// use frame_support::BoundedVec;
// use alloc::vec::Vec;
use codec::{Decode, Encode, MaxEncodedLen};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use sp_core::ConstU32;

#[subxt::subxt(runtime_metadata_path = "../../artifacts/substrate_node_metadata.scale")]
pub mod runtime {}

// use crate::runtime::runtime_types::sp_core::bounded_vec::BoundedVec; // Not found
// TODO: Why necessary to use the below instead of `use sp_runtime::BoundedVec;`?
use crate::runtime::runtime_types::bounded_collections::bounded_vec::BoundedVec4;
use crate::runtime::runtime_types::pallet_identity::legacy::{IdentityInfo};
use crate::runtime::runtime_types::pallet_identity::types::{Data};

// TODO: Why does I have to do this (copy from Polkadot SDK path frame/identity/src/types.rs)
// to avoid the following error when I use `Data::default()`:
// `variant or associated item not found in `Data`
impl Default for Data {
	fn default() -> Self {
		Self::None
	}
}

impl<Data> From<Vec<Data>> for BoundedVec4<Data> {
	fn from(v: Vec<Data>) -> Self {
		BoundedVec4(v)
	}
}

// newtype pattern
// https://stackoverflow.com/a/25415289/3208553
struct RpcClientT(RpcClient);
impl From<Client> for RpcClientT {
    fn from(c: Client) -> Self {
		c.into()
	}
}

// struct RpcClientZ(Box<dyn RpcClientT>);
// impl From<Client> for RpcClientZ {
//     fn from(c: Client) -> Self {
// 		c.into()
// 	}
// }

struct ClientT(Client);
impl From<RpcClient> for ClientT {
    fn from(c: RpcClient) -> Self {
		c.into()
	}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with with a reconnecting RPC client.
    let rpc = Client::builder()
        // Reconnect with exponential backoff
        //
        // This API is "iterator-like" so one could limit it to only
        // reconnect x times and then quit.
        .retry_policy(ExponentialBackoff::from_millis(100).max_delay(Duration::from_secs(10)))
        // Send period WebSocket pings/pongs every 6th second and if it's not ACK:ed in 30 seconds
        // then disconnect.
        //
        // This is just a way to ensure that the connection isn't idle if no message is sent that often
        .enable_ws_ping(
            PingConfig::new()
                .ping_interval(Duration::from_secs(6))
                .inactive_limit(Duration::from_secs(30)),
        )
        // There are other configurations as well that can be found here:
        // <https://docs.rs/reconnecting-jsonrpsee-ws-client/latest/reconnecting_jsonrpsee_ws_client/struct.ClientBuilder.html>
        .build("ws://127.0.0.1:9944".to_string())
        .await?;

    // let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;
    // let rpc_legacy = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc.clone()).await?;
    // let api: OnlineClient<PolkadotConfig> =
    //     OnlineClient::from_rpc_client(RpcClient::new(rpc.clone())).await?;

    // Now, we can make some RPC calls using some legacy RPC methods.
    println!(
        "📛 System Name: {:?}\n🩺 Health: {:?}\n🖫 Properties: {:?}\n🔗 Chain: {:?}\n",
        rpc.system_name().await?,
        rpc.system_health().await?,
        rpc.system_properties().await?,
        rpc.system_chain().await?
    );

    let alice_signer = dev::alice();
    let alice_public_key = dev::alice().public_key();

    // let value: u32 = 123;
    // let encoded: Vec<u8> = value.encode();
    // let my_bounded_vec: BoundedVec<u8, ConstU32<4>> = encoded.try_into()?;
    // let my_bounded_vec: MyBoundedVec<T> =
    // "default".as_bytes().to_vec().try_into().unwrap();
    let my_bounded_vec: BoundedVec4<(Data, Data)> = BoundedVec4::try_from(vec![
		(
			Data::Raw0(b"special1".to_vec().try_into().unwrap()),
			Data::Raw0(b"special2".to_vec().try_into().unwrap()),
		),
		(Data::None, Data::None),
	])
	.unwrap();

    let info = IdentityInfo {
        additional: my_bounded_vec,
        // TODO: Why does this give error `expected `BoundedVec4<(Data, Data)>`, found `BoundedVec<_, _>`?
		// additional: BoundedVec::default(),
        // additional: BoundedVec4<(Data::default(), Data::default())>,
        // TODO: Why does using `Raw` instead of `Raw0` give error
        // `variant or associated item not found in `Data` help: there is a variant with a similar name: `Raw0``
		display: Data::Raw0(b"name".to_vec().try_into().unwrap()),
		legal: Data::default(),
		web: Data::Raw0(b"website".to_vec().try_into().unwrap()),
		riot: Data::default(),
		email: Data::Raw0(b"abcdefghijklmnopqrstuvwxyz@me.com".to_vec().try_into().unwrap()),
		pgp_fingerprint: None,
		image: Data::default(),
		twitter: Data::default(),
	};
    let tx_payload = runtime::tx().identity().set_identity(info);
    println!("tx_payload {:?}", tx_payload);

    // let tx_config = DefaultExtrinsicParamsBuilder::<PolkadotConfig>::new()
    //     .build();
    // // provide the extrinsic params when submitting a transaction
    // let _ = api
    //     .api::transaction)
    //     .sign_and_submit_then_watch(&tx_payload, &dev::alice(), tx_config)
    //     .await;

    Ok(())
}
