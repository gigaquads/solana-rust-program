use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey,
};

mod create_lucky_number;
mod update_lucky_number;

/// Add route handlers to this array to register with app...
pub const ROUTES_ENABLED: &'static [fn(
    &Pubkey,
    &[AccountInfo],
    &[u8],
) -> ProgramResult] = &[
    create_lucky_number::on_request,
    update_lucky_number::on_request,
];
