import "mocha";

import * as anchor from "@coral-xyz/anchor";
import { AnchorProvider } from "@coral-xyz/anchor";
import * as sbv2 from "@switchboard-xyz/solana.js";
import { SwitchboardInit } from "../target/types/switchboard_init";
import { assert } from "chai";
import { BN } from "bn.js";

describe("vrf-client", () => {
  // Configure the client to use the local cluster.
  const provider = AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SwitchboardInit as anchor.Program<SwitchboardInit>;

  const payer = (provider.wallet as sbv2.AnchorWallet).payer;

  it("init_client", async () => {
    // Add your test here.
    try {
        const tx = await program.methods.initClient({}).rpc();
        console.log("init_client transaction signature", tx);
    } catch (err) {
        console.log(err);
        throw err;
    }
  });
});
