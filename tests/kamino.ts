import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MirrorfiVault } from "../target/types/mirrorfi_vault";

import { 
  KaminoAction, 
  KaminoMarket, 
  KaminoReserve, 
  PROGRAM_ID, 
  DEFAULT_RECENT_SLOT_DURATION_MS,
  getMedianSlotDurationInMsFromLastEpochs,
  VanillaObligation, 
  LendingMarket,
  ReserveConfig,
  refreshReserve
} from '@kamino-finance/klend-sdk';
import { Connection, Keypair, PublicKey, Transaction } from "@solana/web3.js";
import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { BN } from "bn.js";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// We'll use anchor's provider connection and wallet as requested
const connection = provider.connection;
const wallet = provider.wallet;

// Constants for mainnet use
const USDC_MINT = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");
const KAMINO_LENDING_MARKET = new PublicKey("8nU46YepyfwX3CJLeyG1Vx8d3Ug26r6kNVWA2xK7P3hF");
const MAIN_MARKET = new PublicKey("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF");

describe("kamino", () => {
  const program = anchor.workspace.mirrorfiVault as Program<MirrorfiVault>;
  let kaminoMarket: KaminoMarket;

  // Setup - Initialize KaminoMarket instance
  before(async () => {
    console.log("Initializing KaminoMarket...");
    
    try {
      const slotDuration = await getMedianSlotDurationInMsFromLastEpochs();
      // Initialize KaminoMarket with provider's connection and wallet
      kaminoMarket = await KaminoMarket.load(
        connection, 
        MAIN_MARKET,
        slotDuration,
      );
      if (!kaminoMarket) {
        throw Error(`Could not load market ${KAMINO_LENDING_MARKET.toBase58()}`);
      }
      console.log(`Initialized KaminoMarket for market: ${KAMINO_LENDING_MARKET.toBase58()}`);
    } catch (error) {
      console.error("Failed to initialize KaminoMarket:", error);
      throw error;
    }
  });

  it("Get Market Data", async ()=>{
    const reserves = await kaminoMarket.getReserves();
    const reservesArray = Array.from(reserves.values());
    
    console.log("List of Available Reserves:")
    for (const reserve of reservesArray) {
        console.log(reserve.symbol);
        console.log(`- ${reserve.stats}`);
    }
    // console.log(reservesArray);
  })
  
//   it("Fetch and display available reserves", async () => {
//     try {
//       // Get all reserves in the market
//       const reservesMap = await kaminoMarket.getReserves();
      
//       // Convert Map to array of KaminoReserve objects
//       const reserves = Array.from(reservesMap.values());
      
//       console.log(`Found ${reserves.length} reserves in the Kamino lending market`);
      
//       // Display some basic info about each reserve
//       for (const reserve of reserves) {
//         const tokenSymbol = reserve.stats.symbol || "Unknown";
//         const mintPubkey = reserve.getMint().toBase58();
//         const ltv = reserve.getStats().loanToValueRatio / 100;
//         const liquidationThreshold = reserve.getStats().liquidationThreshold / 100;
        
//         console.log(`Reserve: ${tokenSymbol} | Mint: ${mintPubkey.slice(0, 8)}... | LTV: ${ltv}% | Liquidation Threshold: ${liquidationThreshold}%`);
//       }
//     } catch (error) {
//       console.error("Failed to fetch reserves:", error);
//       throw error;
//     }
//   });
  
//   it("Get detailed USDC reserve info", async () => {
//     try {
//       // Find the USDC reserve
//       const usdcReserve = await kaminoMarket.getReserveByMint(USDC_MINT);
//       if (!usdcReserve) {
//         console.log("USDC reserve not found");
//         return;
//       }
      
//       // Get detailed stats about the reserve
//       const stats = usdcReserve.getStats();
      
//       console.log("USDC Reserve Details:");
//       console.log(`- Liquidity Supply: ${stats.totalSupplyUsd.toFixed(2)} USD`);
//       console.log(`- Borrowed Amount: ${stats.totalBorrowUsd.toFixed(2)} USD`);
//       console.log(`- Utilization Rate: ${(stats.utilizationRate * 100).toFixed(2)}%`);
//       console.log(`- Deposit APY: ${(stats.supplyInterestRate * 100).toFixed(2)}%`);
//       console.log(`- Borrow APY: ${(stats.borrowInterestRate * 100).toFixed(2)}%`);
//       console.log(`- Liquidity Available: ${stats.availableLiquidity.toFixed(2)} USDC`);
//     } catch (error) {
//       console.error("Failed to get USDC reserve info:", error);
//       throw error;
//     }
//   });
  
//   it("Get lending market info", async () => {
//     try {
//       // Get lending market data
//       const lendingMarketData = kaminoMarket.getLendingMarketData();
      
//       console.log("Lending Market Info:");
//       console.log(`- Address: ${kaminoMarket.getAddress().toBase58()}`);
//       console.log(`- Authority: ${kaminoMarket.getLendingMarketAuthority().toBase58()}`);
//       console.log(`- Owner: ${lendingMarketData.owner.toBase58()}`);
//       console.log(`- Quote Currency: ${lendingMarketData.quoteCurrency}`);
      
//       // Get total TVL across all reserves
//       const tvl = kaminoMarket.getReserves().reduce((acc, reserve) => acc + reserve.getStats().totalSupplyUsd, 0);
//       console.log(`- Total Value Locked (TVL): $${tvl.toFixed(2)}`);
//     } catch (error) {
//       console.error("Failed to get lending market info:", error);
//       throw error;
//     }
//   });
  
//   it("Check user obligations (if they exist)", async () => {
//     try {
//       // Get user wallet address
//       const walletPubkey = wallet.publicKey;
      
//       // Query user obligations
//       const userObligations = await kaminoMarket.getObligationByWallet(walletPubkey);
      
//       console.log(`Found ${userObligations.length} obligations for wallet: ${walletPubkey.toBase58()}`);
      
//       // Display obligation details if any exist
//       if (userObligations.length > 0) {
//         for (let i = 0; i < userObligations.length; i++) {
//           const obligation = userObligations[i];
          
//           console.log(`\nObligation ${i + 1} Details:`);
//           console.log(`- Address: ${obligation.obligationAddress.toBase58()}`);
//           console.log(`- Borrowed Value: $${obligation.stats.borrowedValueUsd.toFixed(2)}`);
//           console.log(`- Collateral Value: $${obligation.stats.collateralValueUsd.toFixed(2)}`);
//           console.log(`- Health Factor: ${obligation.stats.healthFactor.toFixed(4)}`);
//           console.log(`- LTV: ${(obligation.stats.loanToValue * 100).toFixed(2)}%`);
//         }
//       } else {
//         console.log("No active obligations found for this wallet");
        
//         // Demonstrate how to create a new obligation
//         console.log("\nTo create a new obligation, you would use:")
//         console.log("await kaminoMarket.createObligationForReserves([...reserves], walletPubkey)")
//       }
//     } catch (error) {
//       console.error("Failed to check user obligations:", error);
//       throw error;
//     }
//   });
  
//   it("Simulate deposit transaction (without sending)", async () => {
//     try {
//       // Get the WSOL reserve for deposit simulation
//       const wsolReserve = await kaminoMarket.getReserveByMint(WSOL_MINT);
//       if (!wsolReserve) {
//         console.log("WSOL reserve not found");
//         return;
//       }
      
//       // Amount to deposit (0.1 SOL)
//       const depositAmount = new BN(100000000); // 0.1 SOL in lamports
      
//       // Build deposit transaction (without sending)
//       const depositIx = await KaminoAction.buildDepositReserveLiquidityTxns(
//         connection,
//         depositAmount,
//         WSOL_MINT,
//         wallet.publicKey,
//         KAMINO_LENDING_MARKET,
//         PROGRAM_ID,
//       );
      
//       console.log("Deposit transaction built successfully (simulation only):");
//       console.log(`- Deposit Amount: 0.1 SOL`);
//       console.log(`- Reserve: ${wsolReserve.stats.symbol || "WSOL"}`);
//       console.log(`- Instructions: ${depositIx.instructions.length}`);
//       console.log("- Note: Transaction not sent, only simulated");
      
//     } catch (error) {
//       console.error("Failed to simulate deposit transaction:", error);
//       throw error;
//     }
//   });
  
//   it("Demonstrate how to refresh a reserve", async () => {
//     try {
//       // Get SOL reserve for refresh demonstration
//       const wsolReserve = await kaminoMarket.getReserveByMint(WSOL_MINT);
//       if (!wsolReserve) {
//         console.log("WSOL reserve not found");
//         return;
//       }
      
//       console.log("Building refresh reserve transaction:");
      
//       // Create a transaction to refresh the reserve
//       const refreshTx = new Transaction();
      
//       // Add instruction to refresh the specific reserve
//       refreshTx.add(
//         refreshReserve(
//           PROGRAM_ID,
//           wsolReserve.address,
//           wsolReserve.reserveData.pythOracle,
//           wsolReserve.reserveData.switchboardOracle,
//           KAMINO_LENDING_MARKET,
//         )
//       );
      
//       console.log("Refresh reserve transaction built successfully:");
//       console.log(`- Reserve: ${wsolReserve.stats.symbol || "WSOL"}`);
//       console.log(`- Instructions: ${refreshTx.instructions.length}`);
//       console.log("- Note: Transaction not sent, only demonstrated");
      
//     } catch (error) {
//       console.error("Failed to demonstrate refresh reserve:", error);
//       throw error;
//     }
//   });
});
