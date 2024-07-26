import elvos
import time


def main():
    connection = elvos.Connection()
    
    addr1 = elvos.EthernetAddress([0x02, 0x0, 0x0, 0x0, 0x0, 0x01])
    machine1 = elvos.MyCoolMachine(addr1, connection, "left")
    node1 = elvos.MyCoolMachineNode(machine1)
    node1.add_iface(elvos.Config(addr1), elvos.Instant.now())
    node1.update_ip_addr(elvos.IpAddress.v4(1,2,3,4),24)
    node1.add_ipv4_route([1,2,3,100])
    
    addr2 = elvos.EthernetAddress([0x02, 0x0, 0x0, 0x0, 0x0, 0x02])
    machine2 = elvos.MyCoolMachine(addr2, connection, "right")
    node2 = elvos.MyCoolMachineNode(machine2)
    node2.add_iface(elvos.Config(addr2), elvos.Instant.now())
    node2.update_ip_addr(elvos.IpAddress.v4(1,2,3,5),24)
    node2.add_ipv4_route([1,2,3,100])
    
    client_handle = node1.add_tcp_socket()
    server_handle = node2.add_tcp_socket()
    
    remode_addr = elvos.IpAddress.v4(1,2,3,5)
    remote_port = 1234
    host_port = 65000
    
    while True:
        print("Polling Client")
        node1.poll(elvos.Instant.now())
        print("Polling Server")
        node2.poll(elvos.Instant.now())
        
        print("Client Side")
        client_state = node1.socket_status(client_handle)
        print("Server Side")
        server_state = node2.socket_status(server_handle)

        node2.start_http_server(server_handle, remote_port)
        
        node1.start_http_client(client_handle, remode_addr, remote_port, host_port)

        if client_state == elvos.State.Established and server_state == elvos.State.Established:
            node1.send_request(client_handle, "GET", "/elvis.html")
            node2.handle_http_server(server_handle)

        time.sleep(1)


main()