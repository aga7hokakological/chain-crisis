// use borsh::BorshDeserialize;
use solana_program::borsh;
use solana_program::program_error::ProgramError;

use crate::processor::LifeOrigin;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum CharacterInstruction {
    CreateCharacter {
        lifeNum: u8,
        charAttrib: LifeOrigin,
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
                    lifeNum: u8::try_from_slice(&data[1])?,
                    charAttr: LifeOrigin::try_from_slice(&data[2])?,
                }
            )
            }
            // 2 => Ok(StreamInstruction::WithdrawFromStream(
            //     WithdrawInput::try_from_slice(data)?,
            // )),
            // 3 => Ok(StreamInstruction::CloseStream),
            // _ => Err(ProgramError::InvalidInstructionData),
        }
    }