import time
from elvos import (
    Connection,
    EthernetAddress,
    MyCoolMachine,
    MyCoolMachineNode,
    Config,
    Instant,
    IpAddress,
    State
)


def main():
    connection = Connection()
    
    addr1 = EthernetAddress([0x02, 0x0, 0x0, 0x0, 0x0, 0x01])
    machine1 = MyCoolMachine(addr1, connection, "left")
    node1 = MyCoolMachineNode(machine1)
    node1.add_iface(Config(addr1), Instant.now())
    node1.update_ip_addr(IpAddress.v4(1, 2, 3, 4), 24)
    node1.add_ipv4_route([1, 2, 3, 100])
    
    addr2 = EthernetAddress([0x02, 0x0, 0x0, 0x0, 0x0, 0x02])
    machine2 = MyCoolMachine(addr2, connection, "right")
    node2 = MyCoolMachineNode(machine2)
    node2.add_iface(Config(addr2), Instant.now())
    node2.update_ip_addr(IpAddress.v4(1, 2, 3, 5), 24)
    node2.add_ipv4_route([1, 2, 3, 100])
    
    client_handle = node1.add_tcp_socket()
    server_handle = node2.add_tcp_socket()
    
    remote_addr = IpAddress.v4(1, 2, 3, 5)
    remote_port = 1234
    host_port = 65000
    
    while True:
        print("Polling Client")
        node1.poll(Instant.now())
        print("Polling Server")
        node2.poll(Instant.now())
        
        print("Client Side")
        client_state = node1.socket_status(client_handle)
        print("Server Side")
        server_state = node2.socket_status(server_handle)

        node2.start_http_server(server_handle, remote_port)
        
        node1.start_http_client(client_handle, remote_addr, remote_port, host_port)

        if client_state == State.Established and server_state == State.Established:
            node1.send_request(client_handle, "GET", "/elvis.html")
            node2.handle_http_server(server_handle)

        time.sleep(1)


if __name__ == "__main__":
    main()
