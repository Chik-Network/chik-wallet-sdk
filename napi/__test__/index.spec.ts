import test from "ava";

import {
  KlvmAllocator,
  compareBytes,
  curryTreeHash,
  fromHex,
  PublicKey,
  Simulator,
  toCoinId,
  toHex,
} from "../index.js";

test("calculate coin id", (t) => {
  const coinId = toCoinId({
    parentCoinInfo: fromHex(
      "4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a",
    ),
    puzzleHash: fromHex(
      "dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986",
    ),
    amount: 100n,
  });

  t.true(
    compareBytes(
      coinId,
      fromHex(
        "fd3e669c27be9d634fe79f1f7d7d8aaacc3597b855cffea1d708f4642f1d542a",
      ),
    ),
  );
});

test("byte equality", (t) => {
  const a = Uint8Array.from([1, 2, 3]);
  const b = Uint8Array.from([1, 2, 3]);

  t.true(compareBytes(a, b));
  t.true(Buffer.from(a).equals(b));
});

test("byte inequality", (t) => {
  const a = Uint8Array.from([1, 2, 3]);
  const b = Uint8Array.from([1, 2, 4]);

  t.true(!compareBytes(a, b));
  t.true(!Buffer.from(a).equals(b));
});

test("atom roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const expected = Uint8Array.from([1, 2, 3]);
  const atom = klvm.alloc(expected);

  t.true(compareBytes(atom.toAtom()!, expected));
});

test("string roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const expected = "hello world";
  const atom = klvm.alloc(expected);
  t.is(atom.toString(), expected);
});

test("number roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  for (const expected of [
    Number.MIN_SAFE_INTEGER,
    -1000,
    0,
    34,
    1000,
    Number.MAX_SAFE_INTEGER,
  ]) {
    const num = klvm.alloc(expected);
    t.is(num.toBigInt(), BigInt(expected));
  }
});

test("invalid number", (t) => {
  const klvm = new KlvmAllocator();

  for (const expected of [
    Number.MIN_SAFE_INTEGER - 1,
    Number.MAX_SAFE_INTEGER + 1,
    Infinity,
    -Infinity,
    NaN,
  ]) {
    t.throws(() => klvm.alloc(expected));
  }
});

test("bigint roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  for (const expected of [
    0n,
    1n,
    420n,
    67108863n,
    -1n,
    -100n,
    -421489719874198729487129847n,
    4384723984791283749823764732649187498237483927482n,
  ]) {
    const num = klvm.alloc(expected);
    t.is(num.toBigInt(), expected);
  }
});

test("pair roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const ptr = klvm.pair(1, 100n);
  const [first, rest] = ptr.toPair()!;

  t.is(first.toSmallNumber(), 1);
  t.is(rest.toBigInt(), 100n);
});

test("list roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.alloc(items);
  const list = ptr.toList().map((ptr) => ptr.toSmallNumber());

  t.deepEqual(list, items);
});

test("klvm value allocation", (t) => {
  const klvm = new KlvmAllocator();

  const shared = klvm.alloc(42);

  const manual = klvm.alloc([
    klvm.alloc(42),
    klvm.alloc("Hello, world!"),
    klvm.alloc(true),
    klvm.alloc(Uint8Array.from([1, 2, 3])),
    klvm.alloc([klvm.alloc(34)]),
    klvm.alloc(100n),
    shared,
  ]);

  const auto = klvm.alloc([
    42,
    "Hello, world!",
    true,
    Uint8Array.from([1, 2, 3]),
    [34],
    100n,
    shared,
  ]);

  t.true(compareBytes(klvm.treeHash(manual), klvm.treeHash(auto)));
});

test("public key roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const ptr = klvm.alloc(PublicKey.empty());
  const pk = ptr.toPublicKey()!;

  t.true(compareBytes(PublicKey.empty().toBytes(), pk.toBytes()));
});

test("curry add function", (t) => {
  const klvm = new KlvmAllocator();

  const addMod = klvm.deserialize(fromHex("ff10ff02ff0580"));
  const addToTen = klvm.curry(addMod, [klvm.alloc(10)]);
  const result = klvm.run(addToTen, klvm.alloc([5]), 10000000n, true);

  t.is(result.value.toSmallNumber(), 15);
  t.is(result.cost, 1082n);
});

test("curry roundtrip", (t) => {
  const klvm = new KlvmAllocator();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.curry(
    klvm.nil(),
    items.map((i) => klvm.alloc(i)),
  );
  const uncurry = ptr.uncurry()!;
  const args = uncurry.args.map((ptr) => ptr.toSmallNumber());

  t.true(
    compareBytes(klvm.treeHash(klvm.nil()), klvm.treeHash(uncurry.program)),
  );
  t.deepEqual(args, items);
});

test("klvm serialization", (t) => {
  const klvm = new KlvmAllocator();

  for (const [ptr, hex] of [
    [klvm.alloc(Uint8Array.from([1, 2, 3])), "83010203"],
    [klvm.alloc(420), "8201a4"],
    [klvm.alloc(100n), "64"],
    [klvm.pair(Uint8Array.from([1, 2, 3]), 100n), "ff8301020364"],
  ] as const) {
    const serialized = ptr.serialize();
    const deserialized = klvm.deserialize(serialized);

    t.true(compareBytes(klvm.treeHash(ptr), klvm.treeHash(deserialized)));
    t.is(hex as string, toHex(serialized));
  }
});

test("curry tree hash", (t) => {
  const klvm = new KlvmAllocator();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.curry(
    klvm.nil(),
    items.map((i) => klvm.alloc(i)),
  );

  const treeHash = curryTreeHash(
    klvm.treeHash(klvm.nil()),
    items.map((i) => klvm.treeHash(klvm.alloc(i))),
  );
  const expected = klvm.treeHash(ptr);

  t.true(compareBytes(treeHash, expected));
});

test("mint and spend nft", (t) => {
  const klvm = new KlvmAllocator();
  const simulator = new Simulator();
  const p2 = simulator.newP2(1n);

  const result = klvm.mintNfts(toCoinId(p2.coin), [
    {
      metadata: {
        dataUris: ["https://example.com"],
        metadataUris: ["https://example.com"],
        licenseUris: ["https://example.com"],
        editionNumber: 1n,
        editionTotal: 1n,
      },
      p2PuzzleHash: p2.puzzleHash,
      royaltyPuzzleHash: p2.puzzleHash,
      royaltyTenThousandths: 300,
    },
  ]);

  const spend = klvm.spendP2Standard(
    p2.publicKey,
    klvm.delegatedSpendForConditions(result.parentConditions),
  );

  simulator.spend(
    result.coinSpends.concat([
      {
        coin: p2.coin,
        puzzleReveal: spend.puzzle.serialize(),
        solution: spend.solution.serialize(),
      },
    ]),
    [p2.secretKey],
  );

  const innerSpend = klvm.spendP2Standard(
    p2.publicKey,
    klvm.delegatedSpendForConditions([
      klvm.createCoin(p2.puzzleHash, 1n, klvm.alloc([p2.puzzleHash])),
    ]),
  );

  const coinSpends = klvm.spendNft(result.nfts[0], innerSpend);

  simulator.spend(coinSpends, [p2.secretKey]);

  t.true(
    compareBytes(
      klvm
        .nftMetadata(
          klvm.parseNftMetadata(klvm.deserialize(result.nfts[0].info.metadata)),
        )
        .serialize(),
      result.nfts[0].info.metadata,
    ),
  );
});

test("create and parse condition", (t) => {
  const klvm = new KlvmAllocator();

  const puzzleHash = fromHex("ff".repeat(32));

  const condition = klvm.createCoin(puzzleHash, 1n, klvm.alloc([puzzleHash]));
  const parsed = klvm.parseCreateCoin(condition);

  t.true(parsed !== null && compareBytes(parsed.puzzleHash, puzzleHash));
  t.true(parsed !== null && parsed.amount === 1n);

  t.deepEqual(
    parsed?.memos
      ?.toList()
      .map((memo) => memo.toAtom())
      .filter((memo) => memo !== null),
    [puzzleHash],
  );
});
