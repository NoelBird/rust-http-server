import socket

H = "localhost"
P = 4221
soc = socket.socket(socket.AddressFamily.AF_INET, socket.SocketKind.SOCK_STREAM)
soc.connect((H, P))
# soc.send(b"GET / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n")
# soc.send(b"GET /raspberry HTTP/1.1\r\nHost: localhost:4221\r\n\r\n")
soc.send(b"GET / HTTP/1.1\r\nHost: localhost:4221\r\n\r\n")
print(soc.recv(1024))
