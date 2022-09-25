import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { New } from "../target/types/new";

describe("new", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.New as Program<New>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
