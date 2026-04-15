# #!/usr/bin/python3

import socket
import time

#from picamera2 import Picamera2
#from picamera2.encoders import H264Encoder
#from picamera2.outputs import FfmpegOutput
import asyncio
from websockets.asyncio.server import serve

cam_target = "-f rtp rtp://127.0.0.1:5008?pkt_size=1200"
#picam2 = Picamera2()

async def handler(websocket):
    while True:
        message = await websocket.recv(True)
        print(message)

async def camera_ctrl(websocket):
    async for message in websocket:
        print(f"Received: {message}")
        if message == 'CAM START':
            await start_camera()
        elif message == 'CAM STOP':
            await stop_camera()
        #await websocket.send(message)


async def main():
    async with serve(handler, "127.0.0.1", 8888) as server:
        print("Listening on ws://127.0.0.1:8888")
        await server.serve_forever()


async def start_camera():
    pass
    #video_config = picam2.create_video_configuration({"size": (960, 540)})
    #picam2.configure(video_config)
    #encoder = H264Encoder(1000000)
    #picam2.start_recording(encoder, FfmpegOutput(cam_target))

async def stop_camera():
    pass
    #picam2.stop_recording()

if __name__ == "__main__":
    print("PYTHON CAMERA")
    asyncio.run(main())

