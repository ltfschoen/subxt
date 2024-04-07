#![allow(missing_docs)]
use subxt::backend::{legacy::LegacyRpcMethods, rpc::RpcClient};
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First, create a raw RPC client:
    let rpc_client = RpcClient::from_url("ws://127.0.0.1:9944").await?;

    // Use this to construct our RPC methods:
    let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

    // We can use the same client to drive our full Subxt interface too:
    let api = OnlineClient::<PolkadotConfig>::from_rpc_client(rpc_client.clone()).await?;

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


    // // Build extrinsic params using an asset at this location as a tip:
    // let location: MultiLocation = MultiLocation {
    //     parents: 3,
    //     interior: Junctions::Here,
    // };
    // let tx_config = DefaultExtrinsicParamsBuilder::<AssetHubConfig>::new()
    //     .tip_of(1234, location)
    //     .build();

    // // And provide the extrinsic params including the tip when submitting a transaction:
    // let _ = client
    //     .tx()
    //     .sign_and_submit_then_watch(&tx_payload, &dev::alice(), tx_config)
    //     .await;



        // let current_nonce = rpc
        //     .system_account_next_index(&alice.public_key().into())
        //     .await?;
        // let current_header = rpc.chain_get_header(None).await?.unwrap();

        // let ext_params = Params::new()
        //     .mortal(&current_header, 8)
        //     .nonce(current_nonce)
        //     .build();

        // let balance_transfer = polkadot::tx()
        //     .balances()
        //     .transfer_allow_death(bob.public_key().into(), 1_000_000);

        // let ext_hash = api
        //     .tx()
        //     .create_signed_offline(&balance_transfer, &alice, ext_params)?
        //     .submit()
        //     .await?;

        // println!("Submitted ext {ext_hash} with nonce {current_nonce}");
    Ok(())
}
