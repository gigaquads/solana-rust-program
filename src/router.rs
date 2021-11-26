use crate::routes::ROUTES_ENABLED;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program_error::ProgramError, pubkey::Pubkey,
};
use std::cmp::max;

pub fn route(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    // The first byte of `input` is the index of the target route in the
    // `ROUTES_ENABLED` array. The remaining byte array contains a
    // Borsh-serialized struct.
    match input.split_first() {
        None => {
            msg!("reveived invalid instruction data format");
            Err(ProgramError::InvalidInstructionData)
        }
        Some((&first, rest)) => {
            let idx: usize = max(0, first) as usize;
            if idx >= ROUTES_ENABLED.len() {
                // route index out of bounds.
                msg!("received unrecognized route index: {}", idx);
                Err(ProgramError::InvalidInstructionData)
            } else {
                // delegate processing to indexed `on_request` function.
                let request_handler = ROUTES_ENABLED[idx];
                request_handler(program_id, accounts, rest)
            }
        }
    }
}
