import test from "ava";

import {
  bytesEqual,
  Klvm,
  Coin,
  Constants,
  curryTreeHash,
  fromHex,
  NftMetadata,
  NftMint,
  PublicKey,
  Simulator,
  toHex,
} from "../index.js";

test("calculate coin id", (t) => {
  const coinId = new Coin(
    fromHex("4bf5122f344554c53bde2ebb8cd2b7e3d1600ad631c385a5d7cce23c7785459a"),
    fromHex("dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986"),
    100n
  ).coinId();

  t.true(
    bytesEqual(
      coinId,
      fromHex(
        "fd3e669c27be9d634fe79f1f7d7d8aaacc3597b855cffea1d708f4642f1d542a"
      )
    )
  );
});

test("byte equality", (t) => {
  const a = Uint8Array.from([1, 2, 3]);
  const b = Uint8Array.from([1, 2, 3]);

  t.true(bytesEqual(a, b));
  t.true(Buffer.from(a).equals(b));
});

test("byte inequality", (t) => {
  const a = Uint8Array.from([1, 2, 3]);
  const b = Uint8Array.from([1, 2, 4]);

  t.true(!bytesEqual(a, b));
  t.true(!Buffer.from(a).equals(b));
});

test("atom roundtrip", (t) => {
  const klvm = new Klvm();

  const expected = Uint8Array.from([1, 2, 3]);
  const atom = klvm.alloc(expected);

  t.true(bytesEqual(atom.toAtom()!, expected));
});

test("string roundtrip", (t) => {
  const klvm = new Klvm();

  const expected = "hello world";
  const atom = klvm.alloc(expected);
  t.is(atom.toString(), expected);
});

test("number roundtrip", (t) => {
  const klvm = new Klvm();

  for (const expected of [
    Number.MIN_SAFE_INTEGER,
    -1000,
    0,
    34,
    1000,
    Number.MAX_SAFE_INTEGER,
  ]) {
    const num = klvm.alloc(expected);
    t.is(num.toInt(), BigInt(expected));
  }
});

test("invalid number", (t) => {
  const klvm = new Klvm();

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
  const klvm = new Klvm();

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
    t.is(num.toInt(), expected);
  }
});

test("pair roundtrip", (t) => {
  const klvm = new Klvm();

  const ptr = klvm.pair(klvm.boundCheckedNumber(1), klvm.int(100n));
  const { first, rest } = ptr.toPair()!;

  t.is(first.toBoundCheckedNumber(), 1);
  t.is(rest.toInt(), 100n);
});

test("list roundtrip", (t) => {
  const klvm = new Klvm();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.alloc(items);
  const list = ptr.toList()?.map((ptr) => ptr.toBoundCheckedNumber());

  t.deepEqual(list, items);
});

test("klvm value allocation", (t) => {
  const klvm = new Klvm();

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

  t.true(bytesEqual(manual.treeHash(), auto.treeHash()));
});

test("public key roundtrip", (t) => {
  const klvm = new Klvm();

  const ptr = klvm.alloc(PublicKey.infinity());
  const pk = PublicKey.fromBytes(ptr.toAtom()!);

  t.true(bytesEqual(PublicKey.infinity().toBytes(), pk.toBytes()));
});

test("curry add function", (t) => {
  const klvm = new Klvm();

  const addMod = klvm.deserialize(fromHex("ff10ff02ff0580"));
  const addToTen = addMod.curry([klvm.alloc(10)]);
  const result = addToTen.run(klvm.alloc([5]), 10000000n, true);

  t.is(result.value.toBoundCheckedNumber(), 15);
  t.is(result.cost, 1082n);
});

test("curry roundtrip", (t) => {
  const klvm = new Klvm();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.nil().curry(items.map((i) => klvm.alloc(i)));
  const uncurry = ptr.uncurry()!;
  const args = uncurry.args?.map((ptr) => ptr.toBoundCheckedNumber());

  t.true(bytesEqual(klvm.nil().treeHash(), uncurry.program.treeHash()));
  t.deepEqual(args, items);
});

test("klvm serialization", (t) => {
  const klvm = new Klvm();

  for (const [ptr, hex] of [
    [klvm.alloc(Uint8Array.from([1, 2, 3])), "83010203"],
    [klvm.alloc(420), "8201a4"],
    [klvm.alloc(100n), "64"],
    [
      klvm.pair(klvm.atom(Uint8Array.from([1, 2, 3])), klvm.int(100n)),
      "ff8301020364",
    ],
  ] as const) {
    const serialized = ptr.serialize();
    const deserialized = klvm.deserialize(serialized);

    t.true(bytesEqual(ptr.treeHash(), deserialized.treeHash()));
    t.is(hex as string, toHex(serialized));
  }
});

test("curry tree hash", (t) => {
  const klvm = new Klvm();

  const items = Array.from({ length: 10 }, (_, i) => i);
  const ptr = klvm.nil().curry(items.map((i) => klvm.alloc(i)));

  const treeHash = curryTreeHash(
    klvm.nil().treeHash(),
    items.map((i) => klvm.alloc(i).treeHash())
  );
  const expected = ptr.treeHash();

  t.true(bytesEqual(treeHash, expected));
});

test("mint and spend nft", (t) => {
  const klvm = new Klvm();
  const simulator = new Simulator();
  const alice = simulator.bls(1n);

  const metadata = new NftMetadata(
    1n,
    1n,
    ["https://example.com"],
    null,
    ["https://example.com"],
    null,
    ["https://example.com"],
    null
  );

  const result = klvm.mintNfts(alice.coin.coinId(), [
    new NftMint(
      klvm.nftMetadata(metadata),
      Constants.nftMetadataUpdaterDefaultHash(),
      alice.puzzleHash,
      alice.puzzleHash,
      300
    ),
  ]);

  const spend = klvm.standardSpend(
    alice.pk,
    klvm.delegatedSpend(result.parentConditions)
  );

  klvm.spendCoin(alice.coin, spend);

  simulator.spendCoins(klvm.coinSpends(), [alice.sk]);

  const innerSpend = klvm.standardSpend(
    alice.pk,
    klvm.delegatedSpend([
      klvm.createCoin(alice.puzzleHash, 1n, klvm.alloc([alice.puzzleHash])),
    ])
  );

  klvm.spendNft(result.nfts[0], innerSpend);

  simulator.spendCoins(klvm.coinSpends(), [alice.sk]);

  t.true(
    bytesEqual(
      klvm
        .nftMetadata(result.nfts[0].info.metadata.parseNftMetadata()!)
        .serialize(),
      result.nfts[0].info.metadata.serialize()
    )
  );
});

test("create and parse condition", (t) => {
  const klvm = new Klvm();

  const puzzleHash = fromHex("ff".repeat(32));

  const condition = klvm.createCoin(puzzleHash, 1n, klvm.alloc([puzzleHash]));
  const parsed = condition.parseCreateCoin();

  t.true(parsed !== null && bytesEqual(parsed.puzzleHash, puzzleHash));
  t.true(parsed !== null && parsed.amount === 1n);

  t.deepEqual(
    parsed?.memos
      ?.toList()
      ?.map((memo) => memo.toAtom())
      .filter((memo) => memo !== null),
    [puzzleHash]
  );
});
