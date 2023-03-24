import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorMovieReview } from "../target/types/anchor_movie_review";

describe("anchor-movie-review", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorMovieReview as Program<AnchorMovieReview>;

  // const movieReviewAccount = anchor.web3.PublicKey.createProgramAddressSync([anchor.workspace.])
  // const counter = anchor.web3.PublicKey.createWithSeed()

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.addMovieReview().accounts([]).rpc();
    console.log("Your transaction signature", tx);
  });
});
