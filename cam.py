#!/usr/bin/python3

import socket
import time

HOST = "127.0.0.1"
PORT = 8888
cam_target = "-f rtp rtp://127.0.0.1:5008?pkt_size=1200"

                

print("CAMERA: READY")
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.bind((HOST, PORT))
    s.listen()
    print("LISTENING...")
    conn, addr = s.accept()
    with conn:
        print(f"CONNECTION: {addr}")
        while True:
            data = conn.recv(1024)
            #if not data:
            #    break
            #else:
            if data:
                print(f"MSG: {data}")





#ffmpeg -re -f lavfi -i testsrc=size=640x480:rate=30 -vcodec libvpx -cpu-used 5 -deadline 1 -g 10 -error-resilient 1 -auto-alt-ref 1 -f rtp rtp://127.0.0.1:5004?pkt_size=1200
