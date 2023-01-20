import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IDL, Tut2 } from "../target/types/tut2";
import { readFileSync } from 'fs';
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
const log = console.log;

describe("tut1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const connection = provider.connection;
  const receiver = new anchor.web3.PublicKey(
    "8K4Zorrw9kBLMqVEmrABdsjWCSKrcrVvMdJ8CTtyCf2z"
  );

  const mainUser = provider.publicKey;

  const program = anchor.workspace.Tut1 as Program<Tut2>;

  // let fileTxt = readFileSync("./_user/additions.json", { encoding: 'utf-8' });
  // let kepairJson = JSON.parse(fileTxt);
  // let buffers_8 = Uint8Array.from(kepairJson);
  // let account = anchor.web3.Keypair.fromSecretKey(buffers_8);

  /*
  ///NOTE: Test for simple to log on blockchain.
  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  //NOTE: Here we are calling the systemProgram to send sol from our smart contract
  it("Sol transfer from our contract : ", async () => {
    const ix = await program.methods.solTransfer(new anchor.BN(503)).accounts({
      sender: mainUser,
      receiver: receiver,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).instruction()

    const tx = new anchor.web3.Transaction(); //NOTE: creating the transaction 
    tx.add(ix); //NOTE: adding the instruction to the trasaction

    let res = await provider.sendAndConfirm(tx);
    log("Transaction res: ", res);
  })

  ///NOTE: Here we are calling the systemProgram directly from the OFF-chain
  it("Native Sol transfer ", async () => {
    let ix = anchor.web3.SystemProgram.transfer({
      fromPubkey: mainUser,
      toPubkey: receiver,
      lamports: 1,
    });

    let tx = new anchor.web3.Transaction().add(ix);

    let res = await provider.sendAndConfirm(tx);
    log("transaction : ", res);
  });


  ////NOTE: it's space allocation in blockchain to store the sum to two value
  it("initialize account to store addition value: ", async () => {
    let res = await program.methods.initAccount().
      accounts({
        account: account.publicKey,
        user: mainUser,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([account])
      .rpc()

    log("Tx res: ", res);
  })

  ///NOTE: here we are sending the two value from OFF-chain and storing the sum in blockchain(in account address)
  it("Addtion : ", async () => {
    let res = await program.methods.add(1, 3)
      .accounts({
        account: account.publicKey,
      })
      .signers([account])
      .rpc();

    log("Tx res: ", res)
  })


  //NOTE: Here we are Fetching the account address data and reading the value in the account address from OFF-chain
  it("Fetching value from account : ", async () => {
    let data = await program.account.answer.fetch(account.publicKey);
    log("data: ", data);
  })


  /// Addition on Pda:
  const [pda, _bump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      // utf8.encode("123"),
      provider.publicKey.toBuffer()
    ],
    program.programId
  );

  it("init Pda Account:", async () => {
    log("pda: ", pda.toBase58());

    let sign = await program.methods.initPdaAccount().accounts({
      account: pda,
      user: provider.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    log("Tx Sign: ", sign);
  })

  it("Store addition in pda Account: ", async () => {
    const sign = await program.methods.addInPda(1, 5).accounts({
      account: pda,
      user: provider.publicKey
    }).rpc();

    log("Tx Sign: ", sign);
  })
  */

  it("Aidrop", async () => {
    // const pid = new anchor.web3.PublicKey("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb")
    // const _program = new Program(IDL,pid, provider);
    
    let [pda, bump] = anchor.web3.PublicKey.findProgramAddressSync([
      utf8.encode("ad")
    ], program.programId)
  // ], pid)

  

    let res = await program.methods.airdrop(new anchor.BN(100)).accounts({
      pda: pda,
      receiver: provider.publicKey
    }).rpc();

    console.log("res: ", res);
  })

});