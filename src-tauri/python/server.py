from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.routing import Route
import uvicorn
import json
import signal
import sys
import os
import asyncio

# Global server reference
uvicorn_server = None

def signal_handler(signum, frame):
    """Handle termination signals"""
    print("Received signal to terminate")
    if uvicorn_server and uvicorn_server.server:
        uvicorn_server.server.should_exit = True
    sys.exit(0)

async def process_text(request):
    data = await request.json()
    text = data.get('text', '')
    # Simple text transformation: reverse the text
    processed_text = text[::-1]
    return JSONResponse({
        'processed_text': processed_text
    })

async def shutdown(request):
    """Endpoint to trigger server shutdown"""
    print("Shutdown endpoint called")
    if uvicorn_server:
        uvicorn_server.server.should_exit = True
        # Force exit after a brief delay
        asyncio.create_task(_delayed_exit())
    return JSONResponse({"status": "shutting down"})

async def _delayed_exit():
    """Exit after a brief delay to allow response to be sent"""
    await asyncio.sleep(0.5)
    os._exit(0)

routes = [
    Route('/process', process_text, methods=['POST']),
    Route('/shutdown', shutdown, methods=['POST'])
]

app = Starlette(debug=True, routes=routes)

class UvicornServer:
    def __init__(self):
        self.config = uvicorn.Config(
            app=app,
            host='127.0.0.1',
            port=8000
        )
        self.server = uvicorn.Server(self.config)
    
    async def run(self):
        await self.server.serve()

if __name__ == '__main__':
    # Register signal handlers
    signal.signal(signal.SIGTERM, signal_handler)
    signal.signal(signal.SIGINT, signal_handler)
    
    # Make server globally accessible
    uvicorn_server = UvicornServer()
    
    # Run the server
    try:
        asyncio.run(uvicorn_server.run())
    except KeyboardInterrupt:
        print("Received KeyboardInterrupt")
        sys.exit(0) 