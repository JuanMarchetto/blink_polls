import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PollProgram } from "../target/types/poll_program";

const pollName = "Best Chain";
// current timestamp
const startTime = new anchor.BN(Math.floor(Date.now() / 1000));
const endTime = new anchor.BN(Math.floor((Date.now()  / 1000) + 1_000_000));
const options = 3;
const description = "Which is the best chain?";
const optionsDescriptions = ["Solana", "Ethereum", "Bitcoin"];

describe("poll_program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PollProgram as Program<PollProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createPoll(pollName, options, startTime, endTime, description).rpc({skipPreflight: true});
    console.log("Your transaction signature", tx);
  });
});
