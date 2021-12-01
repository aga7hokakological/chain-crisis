// use std::str::FromStr;

use crate::{
    error::ErrorCode,
    instruction::CharacterInstruction,
    state::{CharacterAttributes, CreateMyCharacter},
};


use crate::state::LifeOrigin;


// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    // clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    borsh,
};

pub struct Processor;


impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // let instruction = CharacterInstruction::unpack(instruction_data)?;


        Ok(())
    }
    // Ok(())
}