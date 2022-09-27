import * as anchor from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { assert } from "chai";
import { Program } from "@project-serum/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";
const { SystemProgram } = anchor.web3;

describe("user", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;
  const wallet = anchor.web3.Keypair.generate();

  it("initializes wallet account", async () => {
    const airdropWalletSig = await provider.connection.requestAirdrop(
      wallet.publicKey,
      2e9
    );

    const walletBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: walletBlockhash.blockhash,
      lastValidBlockHeight: walletBlockhash.lastValidBlockHeight,
      signature: airdropWalletSig,
    });
  });

  it("create user", async () => {
    const [pda] = await web3.PublicKey.findProgramAddress(
      [
        wallet.publicKey.toBuffer(),
      ],
      new web3.PublicKey(program.programId)
    );

    await program.methods
      .createUser()
      .accounts({
        initializer: wallet.publicKey,
        userAccount: pda,
        systemProgram: SystemProgram.programId
      })
      .signers([wallet])
      .rpc();

    const pdaUser = await program.account.user.fetch(pda);

    assert.ok(pdaUser.name === "Unknown")
  });
});
