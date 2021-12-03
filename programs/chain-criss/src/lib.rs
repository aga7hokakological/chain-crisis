use anchor_lang::prelude::*;
// use anchor_lang::{AccountDeserialize, AccountSerialize};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod chain_criss {
    use super::*;
    pub fn initialize_character(ctx: Context<InitializeCharacter>) -> ProgramResult {
        let character_life = &mut ctx.accounts.character;
        character_life.character_attrib = Default::default();
        Ok(())
    }

    pub fn create_character(ctx: Context<InitializeCharacter>, life: LifeOrigin,
        //  _character: CharacterAttributes
        dmg: u64, res: u64, throw_dist: u64
        ) -> ProgramResult {
        let character_life = &mut ctx.accounts.character;
        // let character_attrib: CharacterAttributes = &mut ctx.accounts.character;

        match life {
            LifeOrigin::CorporateEspionage => "Corporate Espionage".to_string(),
            LifeOrigin::SlumsSurvivor => "Slums Survivor".to_string(),
            LifeOrigin::Drifter => "Drifter".to_string(),
        };
        
        character_life.character_origin = life;
        character_life.character_attrib = CharacterAttributes {
            damage: dmg,
            resistance: res,
            throw_distance: throw_dist
        };
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeCharacter<'info> {
    // #[account(init, payer = user)]
    pub character: Account<'info, MyCharacter>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateCharacter<'info> {
    // #[account(init, payer = user)]
    #[account(mut)]
    pub character: Account<'info, MyCharacter>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
// #[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MyCharacter {
    // #[account(mut)]
    pub character_origin: LifeOrigin,
    pub character_attrib: CharacterAttributes,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum LifeOrigin {
    CorporateEspionage,
    SlumsSurvivor,
    Drifter,
}

// impl LifeOrigin {
//     fn getLife(lifeOrigin: LifeOrigin) {
//         match lifeOrigin {
//             LifeOrigin::CorporateEspionage(_) => "Corporate Espionage",
//             LifeOrigin::SlumsSurvivor(_) => "Slums Survivor",
//             LifeOrigin::Drifter(_) => "Drifter",
//         };
//     }
// }

#[account]
pub struct CharacterAttributes {
    damage: u64,
    resistance: u64,
    throw_distance: u64,
    // adrenaline: u64,
    // deathblow_survival: u64,
    // damage_reduction: u64,
    // elemental_damage: u64,
    // ram_efficiency: u64,
    // ram_capacity: u64,
    // system_accuracy: u64,
    // critical_damage: u64,
    // critical_hit_rate: u32,
    // stealth_damage: u32,
}

impl Default for CharacterAttributes {
    fn default() -> CharacterAttributes {
        CharacterAttributes {
            damage: 100,
            resistance: 30,
            throw_distance: 5,
            // adrenaline: 70,
            // deathblow_survival: 10,
            // damage_reduction: 30,
            // elemental_damage: 60,
            // ram_efficiency: 80,
            // ram_capacity: 1000,
            // system_accuracy: 45,
            // critical_damage: 60,
            // critical_hit_rate: 90,
            // stealth_damage: 56,
        }
    }
}
