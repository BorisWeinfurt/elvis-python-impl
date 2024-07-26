//! Represents a point-to-point connection, consisting of 2 Channels

use std::sync::{Arc, Mutex};

use pyo3::prelude::*;
use crate::channel::Channel;

#[pyclass]
#[derive(Clone)]
pub(crate) struct Connection {
    left: Arc<Mutex<Channel>>,
    right: Arc<Mutex<Channel>>,
}

impl Connection {
    pub fn get_left(&self) -> Arc<Mutex<Channel>> {
        self.left.clone()
    }

    pub fn get_right(&self) -> Arc<Mutex<Channel>> {
        self.right.clone()
    }
}

#[pymethods]
impl Connection {
    /// Builds a connection between two channels
    #[new]
    pub fn new() -> Self {
        let mut left = Arc::new(Mutex::new(Channel::new()));
        let mut right = Arc::new(Mutex::new(Channel::new()));

        left.lock().unwrap().add_buddy(right.clone());
        right.lock().unwrap().add_buddy(left.clone());
        Self { left, right }
    }
}

#[pymodule]
pub fn PyConnection(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Connection>()?;
    Ok(())
}