const assert = require("assert");
import * as anchor from "@coral-xyz/anchor";
//import { SystemProgram } from "@coral-xyz/anchor";
const { SystemProgram } = anchor.web3;

describe("mycalculatordapp", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const calculator = anchor.web3.Keypair.generate();
  const program = anchor.workspace.mycalculatordapp;

  it("Create a calculator", async () => {
    await program.rpc.create("Welcome to solana", {
      accounts: {
        calculator: calculator.publicKey,
        user: provider.wallet.publicKey,
        systemProgrem: SystemProgram.programId,
      },
      signers: [calculator],
    });
    const account = await program.account.calculator.fetch(
      calculator.publicKey
    );
    assert.ok(account.greeting === "Welcome to solana");
  });
  it("Add two  number", async () => {
    await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
      account: {
        calculator: calculator.publicKey,
      },
    });
    const account = await program.account.calculator.feth(calculator.publicKey);
    assert.ok(account.result.eq(new anchor.BN(5)));
  });
});
