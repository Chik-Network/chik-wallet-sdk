use bindy::Result;
use chik_protocol::{Bytes32, Program};

use crate::Signature;

#[derive(Clone, Copy)]
pub struct Coin {
    pub parent_coin_info: Bytes32,
    pub puzzle_hash: Bytes32,
    pub amount: u64,
}

impl Coin {
    pub fn coin_id(&self) -> Result<Bytes32> {
        Ok(self.rs().coin_id())
    }

    pub(crate) fn rs(self) -> chik_protocol::Coin {
        chik_protocol::Coin::new(self.parent_coin_info, self.puzzle_hash, self.amount)
    }
}

impl From<chik_protocol::Coin> for Coin {
    fn from(value: chik_protocol::Coin) -> Self {
        Self {
            parent_coin_info: value.parent_coin_info,
            puzzle_hash: value.puzzle_hash,
            amount: value.amount,
        }
    }
}

impl From<Coin> for chik_protocol::Coin {
    fn from(value: Coin) -> Self {
        value.rs()
    }
}

#[derive(Clone)]
pub struct CoinSpend {
    pub coin: Coin,
    pub puzzle_reveal: Program,
    pub solution: Program,
}

impl CoinSpend {
    pub(crate) fn rs(self) -> chik_protocol::CoinSpend {
        chik_protocol::CoinSpend::new(
            self.coin.rs(),
            self.puzzle_reveal.clone(),
            self.solution.clone(),
        )
    }
}

impl From<chik_protocol::CoinSpend> for CoinSpend {
    fn from(value: chik_protocol::CoinSpend) -> Self {
        Self {
            coin: value.coin.into(),
            puzzle_reveal: value.puzzle_reveal,
            solution: value.solution,
        }
    }
}

impl From<CoinSpend> for chik_protocol::CoinSpend {
    fn from(value: CoinSpend) -> Self {
        value.rs()
    }
}

#[derive(Clone)]
pub struct SpendBundle {
    pub coin_spends: Vec<CoinSpend>,
    pub aggregated_signature: Signature,
}

impl SpendBundle {
    pub(crate) fn rs(self) -> chik_protocol::SpendBundle {
        chik_protocol::SpendBundle::new(
            self.coin_spends.into_iter().map(CoinSpend::rs).collect(),
            self.aggregated_signature.0,
        )
    }
}

impl From<chik_protocol::SpendBundle> for SpendBundle {
    fn from(value: chik_protocol::SpendBundle) -> Self {
        Self {
            coin_spends: value.coin_spends.into_iter().map(CoinSpend::from).collect(),
            aggregated_signature: Signature(value.aggregated_signature),
        }
    }
}

impl From<SpendBundle> for chik_protocol::SpendBundle {
    fn from(value: SpendBundle) -> Self {
        value.rs()
    }
}
