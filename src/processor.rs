use crate::{
    instruction::CharacterInstruction,
    state::{CharacterAttributes, CreateMyCharacter},
};

// use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::LifeOrigin;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

pub struct Processor {}


impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = CharacterInstruction::unpack(instruction_data)?;

        match instruction {
            CharacterInstruction::CreateCharacter {
                life_origin, 
                char_attrib
            } => {
                Self::create_my_character(
                    program_id, 
                    accounts, 
                    life_origin, 
                    char_attrib
                )
            }
        }
    }


    pub fn create_my_character(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        mylifeval: LifeOrigin, 
        attrib: CharacterAttributes
    ) -> ProgramResult {
            msg!("Creation my character begins");
            let account_info_iter = &mut accounts.iter();
            let character_owner = next_account_info(account_info_iter)?;

            let myCharacter = CreateMyCharacter {
                myLife: mylifeval,
                charAttrib: attrib,
            }; 

            msg!(&*format!("Pool initialized: {:?}", myCharacter));

        Ok(())
    }
}