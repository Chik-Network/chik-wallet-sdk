use chia_sdk_bindings::{AddressInfo, Bytes, BytesImpl, Coin, CoinSpend, Error, Program, Result};

pub trait IntoRust<T> {
    fn rust(self) -> Result<T>;
}

pub trait IntoPy {
    type Py;

    fn py(self) -> Result<Self::Py>;
}

impl<const N: usize> IntoRust<BytesImpl<N>> for Vec<u8> {
    fn rust(self) -> Result<BytesImpl<N>> {
        if self.len() != N {
            return Err(Error::WrongLength {
                expected: N,
                found: self.len(),
            });
        }
        Ok(BytesImpl::new(self.try_into().unwrap()))
    }
}

impl<const N: usize> IntoPy for BytesImpl<N> {
    type Py = Vec<u8>;

    fn py(self) -> Result<Self::Py> {
        Ok(self.into())
    }
}

impl IntoRust<Bytes> for Vec<u8> {
    fn rust(self) -> Result<Bytes> {
        Ok(Bytes::new(self))
    }
}

impl IntoPy for Bytes {
    type Py = Vec<u8>;

    fn py(self) -> Result<Self::Py> {
        Ok(self.into())
    }
}

impl IntoRust<Program> for Vec<u8> {
    fn rust(self) -> Result<Program> {
        Ok(Program::from(self))
    }
}

impl IntoPy for Program {
    type Py = Vec<u8>;

    fn py(self) -> Result<Self::Py> {
        Ok(self.into())
    }
}

impl IntoRust<AddressInfo> for crate::AddressInfo {
    fn rust(self) -> Result<AddressInfo> {
        Ok(AddressInfo {
            puzzle_hash: self.puzzle_hash.rust()?,
            prefix: self.prefix,
        })
    }
}

impl IntoPy for AddressInfo {
    type Py = crate::AddressInfo;

    fn py(self) -> Result<Self::Py> {
        Ok(Self::Py {
            puzzle_hash: self.puzzle_hash.py()?,
            prefix: self.prefix,
        })
    }
}

impl IntoRust<CoinSpend> for crate::CoinSpend {
    fn rust(self) -> Result<CoinSpend> {
        Ok(CoinSpend {
            coin: self.coin.rust()?,
            puzzle_reveal: self.puzzle_reveal.rust()?,
            solution: self.solution.rust()?,
        })
    }
}

impl IntoPy for CoinSpend {
    type Py = crate::CoinSpend;

    fn py(self) -> Result<Self::Py> {
        Ok(Self::Py {
            coin: self.coin.py()?,
            puzzle_reveal: self.puzzle_reveal.py()?,
            solution: self.solution.py()?,
        })
    }
}

impl IntoRust<Coin> for crate::Coin {
    fn rust(self) -> Result<Coin> {
        Ok(Coin {
            parent_coin_info: self.parent_coin_info.rust()?,
            puzzle_hash: self.puzzle_hash.rust()?,
            amount: self.amount,
        })
    }
}

impl IntoPy for Coin {
    type Py = crate::Coin;

    fn py(self) -> Result<Self::Py> {
        Ok(Self::Py {
            parent_coin_info: self.parent_coin_info.py()?,
            puzzle_hash: self.puzzle_hash.py()?,
            amount: self.amount,
        })
    }
}
