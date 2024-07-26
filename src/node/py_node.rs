use super::{Node,HttpClient,HttpServer};
use crate::machine::MyCoolMachine;
use crate::util::{PyConfig, PyInstant, PyIpAddress, PySocketHandle, PyState};
use pyo3::prelude::*;
use smoltcp::{
    iface::{Config, Context, Interface, SocketHandle, SocketSet},
    phy::Device,
    socket::{
        dhcpv4::Socket,
        tcp::{self, State},
    },
    time::Instant,
    wire::{IpAddress, IpCidr, IpEndpoint, IpListenEndpoint, Ipv4Address},
};

macro_rules! create_interface {
    ($name:ident, $device:ty) => {
        #[pyclass]
        pub struct $name {
            inner: Node<$device>,
        }

        #[pymethods]
        impl $name {
            #[new]
            pub fn new(device: $device) -> Self {
                Self {
                    inner: Node::new(device),
                }
            }

            pub fn add_iface(&mut self, config: PyConfig, timestamp: PyInstant) {
                self.inner.add_iface(config.into(), timestamp.into());
            }

            pub fn update_ip_addr(&mut self, addr: PyIpAddress, subnet: u8) {
                self.inner.update_ip_addr(addr.into(), subnet);
            }

            pub fn add_ipv4_route(&mut self, addr: [u8; 4]) {
                self.inner.add_ipv4_route(addr);
            }

            pub fn poll(&mut self, timestamp: PyInstant) {
                self.inner.poll(timestamp.into());
            }

            // pub fn delay(&mut self, sockets: &mut SocketSet) {
            //     self.inner.delay(sockets);
            // }

            // pub fn context(&mut self) -> &mut Context {
            //     self.inner.context()
            // }

            pub fn add_tcp_socket(&mut self) -> PySocketHandle {
                self.inner.add_tcp_socket().into()
            }

            pub fn socket_status(&mut self, handle: PySocketHandle) -> PyState {
                self.inner.socket_status(handle.into()).into()
            }

            pub fn socket_connect(
                &mut self,
                handle: PySocketHandle,
                remote_addr: PyIpAddress,
                remote_port: u16,
                host_port: u16,
            ) {
                self.inner.socket_connect(
                    handle.into(),
                    remote_addr.into(),
                    remote_port,
                    host_port,
                );
            }

            pub fn socket_listen(&mut self, handle: PySocketHandle, remote_port: u16) {
                self.inner.socket_listen(handle.into(), remote_port);
            }

            // pub fn socket_recv<'b, F, R>(&'b mut self, handle: PySocketHandle, f: F)
            // where
            //     F: FnOnce(&'b mut [u8]) -> (usize, R),
            // {
            //     self.inner.socket_recv(handle, f);
            // }

            pub fn socket_send(&mut self, handle: PySocketHandle, packet: &[u8]) {
                self.inner.socket_send(handle.into(), packet);
            }

            // The following are trait methods that arent technically part of node but are here for testing

            pub fn start_http_server(&mut self, handle: PySocketHandle, remote_port: u16) {
                if std::any::TypeId::of::<$device>() == std::any::TypeId::of::<MyCoolMachine>() {
                    self.inner.start_http_server(handle.into(), remote_port);
                } else {
                    panic!("function does not exist for this type");
                }
            }

            pub fn handle_http_server(&mut self, handle: PySocketHandle) {
                if std::any::TypeId::of::<$device>() == std::any::TypeId::of::<MyCoolMachine>() {
                    self.inner.handle_http_server(handle.into());
                } else {
                    panic!("functin does not exist for this type");
                }
            }
            pub fn start_http_client(
                &mut self,
                handle: PySocketHandle,
                remote_addr: PyIpAddress,
                remote_port: u16,
                host_port: u16,
            ) {
                if std::any::TypeId::of::<$device>() == std::any::TypeId::of::<MyCoolMachine>() {
                    self.inner.start_http_client(handle.into(), remote_addr.into(), remote_port, host_port);
                } else {
                    panic!("functin does not exist for this type");
                }
            }

            pub fn send_request(&mut self, handle: PySocketHandle, method: &str, path: &str) {
                if std::any::TypeId::of::<$device>() == std::any::TypeId::of::<MyCoolMachine>() {
                    self.inner.send_request(handle.into(), method, path);
                } else {
                    panic!("functin does not exist for this type");
                }
            }

            pub fn handle_http_client(&mut self, handle: PySocketHandle) {
                if std::any::TypeId::of::<$device>() == std::any::TypeId::of::<MyCoolMachine>() {
                    self.inner.handle_http_client(handle.into());
                } else {
                    panic!("functin does not exist for this type");
                }
            }
        }
    };
}

create_interface!(MyCoolMachineNode, MyCoolMachine);

#[pymodule]
pub fn Nodes(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyCoolMachineNode>()?;
    Ok(())
}