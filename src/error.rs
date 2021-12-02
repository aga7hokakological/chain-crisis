use solana_program::program_error::ProgramError;
use solana_program::decode_error::DecodeError;

pub enum ErrorCode {
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