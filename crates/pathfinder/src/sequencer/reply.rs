//! Structures used for deserializing replies from Starkware's sequencer REST API.
use crate::serde::{H256AsRelaxedHexStr, U256AsBigDecimal};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DefaultOnError};
use web3::types::{H256, U256};

/// Used to deserialize replies to [Client::block_by_hash](crate::sequencer::Client::block_by_hash) and
/// [Client::block_by_number](crate::sequencer::Client::block_by_number).
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Block {
    #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
    #[serde(default)]
    pub block_hash: Option<H256>,
    #[serde(default)]
    pub block_number: Option<u64>,
    #[serde_as(as = "H256AsRelaxedHexStr")]
    pub parent_block_hash: H256,
    #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
    #[serde(default)]
    pub state_root: Option<H256>,
    pub status: Status,
    pub timestamp: u64,
    pub transaction_receipts: Vec<transaction::Receipt>,
    pub transactions: Vec<transaction::Transaction>,
}

/// Block and transaction status values.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum Status {
    #[serde(rename = "NOT_RECEIVED")]
    NotReceived,
    #[serde(rename = "RECEIVED")]
    Received,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "ACCEPTED_ON_L1")]
    AcceptedOnL1,
    #[serde(rename = "ACCEPTED_ON_L2")]
    AcceptedOnL2,
    #[serde(rename = "REVERTED")]
    Reverted,
    #[serde(rename = "ABORTED")]
    Aborted,
}

/// Used to deserialize a reply from [Client::call](crate::sequencer::Client::call).
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Call {
    #[serde_as(as = "Vec<H256AsRelaxedHexStr>")]
    pub result: Vec<H256>,
}

/// Types used when deserializing L2 call related data.
pub mod call {
    use serde::{Deserialize, Serialize};
    use serde_with::serde_as;
    use std::collections::HashMap;

    /// Describes problems encountered during some of call failures .
    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Problems {
        #[serde_as(as = "HashMap<_, _>")]
        pub calldata: HashMap<u64, Vec<String>>,
    }
}

/// Used to deserialize a reply from [Client::code](crate::sequencer::Client::code).
#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Code {
    // Unknown block hash results in empty abi represented as a JSON
    // object, instead of a JSON array
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub abi: Vec<code::Abi>,
    #[serde_as(as = "Vec<H256AsRelaxedHexStr>")]
    pub bytecode: Vec<H256>,
}

/// Types used when deserializing L2 contract related data.
pub mod code {
    use serde::{Deserialize, Serialize};
    use serde_with::skip_serializing_none;

    /// Represents deserialized L2 contract Application Blockchain Interface element.
    #[skip_serializing_none]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Abi {
        #[serde(default)]
        pub inputs: Option<Vec<abi::Input>>,
        #[serde(default)]
        pub members: Option<Vec<abi::Member>>,
        pub name: String,
        #[serde(default)]
        pub outputs: Option<Vec<abi::Output>>,
        pub r#type: String,
        pub size: Option<u64>,
        #[serde(rename = "stateMutability")]
        #[serde(default)]
        pub state_mutability: Option<String>,
    }

    /// Types used when deserializing L2 contract ABI related data.
    pub mod abi {
        use serde::{Deserialize, Serialize};

        /// Represents deserialized L2 contract ABI input element.
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub struct Input {
            pub name: String,
            pub r#type: String,
        }

        /// Represents deserialized L2 contract ABI member element.
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub struct Member {
            pub name: String,
            pub offset: u64,
            pub r#type: String,
        }

        /// Represents deserialized L2 contract ABI output element.
        #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub struct Output {
            pub name: String,
            pub r#type: String,
        }
    }
}

/// Used to deserialize replies to [Client::transaction](crate::sequencer::Client::transaction).
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
    #[serde(default)]
    pub block_hash: Option<H256>,
    #[serde_as(as = "Option<U256AsBigDecimal>")]
    #[serde(default)]
    pub block_number: Option<U256>,
    pub status: Status,
    #[serde(default)]
    pub transaction: Option<transaction::Transaction>,
    #[serde(default)]
    pub transaction_index: Option<u64>,
}

/// Used to deserialize replies to [Client::transaction_status](crate::sequencer::Client::transaction_status).
#[serde_as]
#[skip_serializing_none]
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TransactionStatus {
    #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
    #[serde(default)]
    pub block_hash: Option<H256>,
    pub tx_status: Status,
}

/// Types used when deserializing L2 transaction related data.
pub mod transaction {
    use crate::serde::{
        H160AsRelaxedHexStr, H256AsRelaxedHexStr, U256AsBigDecimal, U256AsDecimalStr,
    };
    use serde::{Deserialize, Serialize};
    use serde_with::{serde_as, skip_serializing_none};
    use web3::types::{H160, H256, U256};

    /// Represents deserialized L2 transaction entry point values.
    #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub enum EntryPointType {
        #[serde(rename = "EXTERNAL")]
        External,
        #[serde(rename = "L1_HANDLER")]
        L1Handler,
    }

    /// Represents execution resources for L2 transaction.
    #[skip_serializing_none]
    #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct ExecutionResources {
        builtin_instance_counter: execution_resources::BuiltinInstanceCounter,
        n_steps: u64,
        n_memory_holes: u64,
    }

    /// Types used when deserializing L2 execution resources related data.
    pub mod execution_resources {
        use serde::{Deserialize, Serialize};

        /// Sometimes `builtin_instance_counter` JSON object is returned empty.
        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(untagged)]
        #[serde(deny_unknown_fields)]
        pub enum BuiltinInstanceCounter {
            Normal(NormalBuiltinInstanceCounter),
            Empty(EmptyBuiltinInstanceCounter),
        }

        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
        #[serde(deny_unknown_fields)]
        pub struct NormalBuiltinInstanceCounter {
            bitwise_builtin: u64,
            ecdsa_builtin: u64,
            ec_op_builtin: u64,
            output_builtin: u64,
            pedersen_builtin: u64,
            range_check_builtin: u64,
        }

        #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
        pub struct EmptyBuiltinInstanceCounter {}
    }

    /// Represents deserialized L1 to L2 message.
    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct L1ToL2Message {
        #[serde_as(as = "H160AsRelaxedHexStr")]
        pub from_address: H160,
        #[serde_as(as = "Vec<U256AsDecimalStr>")]
        pub payload: Vec<U256>,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub selector: H256,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub to_address: H256,
        #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
        #[serde(default)]
        pub nonce: Option<H256>,
    }

    /// Represents deserialized L2 to L1 message.
    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct L2ToL1Message {
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub from_address: H256,
        #[serde_as(as = "Vec<U256AsDecimalStr>")]
        pub payload: Vec<U256>,
        #[serde_as(as = "H160AsRelaxedHexStr")]
        pub to_address: H160,
    }

    /// Represents deserialized L2 transaction receipt data.
    #[serde_as]
    #[skip_serializing_none]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Receipt {
        pub events: Vec<Event>,
        pub execution_resources: ExecutionResources,
        pub l1_to_l2_consumed_message: Option<L1ToL2Message>,
        pub l2_to_l1_messages: Vec<L2ToL1Message>,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub transaction_hash: H256,
        pub transaction_index: u64,
    }

    /// Represents deserialized L2 transaction event data.
    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Event {
        #[serde_as(as = "Vec<U256AsDecimalStr>")]
        data: Vec<U256>,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        from_address: H256,
        #[serde_as(as = "Vec<U256AsDecimalStr>")]
        keys: Vec<U256>,
    }

    /// Represents deserialized object containing L2 contract address and transaction type.
    #[serde_as]
    #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Source {
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub contract_address: H256,
        pub r#type: Type,
    }

    /// Represents deserialized L2 transaction data.
    #[serde_as]
    #[skip_serializing_none]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Transaction {
        #[serde_as(as = "Option<Vec<U256AsDecimalStr>>")]
        #[serde(default)]
        pub calldata: Option<Vec<U256>>,
        #[serde_as(as = "Option<Vec<U256AsDecimalStr>>")]
        #[serde(default)]
        pub constructor_calldata: Option<Vec<U256>>,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub contract_address: H256,
        #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
        #[serde(default)]
        pub contract_address_salt: Option<H256>,
        #[serde(default)]
        pub entry_point_type: Option<EntryPointType>,
        #[serde_as(as = "Option<H256AsRelaxedHexStr>")]
        #[serde(default)]
        pub entry_point_selector: Option<H256>,
        #[serde_as(as = "Option<Vec<U256AsDecimalStr>>")]
        #[serde(default)]
        pub signature: Option<Vec<U256>>,
        #[serde_as(as = "H256AsRelaxedHexStr")]
        pub transaction_hash: H256,
        pub r#type: Type,
    }

    /// Describes L2 transaction types.
    #[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub enum Type {
        #[serde(rename = "DEPLOY")]
        Deploy,
        #[serde(rename = "INVOKE_FUNCTION")]
        InvokeFunction,
    }

    /// Describes L2 transaction failure details.
    #[serde_as]
    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    #[serde(deny_unknown_fields)]
    pub struct Failure {
        pub code: String,
        pub error_message: String,
        #[serde_as(as = "U256AsBigDecimal")]
        pub tx_id: U256,
    }
}