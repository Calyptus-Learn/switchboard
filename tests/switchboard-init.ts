import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { SwitchboardInit } from '../target/types/switchboard_init';

describe('switchboard-init', () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SwitchboardInit as Program<SwitchboardInit>;

  const keypair = anchor.web3.Keypair.generate();

  it('Is initialized!', async () => {
    try {
      console.log(`switchboard pubkey: ${keypair.publicKey.toString()}`);
      console.log(
        `user program pubkey: ${anchor.AnchorProvider.env().wallet.publicKey.toString()}`
      );
      console.log(
        `systemProgram: ${anchor.web3.SystemProgram.programId.toString()}`
      );
      // get the wallet's balance
      const balance = await anchor
        .getProvider()
        .connection.getBalance(anchor.AnchorProvider.env().wallet.publicKey);
      console.log(`balance: ${balance}\n\n`);
      // Add your test here.
      const tx = await program.methods
        .initialize()
        .accounts({
          switchboard: keypair.publicKey,
          user: anchor.AnchorProvider.env().wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([keypair])
        .rpc();

      console.log('Your transaction signature', tx);
    } catch (err) {
      console.log(err);
      throw err;
    }
  });
});
