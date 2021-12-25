use crate::*;
pub fn get_timestamp() -> U64 {
    return U64::from(env::block_timestamp());
}
