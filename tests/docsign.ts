import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Docsign } from "../target/types/docsign";

describe("docsign", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.docsign as Program<Docsign>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.setDocument("we are XELIO founders. Global Mpesa for stablecoins. Future of finance!", ["JVM", "Abhi-the banker", "10K"]).rpc();
    console.log("Your transaction signature", tx);
  });
});
