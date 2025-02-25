const { readFileSync, writeFileSync } = require("fs");
const path = require("path");

const declarations = path.resolve(__dirname, "..", "index.d.ts");
const content = readFileSync(declarations, "utf8");

const header = "/* auto-generated by `pnpm run update-declarations` */";

if (content.includes(header)) {
  console.log("Declarations are already updated.");
  process.exit(0);
}

const lines = [
  "",
  header,
  "",
  "export type KlvmValue = number | bigint | string | boolean | Program | Uint8Array | PublicKey | Signature | KlvmValue[];",
  "",
];

writeFileSync(declarations, content + lines.join("\n"), "utf8");
