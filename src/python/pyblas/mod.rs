use pyo3::prelude::*;
use pyo3::types::PyCapsule;

// public blas interface
mod blas_wrappers;
pub use blas_wrappers::*;

// public lapack interface
mod lapack_wrappers;
pub use lapack_wrappers::*;

mod blas_loader;
mod blas_types;
mod lapack_loader;
mod lapack_types;

// utilities for scipy blas/lapack import
macro_rules! get_ptr {
    ($api: ident, $str: tt) => {
        std::mem::transmute(get_capsule_void_ptr($api, $str)?)
    };
}
pub(crate) use get_ptr;

fn get_capsule_void_ptr(pyx_capi: &PyAny, name: &str) -> PyResult<*mut libc::c_void> {
    let capsule: &PyCapsule = pyx_capi.get_item(name)?.downcast()?;
    Ok(capsule.pointer())
}

fn get_pyx_capi<'a>(py: Python<'a>, pymodule: &str) -> PyResult<&'a PyAny> {
    let lib = PyModule::import(py, pymodule)?;
    let api = lib.getattr("__pyx_capi__")?;
    Ok(api)
}
