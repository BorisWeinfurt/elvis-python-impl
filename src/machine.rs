use crate::{
    channel::Channel,
    connection::Connection,
    token::{MyRxToken, MyTxToken}, util::PyEthernetAddress,
};
use log::trace;
use pyo3::prelude::*;
use smoltcp::{
    iface::{Config, Context, Interface, SocketSet},
    phy::{Device, DeviceCapabilities, Medium},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address},
};
use std::{
    cell::RefCell,
    collections::VecDeque,
    net::Ipv4Addr,
    rc::Rc,
    sync::{Arc, Mutex},
};

#[pyclass]
#[derive(Clone)]
pub struct MyCoolMachine {
    addr: EthernetAddress,
    channel: Arc<Mutex<Channel>>,
}

#[pymethods]
impl MyCoolMachine {
    #[new]
    pub fn py_new(addr: PyEthernetAddress, conn: Connection, side: &str) -> Self {
        Self {
            addr : addr.into(),
            channel: {
                match side {
                    "left" => conn.get_left(),
                    "right" => conn.get_right(),
                    _ => panic!("Unknown side"),
                }
            },
        }
    }
}
impl MyCoolMachine {
    pub fn new(addr: EthernetAddress, channel: Arc<Mutex<Channel>>) -> Self {
        Self { addr, channel }
    }
}

impl Device for MyCoolMachine {
    type RxToken<'a> = MyRxToken
    where
        Self: 'a;

    type TxToken<'a> = MyTxToken
    where
        Self: 'a;
    fn receive(&mut self, timestamp: Instant) -> Option<(Self::RxToken<'_>, Self::TxToken<'_>)> {
        trace!("Machine recieving");
        let rx = MyRxToken {
            channel: self.channel.clone(),
        };
        let tx = MyTxToken {
            channel: self.channel.clone(),
        };

        let c = self.channel.lock().unwrap();
        // Only do something when the channel is full
        if !c.is_empty() {
            trace!("-- CHANNEL IS [NOT] EMPTY --");
            Some((rx, tx))
        } else {
            trace!("-- CHANNEL IS EMPTY --");
            None
        }
    }

    fn transmit(&mut self, timestamp: Instant) -> Option<Self::TxToken<'_>> {
        trace!("Machine transmitting");
        Some(MyTxToken {
            channel: self.channel.clone(),
        })
    }

    fn capabilities(&self) -> smoltcp::phy::DeviceCapabilities {
        let mut cap = DeviceCapabilities::default();
        cap.medium = Medium::Ethernet;
        cap.max_transmission_unit = 65535;
        cap
    }
}

#[pymodule]
pub fn PyMyCoolMachine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyCoolMachine>()?;
    Ok(())
}