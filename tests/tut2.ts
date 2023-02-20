import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tut2 } from "../target/types/tut2";
import { readFileSync } from 'fs';
import { } from '@project-serum/anchor'


const log = console.log;

describe("tut2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  let myKey = provider.publicKey;

  let fileTxt = readFileSync("./_users/additions.json", { encoding: 'utf-8' });
  let kepairJson = JSON.parse(fileTxt);
  let buffers_8 = Uint8Array.from(kepairJson);
  let account = anchor.web3.Keypair.fromSecretKey(buffers_8);

  const program = anchor.workspace.Tut2 as Program<Tut2>;

  it("init account", async () => {
    let sign = program.methods.initialize().accounts({
      user: provider.publicKey,
      account: account.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
      .signers([account])
      .rpc();

    log("Tx Sign", sign);
  })

  it('Additions', async () => {
    let sign = program.methods.add(1, 4).accounts({
      account: account.publicKey,
    })
      .signers([account])
      .rpc();


    log("Tx Sing: ", sign);
  })

  it("geting result: ", async () => {
    let accountDetails = await program.account.answer.fetch(account.publicKey);
    let sum = accountDetails.res;
    log("Result: ", sum)
  })

});

