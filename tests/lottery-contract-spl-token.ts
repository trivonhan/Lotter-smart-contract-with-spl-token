import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Connection, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { LotteryContractSplToken } from "../target/types/lottery_contract_spl_token";
import { SolanaConfigService} from '@coin98/solana-support-library/config'
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import {
  Account,
  createMint,
  createTransferInstruction,
  getAccount,
  getMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from '@solana/spl-token'; 

describe("lottery-contract-spl-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LotteryContractSplToken as Program<LotteryContractSplToken>;

  const connection = new Connection('http://127.0.0.1:8899', 'confirmed');

  let root : anchor.web3.Keypair;
  let player0 : anchor.web3.Keypair;
  let player1: anchor.web3.Keypair;
  let player2: anchor.web3.Keypair;
  let player0ATA: Account;
  let player1ATA: Account;
  let player2ATA: Account;
  let mint: anchor.web3.PublicKey;

  before(async () => {
    root = await SolanaConfigService.getDefaultAccount();
    console.log('Payer', root.publicKey.toBase58());

    mint = await createMint(
      connection,
      root,
      root.publicKey,
      root.publicKey,
      9 // We are using 9 to match the CLI decimal default exactly
    );
    console.log('Mint account address', mint.toBase58());

    player0 = anchor.web3.Keypair.generate();
    const airdropSignature = await connection.requestAirdrop(player0.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature);
    player0ATA = await getOrCreateAssociatedTokenAccount(
      connection,
      player0,
      mint,
      player0.publicKey
    );
    console.log('Player 0 ATA', player0ATA.address.toBase58());

    await mintTo(
      connection,
      player0,
      mint,
      player0ATA.address,
      root,
      10000000000 // because decimals for the mint are set to 9 
    )

    player1 = anchor.web3.Keypair.generate();
    const airdropSignature1 = await connection.requestAirdrop(player1.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature1);
    player1ATA = await getOrCreateAssociatedTokenAccount(
      connection,
      player1,
      mint,
      player1.publicKey
    );
    console.log('Player 1 ATA', player1ATA.address.toBase58());

    await mintTo(
      connection,
      player1,
      mint,
      player1ATA.address,
      root,
      10000000000 // because decimals for the mint are set to 9 
    )

    player2 = anchor.web3.Keypair.generate();
    const airdropSignature2 = await connection.requestAirdrop(player2.publicKey, LAMPORTS_PER_SOL);
    await connection.confirmTransaction(airdropSignature2);
    player2ATA = await getOrCreateAssociatedTokenAccount(
      connection,
      player2,
      mint,
      player2.publicKey
    );
    console.log('Player 2 ATA', player2ATA.address.toBase58());

    await mintTo(
      connection,
      player2,
      mint,
      player2ATA.address,
      root,
      10000000000 // because decimals for the mint are set to 9 
    )
})

it("Lottery for solana", async () => {

  const [lotteryMasterPda, lotteryMasterBump] = findProgramAddressSync([Buffer.from('INIT_LOTTERY'), root.publicKey.toBuffer()], program.programId);

  console.log('program', program.programId.toString());

  const tx = await program.methods.initLotteryMaster().accounts({
    root: root.publicKey,
    lotteryMaster: lotteryMasterPda,
  }).signers([root]).rpc();

  console.log('Init lottery master transaction', tx);

  const lotteryMasterAccount = await program.account.lotteryMaster.fetch(lotteryMasterPda);
  console.log('Lottery account', lotteryMasterAccount);


  const [lotteryPda, lotteryBump] = findProgramAddressSync([Buffer.from('INIT_LOTTERY'), Buffer.from([lotteryMasterAccount.lotteryCount])], program.programId);
    
  const [lotteryTokenAccountPda, lotteryTokenAccountBump] = findProgramAddressSync([Buffer.from('LOTTERY_TOKEN_ACCOUNT'), Buffer.from([lotteryMasterAccount.lotteryCount])], program.programId);
  

  console.log(`Lottery PDA ${lotteryPda}, ${lotteryBump}` );

  const txInitLottery = await program.methods.initLottery().accounts({
    root: root.publicKey,
    lotteryMaster: lotteryMasterPda,
    lotteryState: lotteryPda,
    lotteryTokenAccount: lotteryTokenAccountPda,
    tokenMint: mint,
  }).signers([root]).rpc();

  console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

  await program.methods.addMoneyToLottery(0).accounts({
    player: player0.publicKey,
    playerTokenAccount: player0ATA.address,
    lotteryState: lotteryPda,
    lotteryTokenAccount: lotteryTokenAccountPda,
    tokenMint: mint,
  }).signers([player0]).rpc();

  console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

  await program.methods.addMoneyToLottery(0).accounts({
    player: player1.publicKey,
    playerTokenAccount: player1ATA.address,
    lotteryState: lotteryPda,
    lotteryTokenAccount: lotteryTokenAccountPda,
    tokenMint: mint,
  }).signers([player1]).rpc();

  console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

  await program.methods.addMoneyToLottery(0).accounts({
    player: player2.publicKey,
    playerTokenAccount: player2ATA.address,
    lotteryState: lotteryPda,
    lotteryTokenAccount: lotteryTokenAccountPda,
    tokenMint: mint,
  }).signers([player2]).rpc();

  console.log('Lottery account', await program.account.lottery.fetch(lotteryPda));

  const txPickWinner = await program.methods.pickWinner(0).accounts({
    root: root.publicKey,
    lotteryState: lotteryPda,
  }).signers([root]).rpc();

  try{    
    const txClaim0 = await program.methods.claim(0, lotteryBump).accounts({
      player: player0.publicKey,
      playerTokenAccount: player0ATA.address,
      lotteryState: lotteryPda,
      lotteryTokenAccount: lotteryTokenAccountPda,
    }).signers([player0]).rpc();
    console.log('Claim 0', txClaim0);
  }
  catch(error) {
    console.log(error.logs);
  }

  await new Promise(f => setTimeout(f, 1000));
  let balancePlayer0 = Number((await getAccount(connection, player0ATA.address)).amount);
  console.log('Balance of player 0', balancePlayer0/LAMPORTS_PER_SOL);

  try{
    const txClaim1 = await program.methods.claim(0, lotteryBump).accounts({
    player: player1.publicKey,
    playerTokenAccount: player1ATA.address,
    lotteryState: lotteryPda,
    lotteryTokenAccount: lotteryTokenAccountPda,
  }).signers([player1]).rpc();
    console.log('Claim 1', txClaim1);
  }
  catch(error) {
    console.log(error.logs);
  }

  await new Promise(f => setTimeout(f, 1000));
  let balancePlayer1 = Number((await getAccount(connection, player1ATA.address)).amount);
  console.log('Balance of player 1', balancePlayer1 / LAMPORTS_PER_SOL);

  try{
    const txClaim2 = await program.methods.claim(0, lotteryBump).accounts({
      player: player2.publicKey,
      playerTokenAccount: player2ATA.address,
      lotteryState: lotteryPda,
      lotteryTokenAccount: lotteryTokenAccountPda,
    }).signers([player2]).rpc();
    console.log('Claim 2', txClaim2);
  }
  catch(error) {
    console.log(error.logs);
  }
  await new Promise(f => setTimeout(f, 1000));
  let balancePlayer2 = Number((await getAccount(connection, player2ATA.address)).amount);
  console.log('Balance of player 2', balancePlayer2 / LAMPORTS_PER_SOL);

});

});

