use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use a5::LonLat;

/// Convert (lon, lat) to an A5 cell index at the given resolution.
#[pyfunction]
fn lonlat_to_cell(lon: f64, lat: f64, resolution: i32) -> PyResult<u64> {
    let lonlat = LonLat::new(lon, lat);
    a5::lonlat_to_cell(lonlat, resolution)
        .map_err(|e| PyValueError::new_err(e))
}

/// Batch convert many (lon, lat) pairs to A5 cell indexes.
/// Takes a flat list [lon0, lat0, lon1, lat1, ...] for efficiency.
#[pyfunction]
fn lonlat_to_cell_batch(coords: Vec<f64>, resolution: i32) -> PyResult<Vec<u64>> {
    if coords.len() % 2 != 0 {
        return Err(PyValueError::new_err("coords must have even length (lon, lat pairs)"));
    }
    let mut cells = Vec::with_capacity(coords.len() / 2);
    for chunk in coords.chunks_exact(2) {
        let lonlat = LonLat::new(chunk[0], chunk[1]);
        let cell = a5::lonlat_to_cell(lonlat, resolution)
            .map_err(|e| PyValueError::new_err(e))?;
        cells.push(cell);
    }
    Ok(cells)
}

/// Get the centre (lon, lat) of a cell.
#[pyfunction]
fn cell_to_lonlat(cell: u64) -> PyResult<(f64, f64)> {
    let lonlat = a5::cell_to_lonlat(cell)
        .map_err(|e| PyValueError::new_err(e))?;
    Ok((lonlat.longitude.0, lonlat.latitude.0))
}

/// Get the boundary of a cell as a list of (lon, lat) tuples.
#[pyfunction]
fn cell_to_boundary(cell: u64) -> PyResult<Vec<(f64, f64)>> {
    let boundary = a5::cell_to_boundary(cell, None)
        .map_err(|e| PyValueError::new_err(e))?;
    Ok(boundary.iter().map(|ll| (ll.longitude.0, ll.latitude.0)).collect())
}

/// Get the resolution of a cell.
#[pyfunction]
fn get_resolution(cell: u64) -> i32 {
    a5::get_resolution(cell)
}

/// Get the parent cell at a given resolution.
#[pyfunction]
fn cell_to_parent(cell: u64, parent_resolution: Option<i32>) -> PyResult<u64> {
    a5::cell_to_parent(cell, parent_resolution)
        .map_err(|e| PyValueError::new_err(e))
}

/// Get child cells at a given resolution.
#[pyfunction]
fn cell_to_children(cell: u64, child_resolution: Option<i32>) -> PyResult<Vec<u64>> {
    a5::cell_to_children(cell, child_resolution)
        .map_err(|e| PyValueError::new_err(e))
}

/// Get the 12 resolution-0 cells.
#[pyfunction]
fn get_res0_cells() -> PyResult<Vec<u64>> {
    a5::get_res0_cells()
        .map_err(|e| PyValueError::new_err(e))
}

/// Get the area of a cell at a given resolution in m².
#[pyfunction]
fn cell_area(resolution: i32) -> f64 {
    a5::cell_area(resolution)
}

/// Get total number of cells at a given resolution.
#[pyfunction]
fn get_num_cells(resolution: i32) -> u64 {
    a5::get_num_cells(resolution)
}

/// Compact a list of cells by merging complete sibling groups into parents.
#[pyfunction]
fn compact(cells: Vec<u64>) -> PyResult<Vec<u64>> {
    a5::compact(&cells)
        .map_err(|e| PyValueError::new_err(e))
}

/// Expand cells to a target resolution.
#[pyfunction]
fn uncompact(cells: Vec<u64>, target_resolution: i32) -> PyResult<Vec<u64>> {
    a5::uncompact(&cells, target_resolution)
        .map_err(|e| PyValueError::new_err(e))
}

/// Convert a cell u64 to hex string.
#[pyfunction]
fn u64_to_hex(cell: u64) -> String {
    a5::u64_to_hex(cell)
}

/// Convert a hex string to cell u64.
#[pyfunction]
fn hex_to_u64(hex: &str) -> PyResult<u64> {
    a5::hex_to_u64(hex)
        .map_err(|e| PyValueError::new_err(e))
}

/// Get cells within k hops of a cell (edge-sharing neighbours).
#[pyfunction]
fn grid_disk(cell: u64, k: usize) -> PyResult<Vec<u64>> {
    a5::grid_disk(cell, k)
        .map_err(|e| PyValueError::new_err(e))
}

#[pymodule]
fn a5_fast(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(lonlat_to_cell, m)?)?;
    m.add_function(wrap_pyfunction!(lonlat_to_cell_batch, m)?)?;
    m.add_function(wrap_pyfunction!(cell_to_lonlat, m)?)?;
    m.add_function(wrap_pyfunction!(cell_to_boundary, m)?)?;
    m.add_function(wrap_pyfunction!(get_resolution, m)?)?;
    m.add_function(wrap_pyfunction!(cell_to_parent, m)?)?;
    m.add_function(wrap_pyfunction!(cell_to_children, m)?)?;
    m.add_function(wrap_pyfunction!(get_res0_cells, m)?)?;
    m.add_function(wrap_pyfunction!(cell_area, m)?)?;
    m.add_function(wrap_pyfunction!(get_num_cells, m)?)?;
    m.add_function(wrap_pyfunction!(compact, m)?)?;
    m.add_function(wrap_pyfunction!(uncompact, m)?)?;
    m.add_function(wrap_pyfunction!(u64_to_hex, m)?)?;
    m.add_function(wrap_pyfunction!(hex_to_u64, m)?)?;
    m.add_function(wrap_pyfunction!(grid_disk, m)?)?;
    Ok(())
}
