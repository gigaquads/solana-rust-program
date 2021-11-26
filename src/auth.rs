use solana_program::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey,
};

/// Return a ProgramError if the given program ID does not "own" the given
/// account_info.
pub fn verify_program_is_account_owner(
    account_info: &AccountInfo,
    program_id: &Pubkey,
) -> Result<(), ProgramError> {
    if account_info.owner != program_id {
        Err(ProgramError::IncorrectProgramId)
    } else {
        Ok(())
    }
}
