import test from "ava";

import {
  BlsPair,
  ClawbackV2,
  Klvm,
  Coin,
  Simulator,
  standardPuzzleHash,
} from "../index.js";

test("test clawback v2 (sender spend)", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(1n);
  const bob = BlsPair.fromSeed(42n);
  const bobPuzzleHash = standardPuzzleHash(bob.pk);

  const clawback = new ClawbackV2(
    alice.puzzleHash,
    bobPuzzleHash,
    5n,
    1n,
    false
  );

  klvm.spendStandardCoin(
    alice.coin,
    alice.pk,
    klvm.delegatedSpend([
      klvm.createCoin(
        clawback.puzzleHash(),
        1n,
        klvm.alloc([clawback.memo(klvm)])
      ),
    ])
  );

  const clawbackCoin = new Coin(alice.coin.coinId(), clawback.puzzleHash(), 1n);

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);

  const clawbackSpend = clawback.senderSpend(
    klvm.standardSpend(
      alice.pk,
      klvm.delegatedSpend([klvm.createCoin(alice.puzzleHash, 1n)])
    )
  );
  klvm.spendCoin(clawbackCoin, clawbackSpend);

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);

  t.true(true);
});

test("test clawback v2 (receiver spend)", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(1n);
  const bob = BlsPair.fromSeed(42n);
  const bobPuzzleHash = standardPuzzleHash(bob.pk);

  const clawback = new ClawbackV2(
    alice.puzzleHash,
    bobPuzzleHash,
    5n,
    1n,
    false
  );

  klvm.spendStandardCoin(
    alice.coin,
    alice.pk,
    klvm.delegatedSpend([
      klvm.createCoin(
        clawback.puzzleHash(),
        1n,
        klvm.alloc([clawback.memo(klvm)])
      ),
    ])
  );

  const clawbackCoin = new Coin(alice.coin.coinId(), clawback.puzzleHash(), 1n);

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);
  sim.passTime(10n);

  const clawbackSpend = clawback.receiverSpend(
    klvm.standardSpend(
      bob.pk,
      klvm.delegatedSpend([klvm.createCoin(bobPuzzleHash, 1n)])
    )
  );
  klvm.spendCoin(clawbackCoin, clawbackSpend);

  sim.spendCoins(klvm.coinSpends(), [bob.sk]);

  t.true(true);
});

test("test clawback v2 (push through)", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(1n);
  const bob = BlsPair.fromSeed(42n);
  const bobPuzzleHash = standardPuzzleHash(bob.pk);

  const clawback = new ClawbackV2(
    alice.puzzleHash,
    bobPuzzleHash,
    5n,
    1n,
    false
  );

  klvm.spendStandardCoin(
    alice.coin,
    alice.pk,
    klvm.delegatedSpend([
      klvm.createCoin(
        clawback.puzzleHash(),
        1n,
        klvm.alloc([clawback.memo(klvm)])
      ),
    ])
  );

  const clawbackCoin = new Coin(alice.coin.coinId(), clawback.puzzleHash(), 1n);

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);
  sim.passTime(10n);

  const clawbackSpend = clawback.pushThroughSpend(klvm);
  klvm.spendCoin(clawbackCoin, clawbackSpend);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});
