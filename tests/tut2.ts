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
  const receiver = new anchor.web3.PublicKey(
    "8K4Zorrw9kBLMqVEmrABdsjWCSKrcrVvMdJ8CTtyCf2z"
  );
  const mint = new anchor.web3.PublicKey("8BvrrBRZhEst73FbRZRdu1PvKjvL6zvGX3GPVQ3mnhoK")

  const mainUser = provider.publicKey;

  // const program = anchor.workspace.Tut2 as Program<Tut2>;
  let txis = [];

  const pid = new anchor.web3.PublicKey("5Bh7cdEJWWkrJ45d1rsJmo25wwFfMsjQY7j5nHvn9Ztb")
  const _program = new Program(IDL, pid, provider);




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

  // it("Deposit in pda: ", async () => {
  //   const pda = anchor.web3.PublicKey.findProgramAddressSync([
  //     utf8.encode("seed")
  //   ], _program.programId)[0];

  //   const pdaAta = await getOrCreateTokenAccount(mint, pda, true);
  //   const senderAta = await getOrCreateTokenAccount(mint, provider.publicKey);

  //   log("pdaAta: ", pdaAta.toBase58())
  //   log("senderAta: ", senderAta.toBase58())

  //   let ix = createTransferInstruction(senderAta, pdaAta, provider.publicKey, 500 * 1_000);
  //   txis.push(ix);

  //   await sendTransaction();
  // })

  it("Airdrop from pda Account: ", async () => {
    const pda = anchor.web3.PublicKey.findProgramAddressSync([
      utf8.encode("seed")
    ], _program.programId)[0];

    const pdaAta = await getOrCreateTokenAccount(mint, pda, true);
    const receiverAta = await getOrCreateTokenAccount(mint, provider.publicKey);

    log("pdaAta: ", pdaAta.toBase58())

    let ix = await _program.methods.tokenAirdropFromPda(new anchor.BN(3 * 1_000)).accounts({
      mint: mint,
      pda: pda,
      pdaAta: pdaAta,
      receiverAta: receiverAta,
      tokenToken: TOKEN_PROGRAM_ID,
    }).instruction()

    txis.push(ix);

    const tx = new anchor.web3.Transaction();
    tx.add(...txis);

    const res = await provider.sendAndConfirm(tx);
    log("res: ", res);
  })


});