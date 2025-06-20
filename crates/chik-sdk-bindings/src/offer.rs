use bindy::Result;
use chik_protocol::SpendBundle;
use chik_sdk_driver::Offer;

pub fn encode_offer(spend_bundle: SpendBundle) -> Result<String> {
    Ok(Offer::from(spend_bundle).encode()?)
}

pub fn decode_offer(offer: String) -> Result<SpendBundle> {
    Ok(Offer::decode(&offer)?.into())
}
