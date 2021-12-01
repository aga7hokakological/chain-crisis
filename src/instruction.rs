// use borsh::BorshDeserialize;
// use solana_program::borsh;
use solana_program::program_error::ProgramError;

use std::convert::TryInto;
use borsh::BorshDeserialize;

use crate::error::ErrorCode;

use crate::state::LifeOrigin;
use crate::state::CharacterAttributes;
use crate::state::CreateMyCharacter;

use crate::state::LifeOrigin::CorporateEspionage;
use crate::state::LifeOrigin::SlumsSurvivor;
use crate::state::LifeOrigin::Drifter;

#[derive(Debug, PartialEq)]
pub enum CharacterInstruction {
    CreateCharacter {
        lifeOrigin: LifeOrigin,
        charAttrib: CharacterAttributes,
    },
}

impl CharacterInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        // let (tag, data) = instruction_data
        //     .split_first()
        //     .ok_or(ProgramError::InvalidInstructionData)?;

        let data = Vec::<Vec<u8>>::try_from_slice(instruction_data).unwrap();

        match data[0][0] {
            1 => Ok(Self::CreateCharacter{
                lifeOrigin: LifeOrigin::try_from_slice(&data[1])?,
                charAttrib: CharacterAttributes::try_from_slice(&data[2])?,
            }),
            _ => Err(ErrorCode::CharacterCreationError.into()),
        }
    }
}