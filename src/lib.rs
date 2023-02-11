use pyo3::prelude::*;
// use hex_literal::hex;
use sha3::{Digest, Sha3_224, Sha3_256, Sha3_384, Sha3_512};
use sha3::{Keccak224, Keccak256, Keccak384, Keccak512};

#[pyfunction]
fn keccak_224(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a Keccak-224 object
        let mut hasher = Keccak224::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}
#[pyfunction]
fn keccak_256(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a Keccak-256 object
        let mut hasher = Keccak256::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}
#[pyfunction]
fn keccak_384(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a Keccak-384 object
        let mut hasher = Keccak384::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}
#[pyfunction]
fn keccak_512(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a Keccak-512 object
        let mut hasher = Keccak512::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}

/// # Sha3-224
#[pyfunction]
fn sha3_224(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a SHA3-224 object
        let mut hasher = Sha3_224::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}

/// # Sha3-256
#[pyfunction]
fn sha3_256(py: Python, msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a SHA3-256 object
        let mut hasher = Sha3_256::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}

/// # Sha3-384
#[pyfunction]
fn sha3_384(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a SHA3-384 object
        let mut hasher = Sha3_384::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}

/// # Sha3-512
#[pyfunction]
fn sha3_512(py: Python,msg: Vec<u8>) -> PyResult<String> {
    py.allow_threads(move || {
        // create a SHA3-512 object
        let mut hasher = Sha3_512::new();

        // write input message
        hasher.update(msg);

        // read hash digest
        let result = hasher.finalize();
        let hex = hex::encode(result.as_slice());
        Ok(hex)
    })
}

/// Rust SHA3, Keccak
#[pymodule]
fn sha_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sha3_224, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_256, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_384, m)?)?;
    m.add_function(wrap_pyfunction!(sha3_512, m)?)?;
    // Keccak
    m.add_function(wrap_pyfunction!(keccak_224, m)?)?;
    m.add_function(wrap_pyfunction!(keccak_256, m)?)?;
    m.add_function(wrap_pyfunction!(keccak_384, m)?)?;
    m.add_function(wrap_pyfunction!(keccak_512, m)?)?;
    Ok(())
}
