use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey
};

use crate::processor::Processor;



entrypoint!(process_instruction);
fn process_instruction<CharacterCreationError> (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(
        program_id, 
        accounts,
        instruction_data,
    ) {
        error.print::<CharacterCreationError>();
        return Err(error);
    } else {
        Ok(())
    }
}
