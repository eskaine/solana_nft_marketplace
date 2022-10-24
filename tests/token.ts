import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { NftMarketplace } from "../target/types/nft_marketplace";

describe("token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const nftTitle = "Beta";
  const nftSymbol = "BETA";
  const nftUri = "https://raw.githubusercontent.com/Coding-and-Crypto/Solana-NFT-Marketplace/master/assets/example.json";

  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;
  const user = anchor.web3.Keypair.generate();

  const mint = anchor.web3.Keypair.generate();
  const metadataId = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );

  let nftToken: PublicKey = null;
  let metadata: PublicKey = null;

  const royalty = 0;

  it("initializes wallet account", async () => {
    const airdropWalletSig = await provider.connection.requestAirdrop(
      user.publicKey,
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
    const [pda] = await PublicKey.findProgramAddress(
      [
        user.publicKey.toBuffer(),
      ],
      new PublicKey(program.programId)
    );

    await program.methods
      .createUser()
      .accounts({
        initializer: user.publicKey,
        userAccount: pda,
      })
      .signers([user])
      .rpc();
  });

  it("initialize mint", async () => {
    nftToken = await anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: user.publicKey
    });

    const [_metadata] = await PublicKey.findProgramAddress(
      [
        Buffer.from("metadata"),
        metadataId.toBuffer(),
        mint.publicKey.toBuffer(),
      ],
      metadataId
    );
    metadata = _metadata;

    await program.methods
        .createNft(nftTitle, nftSymbol, nftUri, royalty)
        .accounts({
          payer: user.publicKey,
          mintAuthority: user.publicKey,
          mint: mint.publicKey,
          nftToken: nftToken,
          metadata: metadata,
          tokenMetadataProgram: metadataId,
    })
    .signers([user, mint])
    .rpc();
  });
});
