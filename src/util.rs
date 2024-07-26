use pyo3::prelude::*;
use pyo3::types::PyAny;
use pyo3::exceptions::PyValueError;
use smoltcp::{
    iface::{Config, SocketHandle},
    socket::tcp::State,
    time::Instant,
    wire::{EthernetAddress, HardwareAddress, IpAddress, Ipv4Address, Ipv6Address},
};

#[pyclass(name = "EthernetAddress")]
#[derive(Clone)]
pub struct PyEthernetAddress([u8; 6]);

#[pymethods]
impl PyEthernetAddress {
    #[new]
    fn new(octets: [u8; 6]) -> Self {
        PyEthernetAddress(octets)
    }
}

impl From<PyEthernetAddress> for EthernetAddress {
    fn from(py_addr: PyEthernetAddress) -> Self {
        EthernetAddress(py_addr.0)
    }
}

#[pyclass(name = "SocketHandle")]
#[derive(Clone)]
pub struct PySocketHandle(SocketHandle);
impl From<PySocketHandle> for SocketHandle {
    fn from(py_handle: PySocketHandle) -> SocketHandle {
        py_handle.0
    }
}
impl From<SocketHandle> for PySocketHandle {
    fn from(handle: SocketHandle) -> PySocketHandle {
        PySocketHandle(handle)
    }
}

#[pyclass(name = "Instant")]
#[derive(Clone)]
pub struct PyInstant(Instant);
#[pymethods]
impl PyInstant {
    #[new]
    fn new(micros: i64) -> Self {
        Self(Instant::from_micros(micros))
    }

    #[staticmethod]
    fn now() -> Self {
        Self(Instant::now())
    }
}
impl From<PyInstant> for Instant {
    fn from(py_instant: PyInstant) -> Instant {
        py_instant.0
    }
}

#[pyclass(name = "IpAddress")]
#[derive(Clone)]
pub struct PyIpAddress(IpAddress);
#[pymethods]
impl PyIpAddress {
    #[staticmethod]
    fn v4(a0: u8, a1: u8, a2: u8, a3: u8) -> PyIpAddress {
        Self(IpAddress::Ipv4(Ipv4Address::new(a0, a1, a2, a3)))
    }
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn v6(a0: u16, a1: u16, a2: u16, a3: u16, a4: u16, a5: u16, a6: u16, a7: u16) -> PyIpAddress {
        Self(IpAddress::Ipv6(Ipv6Address::new(
            a0, a1, a2, a3, a4, a5, a6, a7,
        )))
    }
}
impl From<PyIpAddress> for IpAddress {
    fn from(py_ip: PyIpAddress) -> IpAddress {
        py_ip.0
    }
}

#[pyclass(name = "Config")]
#[derive(Clone)]
pub struct PyConfig([u8; 6]);
#[pymethods]
impl PyConfig {
    #[new]
    fn new(addr: Py<PyAny>) -> PyResult<Self> {
        Python::with_gil(|py| {
            if let Ok(eth) = addr.extract::<PyEthernetAddress>(py) {
                return  Ok(PyConfig(eth.0));
            }
            if let Ok(array) = addr.extract::<[u8; 6]>(py) {
                return Ok(PyConfig(array));
            }
            Err(pyo3::exceptions::PyTypeError::new_err("Invalid type for PyConfig"))
        })
    }

    fn __repr__(&self)-> String {
        format!("PyConfig({:?})", self.0)
    }
}
impl From<PyConfig> for Config {
    fn from(py_config: PyConfig) -> Config {
        Config::new(EthernetAddress(py_config.0).into())
    }
}

#[pyclass(name = "State", eq, eq_int)]
#[derive(Clone, PartialEq, Eq)]
pub enum PyState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
    FinWait1,
    FinWait2,
    CloseWait,
    Closing,
    LastAck,
    TimeWait,
}

impl From<PyState> for State {
    fn from(py_state: PyState) -> State {
        match py_state {
            PyState::Closed => State::Closed,
            PyState::Listen => State::Listen,
            PyState::SynSent => State::SynSent,
            PyState::SynReceived => State::SynReceived,
            PyState::Established => State::Established,
            PyState::FinWait1 => State::FinWait1,
            PyState::FinWait2 => State::FinWait2,
            PyState::CloseWait => State::CloseWait,
            PyState::Closing => State::Closing,
            PyState::LastAck => State::LastAck,
            PyState::TimeWait => State::TimeWait,
        }
    }
}

impl From<State> for PyState {
    fn from(state: State) -> PyState {
        match state {
            State::Closed => PyState::Closed,
            State::Listen => PyState::Listen,
            State::SynSent => PyState::SynSent,
            State::SynReceived => PyState::SynReceived,
            State::Established => PyState::Established,
            State::FinWait1 => PyState::FinWait1,
            State::FinWait2 => PyState::FinWait2,
            State::CloseWait => PyState::CloseWait,
            State::Closing => PyState::Closing,
            State::LastAck => PyState::LastAck,
            State::TimeWait => PyState::TimeWait,
        }
    }
}



#[pymodule]
pub fn util_wrappers(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyEthernetAddress>()?;
    m.add_class::<PyConfig>()?;
    m.add_class::<PySocketHandle>()?;
    m.add_class::<PyInstant>()?;
    m.add_class::<PyState>()?;
    m.add_class::<PyIpAddress>()?;
    Ok(())
}