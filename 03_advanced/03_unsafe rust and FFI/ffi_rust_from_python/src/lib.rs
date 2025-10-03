use pyo3::prelude::*;
use std::collections::HashMap;
use pyo3::types::IntoPyDict;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn count_words(s: String) -> PyResult<PyObject> {
    let mut hm = HashMap::new();
    for sub_str in s.split(' ') {
        let count = hm.entry(sub_str).or_insert(0);
        *count += 1;
    }
    Python::with_gil(|py| {
        let dict = hm.into_py_dict(py)?;
        Ok(dict.into_any().unbind())
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn ffi_rust_from_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}

