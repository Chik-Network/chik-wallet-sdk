use std::str::FromStr;

use chia_protocol::{Bytes, Bytes32, BytesImpl, Program};
use js_sys::wasm_bindgen::{JsCast, UnwrapThrowExt};
use js_sys::Uint8Array;
use num_bigint::BigInt;

use crate::{Error, FromRust, IntoRust, Result};

pub struct WasmContext;

impl<T> FromRust<BigInt, T> for js_sys::BigInt {
    fn from_rust(value: BigInt, _context: &T) -> Result<Self> {
        Ok(js_sys::BigInt::from_str(&value.to_string()).map_err(Error::Js)?)
    }
}

impl<T> IntoRust<BigInt, T> for js_sys::BigInt {
    fn into_rust(self, _context: &T) -> Result<BigInt> {
        Ok(String::from(self.to_string(10).map_err(Error::Range)?).parse()?)
    }
}

impl<T, const N: usize> FromRust<BytesImpl<N>, T> for Vec<u8> {
    fn from_rust(value: BytesImpl<N>, _context: &T) -> Result<Self> {
        Ok(value.to_vec())
    }
}

impl<T, const N: usize> IntoRust<BytesImpl<N>, T> for Vec<u8> {
    fn into_rust(self, _context: &T) -> Result<BytesImpl<N>> {
        let bytes = self.to_vec();

        if bytes.len() != N {
            return Err(Error::WrongLength {
                expected: N,
                found: bytes.len(),
            });
        }

        Ok(bytes.try_into().unwrap())
    }
}

impl<T> FromRust<Bytes, T> for Vec<u8> {
    fn from_rust(value: Bytes, _context: &T) -> Result<Self> {
        Ok(value.to_vec())
    }
}

impl<T> IntoRust<Bytes, T> for Vec<u8> {
    fn into_rust(self, _context: &T) -> Result<Bytes> {
        Ok(self.to_vec().into())
    }
}

impl<T> FromRust<Program, T> for Vec<u8> {
    fn from_rust(value: Program, _context: &T) -> Result<Self> {
        Ok(value.to_vec())
    }
}

impl<T> IntoRust<Program, T> for Vec<u8> {
    fn into_rust(self, _context: &T) -> Result<Program> {
        Ok(self.to_vec().into())
    }
}

impl<T> IntoRust<u64, T> for js_sys::BigInt {
    fn into_rust(self, _context: &T) -> Result<u64> {
        let bigint: BigInt = self.into_rust(_context)?;
        Ok(bigint.try_into()?)
    }
}

impl<T> FromRust<u64, T> for js_sys::BigInt {
    fn from_rust(value: u64, _context: &T) -> Result<Self> {
        Ok(value.into())
    }
}

impl<T> IntoRust<Vec<Bytes32>, T> for js_sys::Array {
    fn into_rust(self, context: &T) -> Result<Vec<Bytes32>> {
        let bytes_array: Vec<Vec<u8>> = self
            .values()
            .into_iter()
            .map(|item| item.unwrap_throw().unchecked_ref::<Uint8Array>().to_vec())
            .collect();

        let mut bytes32_array = Vec::with_capacity(bytes_array.len());

        for bytes in bytes_array {
            bytes32_array.push(bytes.into_rust(context)?);
        }

        Ok(bytes32_array)
    }
}
