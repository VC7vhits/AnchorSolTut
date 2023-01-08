import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tut2 } from "../target/types/tut2";

const log = console.log;

describe("tut2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  let myKey = provider.publicKey;

  const program = anchor.workspace.Tut2 as Program<Tut2>;
  const receiver = new anchor.web3.PublicKey("4MMr4CMFuyBmpbK3RvwdxaipMvkUY4Y25pKYgtEuMYjG");

  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   // let res = await program.methods.initialize(new anchor.BN(12))
  //   let res = await program.methods.initialize(new anchor.BN(10)).accounts({
  //     sender: provider.publicKey,
  //     receiver: receiver,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     // }).rpc();
  //   }).instruction();

  //   let tx = new anchor.web3.Transaction();
  //   tx.add(res);

  //   // tx.add(res);
  //   // tx.add(res);

  //   // let r = await provider.connection.getLatestBlockhash();    
  //   // let blockHash = r.blockhash;

  //   // tx.feePayer = provider.publicKey;
  //   // tx.recentBlockhash = blockHash;

  //   // let kp1 = anchor.web3.Keypair.generate();

  //   const sign = await provider.sendAndConfirm(tx);
  //   console.log("tx sign: ", sign)

  // });

  it("Sol transfer native: ", async () => {
    let ix = anchor.web3.SystemProgram.transfer(
      {
        fromPubkey: provider.publicKey,
        toPubkey: receiver,
        lamports: 1
      });
      
      
    let tx = new anchor.web3.Transaction();
    tx.add(ix);

    tx.feePayer = provider.publicKey;
    let r1 = await provider.connection.getLatestBlockhash();
    let blockHash = r1.blockhash;
    tx.recentBlockhash = blockHash;

    let sign = await provider.sendAndConfirm(tx);

    log("Tx Sign: ", sign);
    

  })




  // async function getData() {
  //   let k = new anchor.web3.PublicKey("GPv247pHoMhA6MFdLmzXzA9JdmVgn6g1VvLUS8kn38Ej")
  //   let a = await provider.connection.getAccountInfo(k);

  //   return a;
  // }

  // it("async :", async () => {
  //   log("start");
  //   let res = await getData();
  //   log("res: ", res)
  //   log('end');

  //   // let data = (await res)
  //   // log("data ", data);

  // })

});
