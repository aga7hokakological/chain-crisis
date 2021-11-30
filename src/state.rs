// use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::borsh;
use solana_program::{clock::UnixTimestamp, pubkey::Pubkey};


// #[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
// pub struct CreateMyCharacter {

// }

#[derive(Clone, Debug, PartialEq)]
pub struct CharacterAttributes {
    damage: u64,
    resistance: u64,
    throw_distance: u64,
    adrenaline: u64,
    deathblow_survival: u64,
    damage_reduction: u64,
    elemental_damage: u64,
    ram_efficiency: u64,
    ram_capacity: u64,
    system_accuracy: u64,
    critical_damage: u64,
    critical_hit_rate: u32,
    stealth_damage: u32,
}

impl Default for CharacterAttributes {
    fn default() -> CharacterAttributes {
        CharacterAttributes {
            damage: 100,
            resistance: 30,
            throw_distance: 5,
            adrenaline: 70,
            deathblow_survival: 10,
            damage_reduction: 30,
            elemental_damage: 60,
            ram_efficiency: 80,
            ram_capacity: 1000,
            system_accuracy: 45,
            critical_damage: 60,
            critical_hit_rate: 90,
            stealth_damage: 56,
        }
    }
}
