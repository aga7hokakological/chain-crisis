use thiserror::Error;
use num_derive::FromPrimitive;
use solana_program::program_error::ProgramError;
use solana_program::decode_error::DecodeError;

// #[error]
pub enum ErrorCode {
    // #[msg("Unable to create character")]
    CharacterCreationError,
}

impl From<ErrorCode> for ProgramError {
    fn from(e: ErrorCode) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for ErrorCode {
    fn type_of() -> &'static str {
        "Character Creation Error"
    }
}