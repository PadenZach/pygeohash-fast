use geohash::encode;
use geohash::Coordinate;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use rayon::prelude::*;
use rayon::ThreadPoolBuildError;

/// Create a rayon thread pool of num threads.
/// Thanks to Carl M. Kadie for writing about this on his blog.
/// Usage:
/// create_pool(n_threads)?.install(|| {
///     my_rayon_func()
/// })?;
pub fn create_pool(num_threads: usize) -> Result<rayon::ThreadPool, ThreadPoolBuildError> {
    match rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
    {
        Err(e) => Err(e.into()),
        Ok(pool) => Ok(pool),
    }
}

/// Encodes x and y coordinates to a geohash of len precision.
///
/// Args:
///     lng (float): The X coordinate or "Latitude".
///     lat (float): The Y coordinate, or "longitude".
///     len (int): The length of geohash.
///
///  Returns:
///     str: The geohash string.
#[pyfunction]
#[pyo3(name = "encode")]
fn encode_py(lng: f64, lat: f64, len: usize) -> PyResult<String> {
    match encode(Coordinate { x: lng, y: lat }, len) {
        Ok(geohash) => Ok(geohash),
        Err(e) => Err(PyException::new_err(e.to_string())),
    }
}

/// Encodes a list of lngs and lats into a list of geohashes of length len.
///
/// Args:
///     lngs (List[float]): List of X/Lat values.
///     lats (List[float]): List of Y/Lng values.
///     len (int): Length of encoded geohashes desired.
///     num_threads Optional[int]: number of threads to use. Defaults to # of cpus.
/// Returns:
///     List[str]: List of Geohashes.
#[pyfunction]
#[pyo3(name = "encode_many")]
fn encode_many_py(
    lngs: Vec<f64>,
    lats: Vec<f64>,
    len: usize,
    num_threads: Option<usize>,
) -> Vec<String> {
    let mut results = Vec::with_capacity(lngs.capacity());

    create_pool(num_threads.unwrap_or(num_cpus::get_physical()))
        .unwrap()
        .install(|| {
            lngs.into_par_iter()
                .zip_eq(lats)
                .map(|xy| {
                    encode(
                        Coordinate {
                            x: (xy.0),
                            y: (xy.1),
                        },
                        len,
                    )
                    .unwrap()
                })
                .collect_into_vec(&mut results);
        });
    return results;
}

/// Decodes a geohash string into a coordinates with error.
/// 
/// Args:
///     encoded_geohash: String - Geohash to decode.
/// Returns:
///     Tuple(lng, lat, lng_err, lat_err)
#[pyfunction]
#[pyo3(name = "decode")]
fn decode_py(encoded_geohash:&str) -> PyResult<(f64, f64, f64, f64)> {
    match geohash::decode(encoded_geohash) {
        Ok(v) => return PyResult::Ok((v.0.x,v.0.y, v.1, v.2)),
        Err(e) => return Err(PyException::new_err(e.to_string()))
    }
}

/// Decodes a list of geohash strings into a list of x,y (long, lat) pairs.
///
/// NOTE: Currently, does not return the lat or lng decode error as the normal
/// decode function does.
/// 
/// Args:
///     geohashes: List[String] - Geohashes to decode.
///     num_threads Optional[int]: number of threads to use. Defaults to # of cpus.
/// Returns:
///     List[Tuple[float, float]]: List of long, lat pairs..
#[pyfunction]
#[pyo3(name = "decode_many")]
fn decode_many_py(
    geohashes: Vec<String>,
    num_threads: Option<usize>,
) -> Vec<(f64, f64)> {
    let mut results = Vec::with_capacity(geohashes.capacity());

    create_pool(num_threads.unwrap_or(num_cpus::get_physical()))
        .unwrap()
        .install(|| {
            geohashes.into_par_iter()
                .map(|hash_str| {
                    geohash::decode(
                        &hash_str,
                    )
                    .unwrap().0.into()
                    
                })
                .collect_into_vec(&mut results);
        });
    return results;
}

 
/// A Python module implemented in Rust.
#[pymodule]
fn pygeohash_fast(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_py, m)?)?;
    m.add_function(wrap_pyfunction!(encode_many_py, m)?)?;
    m.add_function(wrap_pyfunction!(decode_py, m)?)?;
    m.add_function(wrap_pyfunction!(decode_many_py, m)?)?;
    Ok(())
}
