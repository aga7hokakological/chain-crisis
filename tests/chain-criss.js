const anchor = require('@project-serum/anchor');
const { assert } = require('chai');

describe('chain-criss', () => {

  // Configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Chaincriss;

  const character_owner = anchor.web3.Keypair.generate();

  // let lifeOrigin = "Corporate Espionage";

  it("Initialize a character", async() => {

    console.log("🚀 Starting test...");

    const tx = await program.rpc.initializeCharacter({
      accounts: {
        character: character_owner.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.systemProgram.programId,
      },
      signers: [character_owner],
    });
    console.log("Transaction is: ", tx);
  });


  it("Create a Character", async() => {
    await program.rpc.createCharacter(
      LifeOrigin.CorporateEspionage,
      new anchor.BN(45), 
      new anchor.BN(67), 
      new anchor.BN(25), {
        accounts: {
          character: program.provider.publicKey,
          user: provider.wallet.publicKey,
        }, 
      });

      let character_details = await program.account.character_owner.fetch(character.publicKey);
      console.log(character_details);
    })

  const LifeOrigin = {
      CorporateEspionage: { corporateEspionage: {} },
      SlumsSurvivor: { slumsSurvivor: {} },
      Drifter: { drifter: {} },
    } 



     
      // it('Is initialized!', async () => {
      //   // Add your test here.
      //   const program = anchor.workspace.ChainCriss;
      //   const tx = await program.rpc.initialize();
      //   console.log("Your transaction signature", tx);
      // });
});
