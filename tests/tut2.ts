import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { IDL, Tut2 } from "../target/types/tut2";
import { readFileSync } from 'fs';
import { utf8 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import {
  createAssociatedTokenAccountInstruction,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
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

  const program = anchor.workspace.Tut1 as Program<Tut2>;
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

  /*
  it("Token Transfer : ", async () => {
    const receiverAta = await getOrCreateTokenAccount(mint, receiver);
    const senderAta = await getOrCreateTokenAccount(mint, signer);

    const ix = await _program.methods.tokenTransfer1(new anchor.BN(1000)).accounts({
      mint: mint,
      receiverAta: receiverAta,
      sender: signer,
      senderAta: senderAta,
      tokenToken: TOKEN_PROGRAM_ID,
    }).instruction();

    txis.push(ix);

    let tx = new anchor.web3.Transaction();
    tx.add(...txis);

    let res = await provider.sendAndConfirm(tx);
    log("Tx res: ", res);
  })
  */

  
  

});