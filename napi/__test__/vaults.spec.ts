import test from "ava";

import {
  blsMemberHash,
  Klvm,
  Coin,
  customMemberHash,
  force1Of2Restriction,
  k1MemberHash,
  K1Pair,
  K1SecretKey,
  K1Signature,
  MemberConfig,
  mOfNHash,
  passkeyMemberHash,
  preventSideEffectsRestriction,
  R1Pair,
  sha256,
  Simulator,
  singletonMemberHash,
  Spend,
  timelockRestriction,
  treeHashPair,
  Vault,
  wrappedDelegatedPuzzleHash,
} from "../index.js";

test("bls key vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = sim.bls(0n);

  const config = new MemberConfig().withTopLevel(true);

  const [vault, coin] = mintVaultWithCoin(
    sim,
    klvm,
    blsMemberHash(config, alice.pk),
    1n
  );

  const coinDelegatedSpend = klvm.delegatedSpend([klvm.reserveFee(1n)]);

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
    klvm.sendMessage(23, coinDelegatedSpend.puzzle.treeHash(), [
      klvm.alloc(coin.coinId()),
    ]),
  ]);

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.blsMember(config, alice.pk);
  mips.spendVault(vault);

  const p2Spend = klvm.mipsSpend(coin, coinDelegatedSpend);
  p2Spend.singletonMember(
    config,
    vault.launcherId,
    vault.custodyHash,
    vault.coin.amount
  );

  klvm.spendCoin(coin, p2Spend.spend(coin.puzzleHash));

  sim.spendCoins(klvm.coinSpends(), [alice.sk]);

  t.true(true);
});

test("single signer vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const k1 = K1Pair.fromSeed(1n);

  const config = new MemberConfig().withTopLevel(true);

  const vault = mintVault(sim, klvm, k1MemberHash(config, k1.pk, false));

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const signature = signK1(
    k1.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.k1Member(config, k1.pk, signature, false);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("passkey member vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const r1 = R1Pair.fromSeed(1n);

  const config = new MemberConfig().withTopLevel(true);

  const fastForward = false;

  const vault = mintVault(
    sim,
    klvm,
    passkeyMemberHash(config, r1.pk, fastForward)
  );

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const challengeIndex = 23;
  const originalMessage = Buffer.from(
    sha256(
      Buffer.concat([
        Buffer.from(delegatedSpend.puzzle.treeHash()),
        fastForward ? vault.coin.puzzleHash : vault.coin.coinId(),
      ])
    )
  );

  const authenticatorData = Buffer.from(
    "49960de5880e8c687434170f6476605b8fe4aeb9a28632c7995cf3ba831d97630500000009",
    "hex"
  );
  const clientDataJSON = Buffer.from(
    `{"type":"webauthn.get","challenge":"${originalMessage.toString(
      "base64url"
    )}","origin":"http://localhost:3000","crossOrigin":false}`,
    "utf-8"
  );
  // Reproduce web browser passkey behavior
  const message = sha256(
    Buffer.concat([authenticatorData, sha256(clientDataJSON)])
  );

  const signature = r1.sk.signPrehashed(message);

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.passkeyMember(
    config,
    r1.pk,
    signature,
    authenticatorData,
    clientDataJSON,
    challengeIndex,
    fastForward
  );
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("single signer fast forward vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const k1 = K1Pair.fromSeed(1n);

  const config = new MemberConfig().withTopLevel(true);

  const vault = mintVault(sim, klvm, k1MemberHash(config, k1.pk, true));

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const signature = signK1(
    k1.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    true
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.k1Member(config, k1.pk, signature, true);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("1 of 2 vault (path 1)", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = K1Pair.fromSeed(1n);
  const bob = K1Pair.fromSeed(2n);

  const config = new MemberConfig();

  const aliceHash = k1MemberHash(config, alice.pk, false);
  const bobHash = k1MemberHash(config, bob.pk, false);

  const vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 1, [aliceHash, bobHash])
  );

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const signature = signK1(
    alice.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 1, [aliceHash, bobHash]);
  mips.k1Member(config, alice.pk, signature, false);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("1 of 2 vault (path 2)", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = K1Pair.fromSeed(1n);
  const bob = K1Pair.fromSeed(2n);

  const config = new MemberConfig();

  const aliceHash = k1MemberHash(config, alice.pk, false);
  const bobHash = k1MemberHash(config, bob.pk, false);

  const vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 1, [aliceHash, bobHash])
  );

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const signature = signK1(
    bob.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 1, [aliceHash, bobHash]);
  mips.k1Member(config, bob.pk, signature, false);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("2 of 2 vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = K1Pair.fromSeed(1n);
  const bob = K1Pair.fromSeed(2n);

  const config = new MemberConfig();

  const aliceHash = k1MemberHash(config, alice.pk, false);
  const bobHash = k1MemberHash(config, bob.pk, false);

  const vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 2, [aliceHash, bobHash])
  );

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const aliceSignature = signK1(
    alice.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );
  const bobSignature = signK1(
    bob.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 2, [aliceHash, bobHash]);
  mips.k1Member(config, alice.pk, aliceSignature, false);
  mips.k1Member(config, bob.pk, bobSignature, false);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("2 of 3 vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = K1Pair.fromSeed(1n);
  const bob = K1Pair.fromSeed(2n);
  const charlie = K1Pair.fromSeed(3n);

  const config = new MemberConfig();

  const aliceHash = k1MemberHash(config, alice.pk, false);
  const bobHash = k1MemberHash(config, bob.pk, false);
  const charlieHash = k1MemberHash(config, charlie.pk, false);

  const vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 2, [aliceHash, bobHash, charlieHash])
  );

  const delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  const aliceSignature = signK1(
    alice.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );
  const bobSignature = signK1(
    bob.sk,
    vault,
    delegatedSpend.puzzle.treeHash(),
    false
  );

  const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 2, [aliceHash, bobHash, charlieHash]);
  mips.k1Member(config, alice.pk, aliceSignature, false);
  mips.k1Member(config, bob.pk, bobSignature, false);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

test("fast forward paths vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const alice = K1Pair.fromSeed(1n);
  const bob = K1Pair.fromSeed(2n);

  const config = new MemberConfig();

  const aliceRegularHash = k1MemberHash(config, alice.pk, false);
  const aliceFastForwardHash = k1MemberHash(config, alice.pk, true);
  const bobRegularHash = k1MemberHash(config, bob.pk, false);
  const bobFastForwardHash = k1MemberHash(config, bob.pk, true);

  const regularPathHash = mOfNHash(config, 1, [
    aliceRegularHash,
    bobRegularHash,
  ]);
  const fastForwardPathHash = mOfNHash(config, 1, [
    aliceFastForwardHash,
    bobFastForwardHash,
  ]);

  let vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 1, [
      regularPathHash,
      fastForwardPathHash,
    ])
  );

  for (const fastForward of [false, true, false, true]) {
    const delegatedSpend = klvm.delegatedSpend([
      klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
    ]);

    const aliceSignature = signK1(
      alice.sk,
      vault,
      delegatedSpend.puzzle.treeHash(),
      fastForward
    );

    const mips = klvm.mipsSpend(vault.coin, delegatedSpend);
    mips.mOfN(config.withTopLevel(true), 1, [
      regularPathHash,
      fastForwardPathHash,
    ]);
    mips.mOfN(
      config,
      1,
      fastForward
        ? [aliceFastForwardHash, bobFastForwardHash]
        : [aliceRegularHash, bobRegularHash]
    );
    mips.k1Member(config, alice.pk, aliceSignature, fastForward);
    mips.spendVault(vault);

    sim.spendCoins(klvm.coinSpends(), []);

    vault = vault.child(vault.custodyHash);
  }

  t.true(true);
});

test("single signer recovery vault", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const custodyKey = K1Pair.fromSeed(1n);
  const recoveryKey = K1Pair.fromSeed(2n);

  // Initial vault
  const config = new MemberConfig();

  const memberHash = k1MemberHash(config, custodyKey.pk, false);

  const timelock = timelockRestriction(1n);
  const recoveryRestrictions = [
    force1Of2Restriction(
      memberHash,
      0,
      treeHashPair(timelock.puzzleHash, klvm.nil().treeHash()),
      klvm.nil().treeHash()
    ),
    ...preventSideEffectsRestriction(),
  ];
  const initialRecoveryHash = k1MemberHash(
    config.withRestrictions(recoveryRestrictions),
    recoveryKey.pk,
    false
  );

  let vault = mintVault(
    sim,
    klvm,
    mOfNHash(config.withTopLevel(true), 1, [memberHash, initialRecoveryHash])
  );

  let delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);

  let mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 1, [memberHash, initialRecoveryHash]);
  mips.k1Member(
    config,
    custodyKey.pk,
    signK1(custodyKey.sk, vault, delegatedSpend.puzzle.treeHash(), false),
    false
  );
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  // Initiate recovery
  const oldCustodyHash = vault.custodyHash;
  const recoveryDelegatedSpend = new Spend(klvm.nil(), klvm.nil());

  const recoveryFinishMemberSpend = klvm.delegatedSpend([
    klvm.createCoin(oldCustodyHash, vault.coin.amount, null),
    klvm.assertSecondsRelative(1n),
  ]);
  const recoveryFinishMemberHash = customMemberHash(
    config.withRestrictions([timelock]),
    recoveryFinishMemberSpend.puzzle.treeHash()
  );

  const custodyHash = mOfNHash(config.withTopLevel(true), 1, [
    memberHash,
    recoveryFinishMemberHash,
  ]);

  delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(custodyHash, vault.coin.amount, null),
  ]);

  vault = vault.child(vault.custodyHash);
  mips = klvm.mipsSpend(vault.coin, delegatedSpend);

  mips.mOfN(config.withTopLevel(true), 1, [memberHash, initialRecoveryHash]);

  mips.k1Member(
    config.withRestrictions(recoveryRestrictions),
    recoveryKey.pk,
    signK1(
      recoveryKey.sk,
      vault,
      wrappedDelegatedPuzzleHash(
        recoveryRestrictions,
        delegatedSpend.puzzle.treeHash()
      ),
      false
    ),
    false
  );

  mips.preventSideEffects();

  mips.force1Of2RestrictedVariable(
    memberHash,
    0,
    treeHashPair(timelock.puzzleHash, klvm.nil().treeHash()),
    klvm.nil().treeHash(),
    recoveryFinishMemberSpend.puzzle.treeHash()
  );

  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  // Finish recovery
  vault = vault.child(custodyHash);
  mips = klvm.mipsSpend(vault.coin, recoveryDelegatedSpend);
  mips.mOfN(config.withTopLevel(true), 1, [
    memberHash,
    recoveryFinishMemberHash,
  ]);
  mips.customMember(
    config.withRestrictions([timelock]),
    recoveryFinishMemberSpend
  );
  mips.timelock(1n);
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  // Make sure the vault is spendable after recovery
  vault = vault.child(oldCustodyHash);
  delegatedSpend = klvm.delegatedSpend([
    klvm.createCoin(vault.custodyHash, vault.coin.amount, null),
  ]);
  mips = klvm.mipsSpend(vault.coin, delegatedSpend);
  mips.mOfN(config.withTopLevel(true), 1, [memberHash, initialRecoveryHash]);
  mips.k1Member(
    config,
    custodyKey.pk,
    signK1(custodyKey.sk, vault, delegatedSpend.puzzle.treeHash(), false),
    false
  );
  mips.spendVault(vault);

  sim.spendCoins(klvm.coinSpends(), []);

  t.true(true);
});

function mintVault(sim: Simulator, klvm: Klvm, custodyHash: Uint8Array): Vault {
  const p2 = sim.bls(1n);

  const { vault, parentConditions } = klvm.mintVault(
    p2.coin.coinId(),
    custodyHash,
    klvm.nil()
  );

  const spend = klvm.standardSpend(
    p2.pk,
    klvm.delegatedSpend(parentConditions)
  );

  klvm.spendCoin(p2.coin, spend);

  sim.spendCoins(klvm.coinSpends(), [p2.sk]);

  return vault;
}

test("non-vault MIPS spend", (t) => {
  const sim = new Simulator();
  const klvm = new Klvm();

  const p2 = sim.bls(1n);

  const config = new MemberConfig().withTopLevel(true);
  const puzzleHash = blsMemberHash(config, p2.pk);

  const spend1 = klvm.standardSpend(
    p2.pk,
    klvm.delegatedSpend([klvm.createCoin(puzzleHash, 1n, null)])
  );

  const coin: Coin = new Coin(p2.coin.coinId(), puzzleHash, 1n);

  const mipsSpend = klvm.mipsSpend(
    coin,
    klvm.delegatedSpend([klvm.createCoin(puzzleHash, 1n, null)])
  );

  mipsSpend.blsMember(config, p2.pk);
  const spend2 = mipsSpend.spend(puzzleHash);

  klvm.spendCoin(p2.coin, spend1);
  klvm.spendCoin(coin, spend2);

  sim.spendCoins(klvm.coinSpends(), [p2.sk]);

  t.true(true);
});

function mintVaultWithCoin(
  sim: Simulator,
  klvm: Klvm,
  custodyHash: Uint8Array,
  amount: bigint
): [Vault, Coin] {
  const p2 = sim.bls(amount + 1n);

  const { vault, parentConditions } = klvm.mintVault(
    p2.coin.coinId(),
    custodyHash,
    klvm.nil()
  );

  const p2PuzzleHash = singletonMemberHash(
    new MemberConfig().withTopLevel(true),
    vault.launcherId
  );

  const spend = klvm.standardSpend(
    p2.pk,
    klvm.delegatedSpend([
      ...parentConditions,
      klvm.createCoin(p2PuzzleHash, amount, klvm.alloc([vault.launcherId])),
    ])
  );

  klvm.spendCoin(p2.coin, spend);

  sim.spendCoins(klvm.coinSpends(), [p2.sk]);

  return [vault, new Coin(p2.coin.coinId(), p2PuzzleHash, amount)];
}

function signK1(
  sk: K1SecretKey,
  vault: Vault,
  delegatedPuzzleHash: Uint8Array,
  fastForward: boolean
): K1Signature {
  return sk.signPrehashed(
    sha256(
      Uint8Array.from([
        ...delegatedPuzzleHash,
        ...(fastForward ? vault.coin.puzzleHash : vault.coin.coinId()),
      ])
    )
  );
}
