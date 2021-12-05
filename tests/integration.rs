#![cfg(feature = "test-bpf")]

use {
    assert_matches::*,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_sdk::{signature::Signer, transaction::Transaction},
    solana_validator::test_validator::*,
};
use solana_program_test::{processor, ProgramTest};
use chain_crisis::{entrypoint::process_instruction};
use serde::{Deserialize, Serialize};


#[repr(packed)]
#[derive(Serialize, Deserialize)]
struct Attrib {
    damage: u64,
    resistance: u64,
}

#[repr(u8)]
enum Life {
    CorporateEspionage,
    SlumsSurvivor,
    Drifter,
}

#[repr(packed)]
#[derive(Serialize, Deserialize)]
struct Character {
    life: Life,
    attrib: Attrib,
}


#[tokio::test]
async fn test_chain_crisis_character_creation_demo() {
    let program_id = Pubkey::from_str(&"chaincrisis111111111111111111111111111111111").unwrap();

    let character_owner = Keypair::new();

    let mut program_test = 
        ProgramTest::new("chain_crisis", program_id, processor!(process_instruction));

    let c = Character {
        life: Attrib::CorporateEspionage,
        attrib: Attrib {
            damage: 62,
            resistance: 34,
        },
    };

    println!("life: {:?}", {c.life});
    println!("attrib: {:?}", {c.attrib});

    program_test.add_account(
        character_owner.pubkey(), 
        Account {
            lamports: sol_to_lamports(1000.0),
            ..Account::default()
        },
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bytes(
            program_id, 
            unsafe { any_as_u8_slice(&c) }, 
            vec![
                AccountMeta::new(character_owner.pubkey(), true),
                AccountMeta::new(character_owner.pubkey(), false),
                AccountMeta::new(system_program::ID, false),
            ], 
        )],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer, &alice, &pda], recent_blockhash);

    match banks_client.process_instruction(transaction).await {
        Ok(()) => (),
        Err(e) => panic!("{}", e),
    }
}


// mod tests {
//     use super::*;

//     #[test]
//     fn test_unpack_create_character() {
//         let my_life = LifeOrigin::CorporateEspionage;
//         let dmg = 19;
//         let res = 12;
    
//         let my_life_attrib = CharacterAttributes { damage: dmg, resistance: res };
//         let encode_my_life_attrib: &[u8] = bincode::serialize(&my_life_attrib).unwrap();

//         let i = CharacterAttributes::unpack_from_slice(encode_my_life_attrib);
//     }
// }

// #[test]
// fn test_validator_transaction() {
//     let program_id = Pubkey::new_unique();

//     let (test_validator, payer) = TestValidatorGenesis::default()
//         .add_program("bpf_program_template", program_id)
//         .start();
//     let (rpc_client, recent_blockhash, _fee_calculator) = test_validator.rpc_client();

//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction {
//             program_id,
//             accounts: vec![AccountMeta::new(payer.pubkey(), false)],
//             data: vec![1, 2, 3],
//         }],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[&payer], recent_blockhash);

//     assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
// }
