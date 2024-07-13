use near_sdk::{env, log, near, require};

#[near(contract_state)]
#[derive(Default)]
pub struct Contract;

#[near]
impl Contract {
    pub fn get_sha256(&self) {
        use sha2::Digest;

        let data1 = b"Test";
        let res_near = env::sha256(data1);
        let res_aurora = aurora_engine_sdk::sha256(data1);
        let value_hash = sha2::Sha256::digest(&data1);
        require!(
            (res_near.as_slice() == res_aurora.as_bytes())
                && (res_near.as_slice() == value_hash.as_slice())
        );
        log!("sha256 for 'test': {}", hex::encode(res_aurora.0));

        let data2 = &[0];
        let res_near = env::sha256(data2);
        let res_aurora = aurora_engine_sdk::sha256(data2);
        let value_hash = sha2::Sha256::digest(&data2);
        require!(
            (res_near.as_slice() == res_aurora.as_bytes())
                && (res_near.as_slice() == value_hash.as_slice())
        );
        log!("sha256 for &[0]: {}", hex::encode(res_aurora.0));

        let data3 = b"";
        let res_near = env::sha256(data3);
        let res_aurora = aurora_engine_sdk::sha256(data3);
        let value_hash = sha2::Sha256::digest(&data3);
        require!(
            (res_near.as_slice() == res_aurora.as_bytes())
                && (res_near.as_slice() == value_hash.as_slice())
        );
        log!("sha256 for '': {}", hex::encode(res_aurora.0));

        let data4 = &[];
        let res_near = env::sha256(data4);
        let res_aurora = aurora_engine_sdk::sha256(data4);
        let value_hash = sha2::Sha256::digest(&data4);
        require!(
            (res_near.as_slice() == res_aurora.as_bytes())
                && (res_near.as_slice() == value_hash.as_slice())
        );
        log!("sha256 for &[]: {}", hex::encode(res_aurora.0));
    }
}
