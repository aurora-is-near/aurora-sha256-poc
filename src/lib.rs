use near_sdk::near;

#[near(contract_state)]
#[derive(Default)]
pub struct Contract;

#[near]
impl Contract {
    pub fn get_sha256(&self) {
        run_hash(b"Test", "Test");
        run_hash(&[0], "&[0]");
        run_hash(b"", "''");
        run_hash(&[], "&[]");
    }
}

fn run_hash(value: &[u8], hint: &str) {
    use sha2::Digest;

    let res_near = near_sdk::env::sha256(value);
    let res_aurora = aurora_engine_sdk::sha256(value);
    let value_hash = sha2::Sha256::digest(value);
    near_sdk::require!(
        (res_near.as_slice() == res_aurora.as_bytes())
            && (res_near.as_slice() == value_hash.as_slice())
    );
    near_sdk::log!(
        "[Passed] sha256 for {hint:?}: {}",
        hex::encode(res_aurora.0)
    );
}
