use std::str::FromStr;

use crate::{
    error::ErrorCode,
    instruction::CharacterInstruction,
    state::{CharacterAttributes},
};
// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    borsh,
};

pub struct Processor;


pub enum LifeOrigin {
    CorporateEspionage = "Corporate Espionage" as *const _ as isize,
    SlumsSurvivor = "Slums Survivor" as *const _ as isize,
    Drifter = "Drifter" as *const _ as isize,
}

impl LifeOrigin {
    pub fn select_life_origin_of_character(lifeOriginNum: u8) {
        let x = match lifeOriginNum {
            1 => LifeOrigin::CorporateEspionage,
            2 => LifeOrigin::SlumsSurvivor,
            _ => LifeOrigin::Drifter,
        };
    }
}

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8],) -> ProgramResult {
        let instruction = CharacterInstruction::unpack(instruction_data)?;

        match instruction {
            CharacterInstruction::CreateCharacter{lifeNum, charAttrib} => {
                Self::process_create_character(program_id, accounts, lifeNum, charAttrib)
            }
        }
    }

    fn process_create_character(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: CharacterAttributes,
        life: LifeOrigin
    )  -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
    }
}