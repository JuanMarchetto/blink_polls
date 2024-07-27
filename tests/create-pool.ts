import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PollProgram } from "../target/types/poll_program";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";

const pollName = "Best Chain";
const startTime = new anchor.BN(Date.now());
const endTime = new anchor.BN(Date.now() + 1_000_000);
const options = 3;
const description = "Which is the best chain?";
const optionsDescriptions = ["Solana", "Ethereum", "Bitcoin"];

describe("create poll", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PollProgram as Program<PollProgram>;
  const signer = (program.provider as anchor.AnchorProvider).wallet;

  it("Create Chain", async () => {
    // Add your test here.
    const tx = await program.methods
      .createPoll(pollName, options ,startTime, endTime, description)
      .accounts({
        signer: signer.publicKey,

      })
      .rpc({ skipPreflight: true })
    console.log("Your transaction signature", tx);
    const polls = await program.account.poll.all();
    console.log(polls);
  });
});
