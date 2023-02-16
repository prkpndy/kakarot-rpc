pub const KAKAROT_MAIN_CONTRACT_ADDRESS: &str =
    "0x5f22aa755d050e7e77e9cd0d096f12784d9d55a6624489a9bc636ff4ac6f2fe";

pub const CHAIN_ID: u64 = 1263227476;

pub const STARKNET_NATIVE_TOKEN: &str =
    "2087021424722619777119509474943472645767659996348769578120564519014510906823";

pub mod selectors {
    use starknet::core::types::FieldElement;
    use starknet::macros::selector;

    pub const GET_STARKNET_CONTRACT_ADDRESS: FieldElement =
        selector!("get_starknet_contract_address");
    pub const BYTECODE: FieldElement = selector!("bytecode");

    pub const EXECUTE_AT_ADDRESS: FieldElement = selector!("execute_at_address");
    pub const COMPUTE_STARKNET_ADDRESS: FieldElement = selector!("compute_starknet_address");
    pub const CHAIN_ID: u64 = 1263227476_u64;

    pub const GET_EVM_ADDRESS: FieldElement = selector!("get_evm_address");

    pub const BALANCE_OF: FieldElement = selector!("balanceOf");
}