use chia_sdk_utils::AddressInfo;
use hex_literal::hex;

fn main() -> anyhow::Result<()> {
    let puzzle_hash =
        hex!("aca490e9f3ebcafa3d5342d347db2703b31029511f5b40c11441af1c961f6585").into();

    let address = AddressInfo::new(puzzle_hash, "xch".to_string()).encode()?;

    println!("XCH address: {address}");

    let roundtrip = AddressInfo::decode(&address)?;
    println!(
        "Address matches puzzle hash: {}",
        roundtrip
            == AddressInfo {
                puzzle_hash,
                prefix: "xch".to_string()
            }
    );

    Ok(())
}
