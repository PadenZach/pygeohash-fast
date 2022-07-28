use geohash::encode;
use geohash::Coordinate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rayon::prelude::*;

/// Encodes x and y coordinates to a geohash of len precision.
/// 
/// Args:
///     x (float): The X coordinate or "Latitude".
///     y (float): The Y coordinate, or "longitude".
///     len (int): The length of geohash.
///
///  Returns:
///     str: The geohash string.
#[pyfunction]
#[pyo3(name = "encode")]
fn encode_py(x: f64, y: f64, len: usize) -> PyResult<String> {
    match encode(Coordinate { x: x, y: y }, len) {
        Ok(geohash) => Ok(geohash),
        Err(e) => Err(PyException::new_err(e.to_string())),
    }
}

/// Encodes a list of xs and ys into a list of geohashes of length len.
/// 
/// Args:
///     xs (List[float]): List of X/Lat values.
///     ys (List[float]): List of Y/Lng values.
///     len (int): Length of encoded geohashes desired.
/// Returns:
///     List[str]: List of Geohashes.
#[pyfunction]
#[pyo3(name = "encode_many")]
fn encode_many_py(xs: Vec<f64>, ys: Vec<f64>, len: usize) -> PyResult<Vec<String>> {
    match xs
        .into_par_iter()
        .zip_eq(ys)
        .map(|xy| {
            encode(
                Coordinate {
                    x: (xy.0),
                    y: (xy.1),
                },
                len,
            )
        })
        .collect::<Result<Vec<String>, _>>()
    {
        Ok(results) => Ok(results),
        Err(e) => Err(PyException::new_err(e.to_string())),
    }
}
/// A Python module implemented in Rust.
#[pymodule]
fn pygeohash_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_py, m)?)?;
    m.add_function(wrap_pyfunction!(encode_many_py, m)?)?;
    Ok(())
}
