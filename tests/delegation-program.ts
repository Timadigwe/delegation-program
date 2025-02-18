import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DelegationProgram } from "../target/types/delegation_program";
import { PublicKey } from "@solana/web3.js";
import { Buffer } from "buffer";

describe("delegation-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DelegationProgram as Program<DelegationProgram>;

  it("Is initialized with deposit!", async () => {
    // Initial deposit amount (1 SOL)
    const amount = new anchor.BN(1 * anchor.web3.LAMPORTS_PER_SOL);
    
    const tx = await program.methods
      .initializeDelegate(amount)
      .rpc();
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
