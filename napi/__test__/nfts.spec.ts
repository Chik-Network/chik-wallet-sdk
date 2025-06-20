import test from "ava";

import {
  Klvm,
  Coin,
  Constants,
  CreatedDid,
  NftMint,
  PublicKey,
  Simulator,
  standardPuzzleHash,
} from "../index.js";

test("mints and transfers an nft", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(2n);

  // Create a DID
  const { did, parentConditions: didParentConditions } = createDid(
    klvm,
    alice.coin.coinId(),
    alice.pk
  );

  klvm.spendStandardCoin(
    alice.coin,
    alice.pk,
    klvm.delegatedSpend(
      didParentConditions.concat([klvm.createCoin(alice.puzzleHash, 0n)])
    )
  );

  // Mint an NFT
  const mintCoin = new Coin(alice.coin.coinId(), alice.puzzleHash, 0n);

  const {
    nfts: [nft],
    parentConditions: mintParentConditions,
  } = klvm.mintNfts(mintCoin.coinId(), [
    new NftMint(
      klvm.nil(),
      Constants.nftMetadataUpdaterDefaultHash(),
      alice.puzzleHash,
      alice.puzzleHash,
      300,
      null
    ),
  ]);

  klvm.spendStandardCoin(
    mintCoin,
    alice.pk,
    klvm.delegatedSpend(mintParentConditions)
  );

  // Assign the NFT to the DID by spending both
  klvm.spendNft(
    nft,
    klvm.standardSpend(
      alice.pk,
      klvm.delegatedSpend([
        klvm.createCoin(alice.puzzleHash, 1n, klvm.alloc([alice.puzzleHash])),
        klvm.transferNft(did.info.launcherId, [], did.info.innerPuzzleHash()),
      ])
    )
  );

  klvm.spendDid(
    did,
    klvm.standardSpend(
      alice.pk,
      klvm.delegatedSpend([
        klvm.createCoin(alice.puzzleHash, 1n, klvm.alloc([alice.puzzleHash])),
        klvm.createPuzzleAnnouncement(nft.info.launcherId),
      ])
    )
  );

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);

  t.true(true);
});

test("mints 5 nfts", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(6n);

  // Create a DID
  const { did, parentConditions: didParentConditions } = createDid(
    klvm,
    alice.coin.coinId(),
    alice.pk
  );

  klvm.spendStandardCoin(
    alice.coin,
    alice.pk,
    klvm.delegatedSpend(
      didParentConditions.concat([klvm.createCoin(alice.puzzleHash, 0n)])
    )
  );

  // Mint 5 NFTs
  const mintCoin = new Coin(alice.coin.coinId(), alice.puzzleHash, 0n);

  const { nfts, parentConditions: mintParentConditions } = klvm.mintNfts(
    mintCoin.coinId(),
    Array.from(
      { length: 5 },
      () =>
        new NftMint(
          klvm.nil(),
          Constants.nftMetadataUpdaterDefaultHash(),
          alice.puzzleHash,
          alice.puzzleHash,
          300,
          null
        )
    )
  );

  klvm.spendStandardCoin(
    mintCoin,
    alice.pk,
    klvm.delegatedSpend(mintParentConditions)
  );

  // Transfer all of the NFTs to the same p2 puzzle hash
  for (const nft of nfts) {
    klvm.spendNft(
      nft,
      klvm.standardSpend(
        alice.pk,
        klvm.delegatedSpend([
          klvm.createCoin(alice.puzzleHash, 1n, klvm.alloc([alice.puzzleHash])),
        ])
      )
    );
  }

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);

  t.true(true);
});

function createDid(
  klvm: Klvm,
  parentCoinId: Buffer,
  pk: PublicKey
): CreatedDid {
  const p2PuzzleHash = standardPuzzleHash(pk);
  const eveDid = klvm.createEveDid(parentCoinId, p2PuzzleHash);

  klvm.spendDid(
    eveDid.did,
    klvm.standardSpend(
      pk,
      klvm.delegatedSpend([
        klvm.createCoin(
          eveDid.did.info.innerPuzzleHash(),
          1n,
          klvm.alloc([p2PuzzleHash])
        ),
      ])
    )
  );

  return new CreatedDid(
    eveDid.did.child(p2PuzzleHash, eveDid.did.info.metadata),
    eveDid.parentConditions
  );
}
