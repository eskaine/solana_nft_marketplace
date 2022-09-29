import * as anchor from "@project-serum/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Program } from "@project-serum/anchor";
import { NftMarketplace } from "../target/types/nft_marketplace";

describe("mint", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;
  const wallet = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();

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

  it("initialize mint", async () => {
    const tx = await program.methods
        .initializeMint()
        .accounts({
        mint: mint.publicKey,
        payer: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY
    }).signers([wallet, mint]).rpc();

    // to add assert
  });
});
