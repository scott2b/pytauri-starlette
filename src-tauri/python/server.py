from starlette.applications import Starlette
from starlette.responses import JSONResponse
from starlette.routing import Route
import uvicorn
import json

async def process_text(request):
    data = await request.json()
    text = data.get('text', '')
    # Simple text transformation: reverse the text
    processed_text = text[::-1]
    return JSONResponse({
        'processed_text': processed_text
    })

routes = [
    Route('/process', process_text, methods=['POST'])
]

app = Starlette(debug=True, routes=routes)

if __name__ == '__main__':
    uvicorn.run(app, host='127.0.0.1', port=8000) 