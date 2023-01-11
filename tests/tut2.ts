import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tut2 } from "../target/types/tut2";
import { readFileSync } from 'fs';


const log = console.log;

describe("tut2", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.Tut2 as Program<Tut2>;
  const receiver = new anchor.web3.PublicKey("4MMr4CMFuyBmpbK3RvwdxaipMvkUY4Y25pKYgtEuMYjG");

  it("sol transfer through Our Contract", async () => {
    let sign = await program.methods.solTransfer(new anchor.BN(1)).accounts({
      sender: provider.publicKey,
      receiver: receiver,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
      .rpc();

    console.log("Transaction Sign: ", sign);


    //? Basically the .rpc() methdos hides much more things
    //* here it ix means the instruction. => a Transaction is made by the instructions.
    let ix = await program.methods.solTransfer(new anchor.BN(1)).accounts({
      sender: provider.publicKey,
      receiver: receiver,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
      .instruction();

    let tx = new anchor.web3.Transaction();
    tx.add(ix); //? Addding the transaction.

    tx.feePayer = provider.publicKey; //? to execute the transaction in blockchain it's take fees
    //? to execute the transaction, here it's require the feePayer.


    //? To send transaction it's require the recentBlockhash,
    //? may be it's user the find out the time of transaction when it was sended.
    const blockHashResult = await provider.connection.getLatestBlockhash()
    tx.recentBlockhash = blockHashResult.blockhash


    //? every transaction is signed by at least one keypair.
    //? because to make the trasaction it's require to the feesPayer sign.
    //? by calling `provider.sendAndConfirm` it's automatically sign the trasaction 
    //? by taking the keypair from the provider.
    // let a = anchor.web3.Keypair.generate();
    // tx.sign(a);

    let result = await provider.sendAndConfirm(tx);
    log("Transaction result: ", result);
  })


  //? Sending sol by direct calling system program.
  it("Sol Transafer (Native)", async () => {
    let ix = anchor.web3.SystemProgram.transfer({
      fromPubkey: provider.publicKey,
      toPubkey: receiver,
      lamports: 2
    })


    
    let tx = new anchor.web3.Transaction();
    
    tx.add(ix);

    tx.feePayer = provider.publicKey;

    const blockHashResult = await provider.connection.getLatestBlockhash()
    tx.recentBlockhash = blockHashResult.blockhash

    let result = await provider.sendAndConfirm(tx);
    log("Transaction result: ", result);
  })
  
  it("Native Sol transfer 2 ", async () => {
     let ix = anchor.web3.SystemProgram.transfer({
          fromPubkey: mainUser,
          toPubkey: receiver,
          lamports: 1,
      });
    
      let tx = new anchor.web3.Transaction().add(ix);
      let res = await provider.sendAndConfirm(tx);
      log("transaction : ", res);
  });

});

