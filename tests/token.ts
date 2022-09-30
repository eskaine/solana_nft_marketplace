import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import * as web3 from "@solana/web3.js";
import { PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { NftMarketplace } from "../target/types/nft_marketplace";

describe("token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;
  const user = anchor.web3.Keypair.generate();
  const mint = anchor.web3.Keypair.generate();

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
    const [pda] = await web3.PublicKey.findProgramAddress(
      [
        user.publicKey.toBuffer(),
      ],
      new web3.PublicKey(program.programId)
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
    const nftToken = await anchor.utils.token.associatedAddress({
      mint: mint.publicKey,
      owner: user.publicKey
    });

    await program.methods
        .createToken()
        .accounts({
          user: user.publicKey,
          mint: mint.publicKey,
          nftToken: nftToken,
    }).signers([user, mint]).rpc();
  });
});
