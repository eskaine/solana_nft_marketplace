// import * as anchor from "@project-serum/anchor";
// import * as web3 from "@solana/web3.js";
// import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import { assert } from "chai";
// import { Program } from "@project-serum/anchor";
// import { NftMarketplace } from "../target/types/nft_marketplace";
// const { SystemProgram } = anchor.web3;

// describe("mint", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);

//   const program = anchor.workspace.NftMarketplace as Program<NftMarketplace>;
//   const wallet = anchor.web3.Keypair.generate();
//   const mint = anchor.web3.Keypair.generate()

//   it("initialize mint", async () => {

//     const tx = await program.methods.initializeMint().accounts({
//         mint: mint.publicKey,
//         payer: wallet.publicKey,
//         systemProgram: anchor.web3.SystemProgram.programId,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         rent: anchor.web3.SYSVAR_RENT_PUBKEY
//     }).signers([wallet, mint]).rpc();

//     console.log({tx});
//     // assert.ok(wallet.publ === "Unknown")
//   });
// });



