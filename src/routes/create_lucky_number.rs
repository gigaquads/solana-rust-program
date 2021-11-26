use crate::accounts::{save, LuckyNumberAccount};
use crate::auth::verify_program_is_account_owner;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(Debug, BorshSchema, BorshDeserialize, BorshSerialize)]
pub struct CreateLuckyNumber {
    pub value: u8,
}

/// Custom deserialization logic.
pub fn on_request(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let payload = CreateLuckyNumber::try_from_slice(input)?;
    process_request(program_id, account_infos, &payload)
}

/// Process instruction with deserialized data.
pub fn process_request(
    program_id: &Pubkey,
    account_infos: &[AccountInfo],
    data: &CreateLuckyNumber,
) -> ProgramResult {
    let accounts_info_iter = &mut account_infos.iter();
    let account_info = next_account_info(accounts_info_iter)?;

    // account must be owned by program to modify data
    verify_program_is_account_owner(account_info, program_id)?;

    msg!("setting lucky number to {}", data.value);

    let account = LuckyNumberAccount { value: data.value };

    save::<LuckyNumberAccount>(account_info, &account)?;

    Ok(())
}
