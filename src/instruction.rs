// use borsh::BorshDeserialize;
use solana_program::borsh;
use solana_program::program_error::ProgramError;

use crate::processor::LifeOrigin;
use crate::state::CharacterAttributes;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterInstruction {
    CreateCharacter {
        lifeNum: u8,
        charAttrib: CharacterAttributes,
    },
}

impl CharacterInstruction {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        // let (tag, data) = instruction_data
        //     .split_first()
        //     .ok_or(ProgramError::InvalidInstructionData)?;
        let (tag, data) = Vec::<Vec<u8>>::try_from_slice(instruction_data).unwrap();
        match tag {
            1 => Ok(Self::CreateCharacter{
                    lifeNum: LifeOrigin::try_from_slice(&data[1])?,
                    charAttr: CharacterAttributes::try_from_slice(&data[2])?,
                }
            )
            }
        }
    }