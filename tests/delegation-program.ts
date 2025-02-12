import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DelegationProgram } from "../target/types/delegation_program";
import { PublicKey } from "@solana/web3.js";
import { Buffer } from "buffer";

describe("delegation-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DelegationProgram as Program<DelegationProgram>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initializeDelegate().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Finds the delegate account", async () => {
    const userWallet = anchor.AnchorProvider.local().wallet.publicKey;
    const programId = program.programId;

    const [delegateAccount] = PublicKey.findProgramAddressSync(
        [
            Buffer.from("delegate"),
            userWallet.toBuffer()
        ],
        programId
    );

    console.log("Delegate account:", delegateAccount.toString());
  });
});
