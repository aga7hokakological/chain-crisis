use crate::{
    instruction::CharacterInstruction,
    state::{CharacterAttributes},
};


use crate::state::LifeOrigin;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

pub struct Processor;

create_my_character(program_id: &Pubkey, accounts: &[AccountInfo], mylifeval: LifeOrigin, attrib: CharacterAttributes) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    CreateMyCharacter::set_values(mylifeval);

    Ok(())
}


impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CharacterInstruction::unpack(instruction_data)?;

        match instruction {
            CharacterInstruction::CreateCharacter {lifeOrigin, charAttrib} => {
                Self::create_my_character(program_id, accounts, life, attrib)
            }
        }

        Ok(())
    }
}