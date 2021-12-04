use std::str::FromStr;
use std::str;

use solana_program::{
    program_error::ProgramError,
    program_pack::Pack,
};

// use borsh::BorshDeserialize;
use crate::error::ErrorCode::CharacterCreationError;

use crate::state::LifeOrigin;
use crate::state::CharacterAttributes;

#[derive(Debug, PartialEq)]
pub enum CharacterInstruction {
    CreateCharacter {
        life_origin: LifeOrigin,
        char_attrib: CharacterAttributes,
    },
}

impl CharacterInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (tag, data) = instruction_data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            // 0 => {
            //     let (life, data) = Self::try_from_slice(data).unwrap();
            //     let (attrib, data) = Self::try_from_slice(data).unwrap();
            //     Ok(Self::CreateCharacter {
            //         lifeOrigin: life,
            //         charAttrib: attrib,
            //     })
            // }
            0 => Ok(Self::CreateCharacter {
                life_origin: LifeOrigin::from_str(str::from_utf8(&data).unwrap()).unwrap(),
                char_attrib: CharacterAttributes::unpack_from_slice(&data).unwrap(),
            }) ,
            _ => Err(CharacterCreationError.into())
        }
    }
}