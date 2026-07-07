import asyncio
import websockets
import json
import signal

CLIENTS = set()

async def handler(websocket):
    CLIENTS.add(websocket)
    try:
        async for message in websocket:
            print(message)
            #await websocket.send(f"echo: {message}")
            #others = CLIENTS - {websocket}
            #data = json.dumps({"from": id(websocket), "msg": message})
            #websockets.broadcast(others, data)
    except websockets.ConnectionClosed:
        pass
    finally:
        CLIENTS.discard(websocket)

async def main():
    loop = asyncio.get_running_loop()
    stop = loop.create_future()
    #loop.add_signal_handler(signal.SIGTERM, stop.set_result, None)

    async with websockets.serve(handler, "0.0.0.0", 8001):
        print("Server running on ws://0.0.0.0:8765")
        await stop  # Run until SIGTERM

if __name__ == "__main__":
    asyncio.run(main())