/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function mnemonicFromEntropy(entropy: Uint8Array): string
export declare function mnemonicToEntropy(mnemonic: string): Uint8Array
export declare function verifyMnemonic(mnemonic: string): boolean
export declare function randomBytes(bytes: number): Uint8Array
export declare function generateMnemonic(use24: boolean): string
export declare function mnemonicToSeed(mnemonic: string, password: string): Uint8Array
export interface Output {
  value: Program
  cost: bigint
}
export declare function curryTreeHash(treeHash: Uint8Array, args: Array<Uint8Array>): Uint8Array
export declare function intToSignedBytes(bigInt: bigint): Uint8Array
export declare function signedBytesToInt(bytes: Uint8Array): bigint
export interface Remark {
  rest: Program
}
export interface AggSigParent {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigPuzzle {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigAmount {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigPuzzleAmount {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigParentAmount {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigParentPuzzle {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigUnsafe {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface AggSigMe {
  publicKey: Uint8Array
  message: Uint8Array
}
export interface CreateCoin {
  puzzleHash: Uint8Array
  amount: bigint
  memos: Array<Uint8Array>
}
export interface ReserveFee {
  amount: bigint
}
export interface CreateCoinAnnouncement {
  message: Uint8Array
}
export interface CreatePuzzleAnnouncement {
  message: Uint8Array
}
export interface AssertCoinAnnouncement {
  announcementId: Uint8Array
}
export interface AssertPuzzleAnnouncement {
  announcementId: Uint8Array
}
export interface AssertConcurrentSpend {
  coinId: Uint8Array
}
export interface AssertConcurrentPuzzle {
  puzzleHash: Uint8Array
}
export interface AssertSecondsRelative {
  seconds: bigint
}
export interface AssertSecondsAbsolute {
  seconds: bigint
}
export interface AssertHeightRelative {
  height: number
}
export interface AssertHeightAbsolute {
  height: number
}
export interface AssertBeforeSecondsRelative {
  seconds: bigint
}
export interface AssertBeforeSecondsAbsolute {
  seconds: bigint
}
export interface AssertBeforeHeightRelative {
  height: number
}
export interface AssertBeforeHeightAbsolute {
  height: number
}
export interface AssertMyCoinId {
  coinId: Uint8Array
}
export interface AssertMyParentId {
  parentId: Uint8Array
}
export interface AssertMyPuzzleHash {
  puzzleHash: Uint8Array
}
export interface AssertMyAmount {
  amount: bigint
}
export interface AssertMyBirthSeconds {
  seconds: bigint
}
export interface AssertMyBirthHeight {
  height: number
}
export interface AssertEphemeral {
  
}
export interface SendMessage {
  mode: number
  message: Uint8Array
  data: Array<Program>
}
export interface ReceiveMessage {
  mode: number
  message: Uint8Array
  data: Array<Program>
}
export interface Softfork {
  cost: bigint
  rest: Program
}
export interface Coin {
  parentCoinInfo: Uint8Array
  puzzleHash: Uint8Array
  amount: bigint
}
export declare function toCoinId(coin: Coin): Uint8Array
export interface CoinSpend {
  coin: Coin
  puzzleReveal: Uint8Array
  solution: Uint8Array
}
export interface Spend {
  puzzle: Program
  solution: Program
}
export interface CoinState {
  coin: Coin
  spentHeight?: number
  createdHeight?: number
}
export interface LineageProof {
  parentParentCoinInfo: Uint8Array
  parentInnerPuzzleHash?: Uint8Array
  parentAmount: bigint
}
export interface Nft {
  coin: Coin
  lineageProof: LineageProof
  info: NftInfo
}
export interface NftInfo {
  launcherId: Uint8Array
  metadata: Uint8Array
  metadataUpdaterPuzzleHash: Uint8Array
  currentOwner?: Uint8Array
  royaltyPuzzleHash: Uint8Array
  royaltyTenThousandths: number
  p2PuzzleHash: Uint8Array
}
export interface NftMetadata {
  editionNumber: bigint
  editionTotal: bigint
  dataUris: Array<string>
  dataHash?: Uint8Array
  metadataUris: Array<string>
  metadataHash?: Uint8Array
  licenseUris: Array<string>
  licenseHash?: Uint8Array
}
export interface ParsedNft {
  info: NftInfo
  innerPuzzle: Program
}
export interface NftMint {
  metadata: NftMetadata
  p2PuzzleHash: Uint8Array
  royaltyPuzzleHash: Uint8Array
  royaltyTenThousandths: number
}
export interface MintedNfts {
  nfts: Array<Nft>
  coinSpends: Array<CoinSpend>
  parentConditions: Array<Program>
}
export interface Curry {
  program: Program
  args: Array<Program>
}
export interface P2Coin {
  coin: Coin
  puzzleHash: Uint8Array
  publicKey: Uint8Array
  secretKey: Uint8Array
}
export declare function compareBytes(a: Uint8Array, b: Uint8Array): boolean
export declare function sha256(bytes: Uint8Array): Uint8Array
export declare function fromHexRaw(hex: string): Uint8Array
export declare function fromHex(hex: string): Uint8Array
export declare function toHex(bytes: Uint8Array): string
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
  deriveUnhardenedWalletIntermediate(): SecretKey
  deriveHardenedWalletIntermediate(): SecretKey
  deriveUnhardenedWallet(index: number): SecretKey
  deriveHardenedWallet(index: number): SecretKey
  deriveSynthetic(): SecretKey
  deriveSyntheticWithHiddenPuzzle(hiddenPuzzleHash: Uint8Array): SecretKey
}
export declare class PublicKey {
  static fromBytes(bytes: Uint8Array): PublicKey
  toBytes(): Uint8Array
  static empty(): PublicKey
  static aggregate(publicKeys: Array<PublicKey>): PublicKey
  fingerprint(): number
  isInfinity(): boolean
  isValid(): boolean
  verify(message: Uint8Array, signature: Signature): boolean
  deriveUnhardened(index: number): PublicKey
  deriveUnhardenedPath(path: Array<number>): PublicKey
  deriveUnhardenedWalletIntermediate(): PublicKey
  deriveUnhardenedWallet(index: number): PublicKey
  deriveSynthetic(): PublicKey
  deriveSyntheticWithHiddenPuzzle(hiddenPuzzleHash: Uint8Array): PublicKey
}
export declare class Signature {
  static fromBytes(bytes: Uint8Array): Signature
  toBytes(): Uint8Array
  static empty(): Signature
  static aggregate(signatures: Array<Signature>): Signature
  isValid(): boolean
}
export declare class ClvmAllocator {
  constructor()
  nil(): Program
  deserialize(value: Uint8Array): Program
  deserializeWithBackrefs(value: Uint8Array): Program
  treeHash(program: Program): Uint8Array
  run(puzzle: Program, solution: Program, maxCost: bigint, mempoolMode: boolean): Output
  curry(program: Program, args: Array<Program>): Program
  pair(first: ClvmValue, rest: ClvmValue): Program
  alloc(value: ClvmValue): Program
  nftMetadata(value: NftMetadata): Program
  parseNftMetadata(value: Program): NftMetadata
  delegatedSpendForConditions(conditions: Array<Program>): Spend
  spendP2Standard(syntheticKey: Uint8Array, delegatedSpend: Spend): Spend
  spendP2DelegatedSingleton(launcherId: Uint8Array, coinId: Uint8Array, singletonInnerPuzzleHash: Uint8Array, delegatedSpend: Spend): Spend
  mintNfts(parent_coin_id: Uint8Array, nft_mints: Array<NftMint>): MintedNfts
  parseNftInfo(puzzle: Program): ParsedNft | null
  parseChildNft(parentCoin: Coin, parentPuzzle: Program, parentSolution: Program): Nft | null
  spendNft(nft: Nft, innerSpend: Spend): Array<CoinSpend>
  remark(rest: Program): Program
  parseRemark(program: Program): Remark | null
  aggSigParent(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigParent(program: Program): AggSigParent | null
  aggSigPuzzle(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigPuzzle(program: Program): AggSigPuzzle | null
  aggSigAmount(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigAmount(program: Program): AggSigAmount | null
  aggSigPuzzleAmount(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigPuzzleAmount(program: Program): AggSigPuzzleAmount | null
  aggSigParentAmount(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigParentAmount(program: Program): AggSigParentAmount | null
  aggSigParentPuzzle(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigParentPuzzle(program: Program): AggSigParentPuzzle | null
  aggSigUnsafe(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigUnsafe(program: Program): AggSigUnsafe | null
  aggSigMe(publicKey: Uint8Array, message: Uint8Array): Program
  parseAggSigMe(program: Program): AggSigMe | null
  createCoin(puzzleHash: Uint8Array, amount: bigint, memos: Array<Uint8Array>): Program
  parseCreateCoin(program: Program): CreateCoin | null
  reserveFee(amount: bigint): Program
  parseReserveFee(program: Program): ReserveFee | null
  createCoinAnnouncement(message: Uint8Array): Program
  parseCreateCoinAnnouncement(program: Program): CreateCoinAnnouncement | null
  createPuzzleAnnouncement(message: Uint8Array): Program
  parseCreatePuzzleAnnouncement(program: Program): CreatePuzzleAnnouncement | null
  assertCoinAnnouncement(announcementId: Uint8Array): Program
  parseAssertCoinAnnouncement(program: Program): AssertCoinAnnouncement | null
  assertPuzzleAnnouncement(announcementId: Uint8Array): Program
  parseAssertPuzzleAnnouncement(program: Program): AssertPuzzleAnnouncement | null
  assertConcurrentSpend(coinId: Uint8Array): Program
  parseAssertConcurrentSpend(program: Program): AssertConcurrentSpend | null
  assertConcurrentPuzzle(puzzleHash: Uint8Array): Program
  parseAssertConcurrentPuzzle(program: Program): AssertConcurrentPuzzle | null
  assertSecondsRelative(seconds: bigint): Program
  parseAssertSecondsRelative(program: Program): AssertSecondsRelative | null
  assertSecondsAbsolute(seconds: bigint): Program
  parseAssertSecondsAbsolute(program: Program): AssertSecondsAbsolute | null
  assertHeightRelative(height: number): Program
  parseAssertHeightRelative(program: Program): AssertHeightRelative | null
  assertHeightAbsolute(height: number): Program
  parseAssertHeightAbsolute(program: Program): AssertHeightAbsolute | null
  assertBeforeSecondsRelative(seconds: bigint): Program
  parseAssertBeforeSecondsRelative(program: Program): AssertBeforeSecondsRelative | null
  assertBeforeSecondsAbsolute(seconds: bigint): Program
  parseAssertBeforeSecondsAbsolute(program: Program): AssertBeforeSecondsAbsolute | null
  assertBeforeHeightRelative(height: number): Program
  parseAssertBeforeHeightRelative(program: Program): AssertBeforeHeightRelative | null
  assertBeforeHeightAbsolute(height: number): Program
  parseAssertBeforeHeightAbsolute(program: Program): AssertBeforeHeightAbsolute | null
  assertMyCoinId(coinId: Uint8Array): Program
  parseAssertMyCoinId(program: Program): AssertMyCoinId | null
  assertMyParentId(parentId: Uint8Array): Program
  parseAssertMyParentId(program: Program): AssertMyParentId | null
  assertMyPuzzleHash(puzzleHash: Uint8Array): Program
  parseAssertMyPuzzleHash(program: Program): AssertMyPuzzleHash | null
  assertMyAmount(amount: bigint): Program
  parseAssertMyAmount(program: Program): AssertMyAmount | null
  assertMyBirthSeconds(seconds: bigint): Program
  parseAssertMyBirthSeconds(program: Program): AssertMyBirthSeconds | null
  assertMyBirthHeight(height: number): Program
  parseAssertMyBirthHeight(program: Program): AssertMyBirthHeight | null
  assertEphemeral(): Program
  parseAssertEphemeral(program: Program): AssertEphemeral | null
  sendMessage(mode: number, message: Uint8Array, data: Array<Program>): Program
  parseSendMessage(program: Program): SendMessage | null
  receiveMessage(mode: number, message: Uint8Array, data: Array<Program>): Program
  parseReceiveMessage(program: Program): ReceiveMessage | null
  softfork(cost: bigint, rest: Program): Program
  parseSoftfork(program: Program): Softfork | null
}
export declare class Tls {
  constructor(certPath: string, keyPath: string)
}
export declare class Peer {
  static connect(uri: string, tls: Tls, networkId: string): Promise<Peer>
  requestChildren(coinId: Uint8Array): Promise<Array<CoinState>>
  close(): Promise<void>
}
export declare class Program {
  isAtom(): boolean
  isPair(): boolean
  treeHash(): Uint8Array
  serialize(): Uint8Array
  serializeWithBackrefs(): Uint8Array
  length(): number | null
  toAtom(): Uint8Array | null
  toPair(): [Program, Program] | null
  get first(): Program
  get rest(): Program
  toList(): Array<Program>
  uncurry(): Curry | null
  toString(): string | null
  toSmallNumber(): number | null
  toBigInt(): bigint | null
}
export declare class Simulator {
  constructor()
  newCoin(puzzleHash: Uint8Array, amount: bigint): Coin
  newP2(amount: bigint): P2Coin
  spend(coinSpends: Array<CoinSpend>, secretKeys: Array<Uint8Array>): void
}

/* auto-generated by `pnpm run update-declarations` */

export type ClvmValue = number | bigint | string | boolean | Program | Uint8Array | ClvmValue[];
