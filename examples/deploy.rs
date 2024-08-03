use anybuf::Anybuf;
use contract::{
    msg::{InstantiateMsg, MigrateMsg},
    state::models::Config,
    Contract, ContractExecuteMsgFns, ContractQueryMsgFns,
};
use cosmos_sdk_proto::Any;
use cw_orch::{
    anyhow::{self, Ok},
    daemon::{networks, TxSender},
    prelude::*,
};
use dotenv;
use pretty_env_logger;

const FEE_COLLECTION_ADDR: &str = "juno1rec44j9xq8aj4w5kun796f89njzvdlezwk7cy4";

pub fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let network = networks::UNI_6;
    let chain = DaemonBuilder::new(network).build()?;

    let contract = Contract::new(chain.clone());
    let sender = chain.sender().address();

    contract.upload_if_needed()?;

    if contract.address().is_err() {
        contract.instantiate(&InstantiateMsg {}, Some(&sender), None)?;

        let _ = chain.commit_any::<Any>(
            vec![juno_feeshare_msg(
                contract.addr_str()?,
                sender.to_string(),
                FEE_COLLECTION_ADDR.to_string(),
            )],
            None,
        );
    } else {
        contract.migrate_if_needed(&MigrateMsg {})?;
    }

    // can call any necessary execution messages here like adding admin, etc.
    contract.set_config(Config {})?;

    // can also query any necessary data here from the contract
    contract.config()?;

    Ok(())
}

pub fn juno_feeshare_msg(
    contract_address: String,
    deployer_address: String,
    withdrawer_address: String,
) -> Any {
    Any {
        type_url: "/juno.feeshare.v1.MsgRegisterFeeShare".to_string(),
        value: Anybuf::new()
            .append_string(1, contract_address)
            .append_string(2, deployer_address)
            .append_string(3, withdrawer_address)
            .into_vec(),
    }
}

pub fn juno_update_feeshare_msg(
    contract_address: String,
    deployer_address: String,
    withdrawer_address: String,
) -> Any {
    Any {
        type_url: "/juno.feeshare.v1.MsgUpdateFeeShare".to_string(),
        value: Anybuf::new()
            .append_string(1, contract_address)
            .append_string(2, deployer_address)
            .append_string(3, withdrawer_address)
            .into_vec(),
    }
}

// pub fn set_archway_flat_fee_msg() {
//     let metadata_any: archway_proto::Any = MsgSetContractMetadata {
//         sender_address: daemon.sender().to_string(),
//         metadata: Some(ContractMetadata {
//             contract_address: contract.address().unwrap().to_string(),
//             owner_address: daemon.sender().to_string(),
//             rewards_address: daemon.sender().to_string(),
//         }),
//     }
//     .to_any()
//     .unwrap();

//     daemon
//         .commit_any::<prost_types::Any>(
//             vec![prost_types::Any {
//                 type_url: metadata_any.type_url,
//                 value: metadata_any.value,
//             }],
//             None,
//         )
//         .unwrap();

//     let flatfee_any: archway_proto::Any = MsgSetFlatFee {
//         sender_address: daemon.sender().to_string(),
//         contract_address: contract.address().unwrap().to_string(),
//         flat_fee_amount: Some(Coin {
//             amount: (150_000_000_000_000_000u128).to_string(),
//             denom: "aarch".to_string(),
//         }),
//     }
//     .to_any()
//     .unwrap();
// }
