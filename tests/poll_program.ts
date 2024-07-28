import * as anchor from "@coral-xyz/anchor";
import { PollProgram } from "../target/types/poll_program";

describe("poll_program", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.PollProgram as anchor.Program<PollProgram>;

  const pollName = "Best Chain";
  const secondsInAWeek = 7 * 24 * 60 * 60;
  const startTime = new anchor.BN(Math.floor(Date.now() / 1000));
  const endTime = new anchor.BN(Math.floor((Date.now() / 1000) + secondsInAWeek));
  const PollDescription = "Which is the best chain?";
  const numberOfOptions = 1;
  const optionNumber = 1;
  const optionDescription = "Solana";

  const [pollPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("poll"), Buffer.from(pollName)],
    program.programId,
  )

  const [optionPublicKey] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("option"), pollPublicKey.toBuffer(), Buffer.from([numberOfOptions])],
    program.programId,
  )

  it("A Pool is created!", async () => {
    const tx = await program.methods
      .createPoll(pollName, numberOfOptions, startTime, endTime, PollDescription)
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("An Option is Added!", async () => {
    const tx = await program.methods
      .addOption(optionNumber, optionDescription)
      .accountsPartial({ poll: pollPublicKey, option: optionPublicKey })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("A vote is cast!", async () => {
    const tx = await program.methods
      .castVote(optionNumber)
      .accountsPartial({ poll: pollPublicKey, option: optionPublicKey })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
