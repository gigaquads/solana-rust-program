use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use std::io::Result;

/// deserialize account info into account data structure.
pub fn load<T: BorshDeserialize>(account_info: &AccountInfo) -> Result<T> {
    T::try_from_slice(&account_info.data.borrow())
}

/// serialize and write account data back to account info.
pub fn save<T: BorshSerialize>(
    account_info: &AccountInfo,
    payload: &T,
) -> Result<()> {
    payload.serialize(&mut &mut account_info.data.borrow_mut()[..])
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LuckyNumberAccount {
    pub value: u8,
}
