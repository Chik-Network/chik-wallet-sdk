use chik_sdk_utils::Address;
use hex_literal::hex;

fn main() -> anyhow::Result<()> {
    let puzzle_hash =
        hex!("aca490e9f3ebcafa3d5342d347db2703b31029511f5b40c11441af1c961f6585").into();

    let address = Address::new(puzzle_hash, "xck".to_string()).encode()?;

    println!("XCK address: {address}");

    let roundtrip = Address::decode(&address)?;
    println!(
        "Address matches puzzle hash: {}",
        roundtrip
            == Address {
                puzzle_hash,
                prefix: "xck".to_string()
            }
    );

    Ok(())
}
