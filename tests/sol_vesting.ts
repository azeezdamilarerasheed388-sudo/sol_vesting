import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolVesting } from "../target/types/sol_vesting";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("sol_vesting", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolVesting as Program<SolVesting>;
  const wallet = provider.wallet;

  // Derive config PDA
  const [configPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  it("Initialize platform", async () => {
    console.log("🚀 Initializing platform...");
    console.log("Program ID:", program.programId.toString());
    console.log("Config PDA:", configPDA.toString());
    console.log("Owner:", wallet.publicKey.toString());

    // Check if already initialized
    try {
      const existing = await program.account.feeConfig.fetch(configPDA);
      console.log("⚠️ Platform already initialized!");
      console.log("Config:", existing);
      return;
    } catch (e) {
      console.log("✅ Config not found - will initialize");
    }

    // Fee parameters
    const feeCollector = wallet.publicKey;
    const singleVestingFee = new anchor.BN(10_000_000); // 0.01 SOL
    const batchVestingFeeBps = 50; // 0.5%
    const batchMinFee = new anchor.BN(100_000_000); // 0.1 SOL
    const batchMaxFee = new anchor.BN(10_000_000_000); // 10 SOL

    try {
      // Send transaction
      const txHash = await program.methods
        .initialize(
          feeCollector,
          singleVestingFee,
          batchVestingFeeBps,
          batchMinFee,
          batchMaxFee
        )
        .accounts({
          config: configPDA,
          owner: wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      console.log("✅ Transaction sent:", txHash);

      // Confirm
      await provider.connection.confirmTransaction(txHash, "confirmed");
      console.log("✅ Transaction confirmed!");

      // Fetch and verify
      const config = await program.account.feeConfig.fetch(configPDA);
      
      console.log("\n🎉 Platform initialized successfully!");
      console.log("──────────────────────────────");
      console.log("Owner:", config.owner.toString());
      console.log("Fee Collector:", config.feeCollector.toString());
      console.log("Single Fee:", config.singleVestingFixedFee.toString(), "lamports");
      console.log("Batch BPS:", config.batchVestingFeeBps);
      console.log("──────────────────────────────");

      // Assertions
      assert.ok(config.owner.equals(wallet.publicKey));
      assert.ok(config.feeCollector.equals(feeCollector));
      assert.ok(config.singleVestingFixedFee.eq(singleVestingFee));
      assert.equal(config.batchVestingFeeBps, batchVestingFeeBps);
      
    } catch (error) {
      console.error("❌ Error:", error);
      throw error;
    }
  });
});