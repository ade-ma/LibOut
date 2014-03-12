import socket, sys

UDP_IP = "127.0.0.1"
UDP_PORT = 9997

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

sock.sendto(sys.argv[1], (UDP_IP, UDP_PORT))
