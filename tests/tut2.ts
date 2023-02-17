import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IDL, Tut2 } from "../target/types/tut2";
import { readFileSync } from 'fs';
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  createTransferInstruction,
} from '@solana/spl-token';
const log = console.log;

describe("tut1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const signer = provider.publicKey;
  const connection = provider.connection;
  const buyer = new anchor.web3.PublicKey(
    "8K4Zorrw9kBLMqVEmrABdsjWCSKrcrVvMdJ8CTtyCf2z"
  );
  const mint = new anchor.web3.PublicKey("CGdNrSHN7WEattbAmfRYiwZQCVkNezxPzj4T1gtbNyNc")

  const mainUser = provider.publicKey;

  // const program = anchor.workspace.Tut2 as Program<Tut2>;
  let txis = [];

  const pid = new anchor.web3.PublicKey("AdAer5ihhyVQAgxZBTZpgLZG9kFBcFrJ9PEH42AtyFtT")
  const _program = new Program(IDL, pid, provider);
  const pda = anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("_seed")
  ], _program.programId)[0];
  const solCollector = new anchor.web3.PublicKey("7D6StyJSfQJ2d28weVscUn4frrsi9VLeQDCi8uvRtx63");

  async function getOrCreateTokenAccount(mint: anchor.web3.PublicKey, owner: anchor.web3.PublicKey, offCurve = false) {
    const ata = getAssociatedTokenAddressSync(mint, owner, offCurve);

    const ataInfo = await connection.getAccountInfo(ata);

    if (ataInfo == null) {
      const it = createAssociatedTokenAccountInstruction(signer, ata, owner, mint);
      txis.push(it)
    }

    return ata;
  }

  async function sendTransaction(keyPairs: anchor.web3.Keypair[] = []) {
    let tx = new anchor.web3.Transaction();
    tx.add(...txis);
    txis = [];

    for (let i of keyPairs) tx.sign(i);

    const res = await provider.sendAndConfirm(tx);
    log("res: ", res);
  }

  // it("init pda", async () => {
  //   let res = await _program.methods.initPda(solCollector).accounts({
  //     owner: provider.publicKey,
  //     pda: pda,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   log("res: ", res);
  // })

  // it('Update sol receiver ', async () => {
  //   let res = await _program.methods.changeSolReceiver(new anchor.web3.PublicKey("Fq7sAPdCADBHz7CKYgxGErvaRwm794bLxhJHL57yxAgC")).accounts({
  //     owner: provider.publicKey,
  //     pda: pda,
  //   }).rpc();

  //   log("res: ", res);
  // })

  // it("Deposit in pda: ", async () => {
  //   const pda = anchor.web3.PublicKey.findProgramAddressSync([
  //     utf8.encode("_seed")
  //   ], _program.programId)[0];

  //   const pdaAta = await getOrCreateTokenAccount(mint, pda, true);
  //   const senderAta = await getOrCreateTokenAccount(mint, provider.publicKey);

  //   log("pdaAta: ", pdaAta.toBase58())
  //   log("senderAta: ", senderAta.toBase58())

  //   let ix = createTransferInstruction(senderAta, pdaAta, provider.publicKey, 500 * 1_000);
  //   txis.push(ix);

  //   await sendTransaction();
  // })

  // it("set price ", async () => {
  //   const price = Math.trunc(0.003 * 1000_000_000);
  //   let res = await _program.methods.setPrice(new anchor.BN(price)).accounts({
  //     owner: provider.publicKey,
  //     pda: pda,
  //   }).rpc()

  //   log("res: ", res);
  // })

  it("pda info fetching: ", async () => {
    let res = await _program.account.pdaInfo.fetch(pda);
    let _solReceiver = res.solReceiver;
    let _owner = res.owner;
    let _soldAmount = res.soldAmount.toNumber();
    let _price = res.price;

    let obj = {
      solReceiver: _solReceiver.toBase58(),
      owner: _owner.toBase58(),
      soldAmount: _soldAmount / 10_000,
      price: _price,
    }

    log(obj);
  })

  // it("buy from pda Account: ", async () => {
  //   const pdaAta = await getOrCreateTokenAccount(mint, pda, true);
  //   const buyerAta = await getOrCreateTokenAccount(mint, provider.publicKey);

  //   log("pdaAta: ", pdaAta.toBase58())

  //   let ix = await _program.methods.buyToken(new anchor.BN(3 * 10_000)).accounts({
  //     mint: mint,
  //     pda: pda,
  //     pdaAta: pdaAta,
  //     buyer: provider.publicKey,
  //     buyerAta: buyerAta,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     solCollector: solCollector, //0x1770 -> 6000
  //     // solCollector: new anchor.web3.PublicKey("Fq7sAPdCADBHz7CKYgxGErvaRwm794bLxhJHL57yxAgC"),
  //   }).instruction()

  //   txis.push(ix);

  //   const tx = new anchor.web3.Transaction();
  //   tx.add(...txis);

  //   const res = await provider.sendAndConfirm(tx);
  //   log("res: ", res);
  // })

});