use std::str::FromStr;

use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
    borsh::try_from_slice_unchecked,
    // pubkey::Pubkey,
};

use std::convert::TryInto;
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CreateMyCharacter {
    pub myLife: LifeOrigin,
    pub charAttrib: CharacterAttributes,
}

// #[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
// impl CreateMyCharacter {
//     pub fn set_values(myLifeorigin: &str, charAttrib: CharacterAttributes) {
//         let mylife = LifeOrigin::from_str(myLifeorigin).unwrap();
//         let charatt = charAttrib;
//     }
// }


#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub enum LifeOrigin {
    CorporateEspionage,
    SlumsSurvivor,
    Drifter,
}


impl FromStr for LifeOrigin {
    type Err = ();

    fn from_str(input: &str) -> Result<LifeOrigin, Self::Err>{
        match input {
            "Corporate Espionage" => Ok(LifeOrigin::CorporateEspionage),
            "Slums Survivor" => Ok(LifeOrigin::SlumsSurvivor),
            "Drifter" => Ok(LifeOrigin::Drifter),
            _ => Err(()),
        }
    }
}

// #[derive(Clone, Debug, PartialEq)]
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CharacterAttributes {
    pub damage: u64,
    pub resistance: u64,
}

impl Sealed for CharacterAttributes {}

impl Pack for CharacterAttributes {
    const LEN: usize = 128;

    fn pack_into_slice(&self, target: &mut [u8]) {
        let dmg = self.damage.to_le_bytes();
        let res = self.resistance.to_le_bytes();
        for i in 0..64 {
            target[i] = dmg[i];
        }

        for i in 65..128 {
            target[i] = res[i - 64];
        }
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let damage = u64::from_le_bytes(src[0..64].try_into().unwrap());
        let resistance = u64::from_le_bytes(src[64..128].try_into().unwrap());

        Ok(Self {
            damage,
            resistance,
        })
    }
}

// impl Default for CharacterAttributes {
//     fn default() -> CharacterAttributes {
//         CharacterAttributes {
//             damage: 100,
//             resistance: 30,
//         }
//     }
// }



// impl Sealed for LifeOrigin {}

// impl Pack for LifeOrigin {
//     const LEN: usize = 128;

//     fn pack_into_slice(&self, dst: &mut [u8]) {

//     }

//     fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {

//         Ok(Self {
//             LifeOrigin::src,
//         }
//         )
//     }
// }