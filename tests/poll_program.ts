import * as anchor from "@coral-xyz/anchor";
import { PollProgram } from "../target/types/poll_program";

const pollName = "Best Chain";
const startTime = new anchor.BN(Math.floor(Date.now() / 1000));
const endTime = new anchor.BN(Math.floor((Date.now() / 1000) + 1_000_000));
const description = "Which is the best chain?";
const numberOfOptions = 1;
const optionDescription = "Solana";

describe("poll_program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PollProgram as anchor.Program<PollProgram>;

  const [pollPDA] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("poll"), Buffer.from(pollName)],
    program.programId,
  )

  const [optionPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("option"), pollPDA.toBuffer(), Buffer.from([numberOfOptions])],
    program.programId,
  )

  it("A Pool is created!", async () => {
    const tx = await program.methods
      .createPoll(pollName, numberOfOptions, startTime, endTime, description)
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });

  it("An Option is created!", async () => {
    const tx = await program.methods
      .addOption(numberOfOptions, optionDescription)
      .accountsPartial({ poll: pollPDA, optionPda: optionPublicKey })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });

  it("A vote is cast!", async () => {
    const tx = await program.methods
      .castVote(numberOfOptions)
      .accountsPartial({ poll: pollPDA, optionPda: optionPublicKey })
      .rpc({ skipPreflight: true });
    console.log("Your transaction signature", tx);
  });
});
