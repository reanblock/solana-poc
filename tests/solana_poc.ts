import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { SolanaPoc } from "../target/types/solana_poc";
 
describe("Solana PoC Program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
 
  const program = anchor.workspace.SolanaPoc as Program<SolanaPoc>;
 
  const counter = anchor.web3.Keypair.generate();
 
  it("Is initialized!", async () => {
    const tx = await program.methods
      .initializeCounter()  
      .accounts({ counter: counter.publicKey })
      .signers([counter])
      .rpc();
   
    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(0);
  });
 
  it("Incremented the count", async () => {
    const tx = await program.methods
      .incrementCounter()
      .accounts({ counter: counter.publicKey, user: provider.wallet.publicKey })
      .rpc();
   
    const account = await program.account.counter.fetch(counter.publicKey);
    expect(account.count.toNumber()).to.equal(1);
  });
});