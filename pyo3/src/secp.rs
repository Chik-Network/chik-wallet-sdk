use pyo3::prelude::*;

use crate::traits::{IntoPy, IntoRust};

#[pyclass]
pub struct K1SecretKey(chia_sdk_bindings::K1SecretKey);

#[pymethods]
impl K1SecretKey {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::K1SecretKey::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }

    pub fn public_key(&self) -> PyResult<K1PublicKey> {
        Ok(K1PublicKey(self.0.public_key()?))
    }

    pub fn sign_prehashed(&self, prehashed: Vec<u8>) -> PyResult<K1Signature> {
        Ok(K1Signature(self.0.sign_prehashed(prehashed.rust()?)?))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct K1PublicKey(chia_sdk_bindings::K1PublicKey);

#[pymethods]
impl K1PublicKey {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::K1PublicKey::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct K1Signature(chia_sdk_bindings::K1Signature);

#[pymethods]
impl K1Signature {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::K1Signature::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }
}

#[pyclass]
pub struct R1SecretKey(chia_sdk_bindings::R1SecretKey);

#[pymethods]
impl R1SecretKey {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::R1SecretKey::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }

    pub fn public_key(&self) -> PyResult<R1PublicKey> {
        Ok(R1PublicKey(self.0.public_key()?))
    }

    pub fn sign_prehashed(&self, prehashed: Vec<u8>) -> PyResult<R1Signature> {
        Ok(R1Signature(self.0.sign_prehashed(prehashed.rust()?)?))
    }
}

#[pyclass]
#[derive(Clone)]
pub struct R1PublicKey(chia_sdk_bindings::R1PublicKey);

#[pymethods]
impl R1PublicKey {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::R1PublicKey::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct R1Signature(chia_sdk_bindings::R1Signature);

#[pymethods]
impl R1Signature {
    #[staticmethod]
    pub fn from_bytes(bytes: Vec<u8>) -> PyResult<Self> {
        Ok(Self(chia_sdk_bindings::R1Signature::from_bytes(
            bytes.rust()?,
        )?))
    }

    pub fn to_bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.to_bytes()?.py()?)
    }
}
