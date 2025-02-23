/* auto-generated by NAPI-RS */
/* eslint-disable */
export declare class Address {
  encode(): string
  static decode(address: string): Address
  constructor(puzzleHash: Uint8Array, prefix: string)
  get puzzleHash(): Uint8Array
  set puzzleHash(value: Uint8Array)
  get prefix(): string
  set prefix(value: string)
}

export declare class Coin {
  coinId(): Uint8Array
  constructor(parentCoinInfo: Uint8Array, puzzleHash: Uint8Array, amount: bigint)
  get parentCoinInfo(): Uint8Array
  set parentCoinInfo(value: Uint8Array)
  get puzzleHash(): Uint8Array
  set puzzleHash(value: Uint8Array)
  get amount(): bigint
  set amount(value: bigint)
}

export declare class CoinSpend {
  constructor(coin: Coin, puzzleReveal: Uint8Array, solution: Uint8Array)
  get coin(): Coin
  set coin(value: Coin)
  get puzzleReveal(): Uint8Array
  set puzzleReveal(value: Uint8Array)
  get solution(): Uint8Array
  set solution(value: Uint8Array)
}

export declare class K1PublicKey {
  static fromBytes(bytes: Uint8Array): K1PublicKey
  toBytes(): Uint8Array
  fingerprint(): number
  verifyPrehashed(prehashed: Uint8Array, signature: K1Signature): boolean
}

export declare class K1SecretKey {
  static fromBytes(bytes: Uint8Array): K1SecretKey
  toBytes(): Uint8Array
  publicKey(): K1PublicKey
  signPrehashed(prehashed: Uint8Array): K1Signature
}

export declare class K1Signature {
  static fromBytes(bytes: Uint8Array): K1Signature
  toBytes(): Uint8Array
}

export declare class Mnemonic {
  constructor(mnemonic: string)
  static fromEntropy(entropy: Uint8Array): Mnemonic
  static generate(use24: boolean): Mnemonic
  static verify(mnemonic: string): boolean
  toString(): string
  toEntropy(): Uint8Array
  toSeed(password: string): Uint8Array
}

export declare class PublicKey {
  static infinity(): PublicKey
  static aggregate(publicKeys: Array<PublicKey>): PublicKey
  static fromBytes(bytes: Uint8Array): PublicKey
  toBytes(): Uint8Array
  fingerprint(): number
  isInfinity(): boolean
  isValid(): boolean
  deriveUnhardened(index: number): PublicKey
  deriveUnhardenedPath(path: Array<number>): PublicKey
  deriveSynthetic(): PublicKey
  deriveSyntheticHidden(hiddenPuzzleHash: Uint8Array): PublicKey
}

export declare class R1PublicKey {
  static fromBytes(bytes: Uint8Array): R1PublicKey
  toBytes(): Uint8Array
  fingerprint(): number
  verifyPrehashed(prehashed: Uint8Array, signature: R1Signature): boolean
}

export declare class R1SecretKey {
  static fromBytes(bytes: Uint8Array): R1SecretKey
  toBytes(): Uint8Array
  publicKey(): R1PublicKey
  signPrehashed(prehashed: Uint8Array): R1Signature
}

export declare class R1Signature {
  static fromBytes(bytes: Uint8Array): R1Signature
  toBytes(): Uint8Array
}

export declare class SecretKey {
  static fromSeed(seed: Uint8Array): SecretKey
  static fromBytes(bytes: Uint8Array): SecretKey
  toBytes(): Uint8Array
  publicKey(): PublicKey
  sign(message: Uint8Array): Signature
  deriveUnhardened(index: number): SecretKey
  deriveHardened(index: number): SecretKey
  deriveUnhardenedPath(path: Array<number>): SecretKey
  deriveHardenedPath(path: Array<number>): SecretKey
  deriveSynthetic(): SecretKey
  deriveSyntheticHidden(hiddenPuzzleHash: Uint8Array): SecretKey
}

export declare class Signature {
  static infinity(): Signature
  static aggregate(signatures: Array<Signature>): Signature
  static fromBytes(bytes: Uint8Array): Signature
  toBytes(): Uint8Array
  isInfinity(): boolean
  isValid(): boolean
}

export declare class SpendBundle {
  constructor(coinSpends: Array<CoinSpend>, aggregatedSignature: Signature)
  get coinSpends(): Array<CoinSpend>
  set coinSpends(value: Array<CoinSpend>)
  get aggregatedSignature(): Signature
  set aggregatedSignature(value: Signature)
}

export declare function bytesEqual(lhs: Uint8Array, rhs: Uint8Array): boolean

export declare function curryTreeHash(program: Uint8Array, args: Array<Uint8Array>): Uint8Array

export declare function fromHex(value: string): Uint8Array

export declare function generateBytes(bytes: number): Uint8Array

export declare function sha256(value: Uint8Array): Uint8Array

export declare function toHex(value: Uint8Array): string

export declare function treeHashAtom(atom: Uint8Array): Uint8Array

export declare function treeHashPair(first: Uint8Array, rest: Uint8Array): Uint8Array
