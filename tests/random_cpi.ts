import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { MirrorfiVault } from "../target/types/mirrorfi_vault";
import { assert } from "chai";

describe("random_cpi test", () => {
  // Use the existing provider
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Load the MirrorFi Vault program
  const program = anchor.workspace.MirrorfiVault as Program<MirrorfiVault>;

  // Define the protocol address (fixed address for the test)
  const protocolAddress = new PublicKey("5bRhcSASW1CaonRHa3ceu5eVPVuvk4wGcP9qgF1qFsGw");
  
  // The Pluto Leverage program ID
  const plutoLeverageProgramId = new PublicKey("DNcR7b5ZpU7X4nTa62sTmroyvsSa52d66hunbCaMUjq2");

  it("Should execute plutonian_initialize via CPI", async () => {
    console.log("Starting random_cpi test...");
    console.log("User wallet:", provider.wallet.publicKey.toString());
    console.log("Protocol address:", protocolAddress.toString());
    
    // Derive the plutonian PDA
    const [plutonian] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("pv2_plutonian_d01", 'utf8'),
        protocolAddress.toBuffer(),
        provider.wallet.publicKey.toBuffer()
      ],
      plutoLeverageProgramId
    );
    console.log("Plutonian address:", plutonian.toString());
    
    // Derive the plutonian authority PDA
    const [plutonianAuthority] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("pv2_plutonian_auth_d01", 'utf8'),
        provider.wallet.publicKey.toBuffer(),
      ],
      plutoLeverageProgramId
    );
    console.log("Plutonian authority address:", plutonianAuthority.toString());
    
    // Derive the event authority PDA
    const [eventAuthority] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("__event_authority", 'utf8'),
      ],
      plutoLeverageProgramId
    );
    console.log("Event authority address:", eventAuthority.toString());
    
    try {
      // Create transaction with compute budget instructions
      const instructions = [
        anchor.web3.ComputeBudgetProgram.setComputeUnitLimit({
          units: 1000000,
        }),
        anchor.web3.ComputeBudgetProgram.setComputeUnitPrice({
          microLamports: 0,
        }),
      ];
      
      // Add our random_cpi instruction
      const tx = await program.methods
        .randomCpi()
        .accounts({
          actor: provider.wallet.publicKey,
          user: provider.wallet.publicKey, // user is the same as actor in this test
          protocol: protocolAddress,
          plutonian: plutonian,
          plutonianAuthority: plutonianAuthority,
          //systemProgram: anchor.web3.SystemProgram.programId,
          eventAuthority: eventAuthority,
          program: plutoLeverageProgramId,
        })
        .transaction();
      
      instructions.push(tx.instructions[0]);
      
      // Create a versioned transaction
      const recentBlockhash = await provider.connection.getLatestBlockhash();
      const messageV0 = new anchor.web3.TransactionMessage({
        payerKey: provider.wallet.publicKey,
        recentBlockhash: recentBlockhash.blockhash,
        instructions,
      }).compileToV0Message([]);
      
      const versionedTransaction = new anchor.web3.VersionedTransaction(messageV0);
      
      // Sign the transaction
      const signedTx = await provider.wallet.signTransaction(versionedTransaction);
      
      // Simulate the transaction first
      console.log("Simulating transaction...");
      const simulation = await provider.connection.simulateTransaction(signedTx);
      console.log("Simulation result:", simulation.value.err ? "Error" : "Success");
      
      if (simulation.value.err) {
        console.error("Simulation error:", simulation.value.err);
      }
      
      // Log simulation logs
      if (simulation.value.logs) {
        console.log("Simulation logs:");
        simulation.value.logs.forEach(log => console.log(log));
      }
      
      // If simulation succeeds or we want to try anyway, send the transaction
      console.log("Sending transaction...");
      const txSignature = await provider.sendAndConfirm(signedTx, [], {
        skipPreflight: true,
      });
      
      console.log("Transaction confirmed!");
      console.log("Transaction signature:", txSignature);
      
      // Verify the plutonian account was created
      // You could add verification logic here if needed
      
    } catch (error) {
      console.error("Error executing random_cpi:", error);
      throw error;
    }
  });
});
