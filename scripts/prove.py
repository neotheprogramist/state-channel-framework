#!/usr/bin/env python

import aiohttp
import asyncio
import json
import sys

async def send_async_request(url, data):
    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2)) as response:
            print("Status:", response.status)
            print("Content-type:", response.headers['content-type'])
            response_data = await response.text()
            print("Response:", response_data)

async def main():
    # Read JSON data from stdin
    input_json = sys.stdin.read()
    data = json.loads(input_json)

    # URL of the local endpoint
    url = "http://localhost:3000/prove/state-diff-commitment"

    # Send the async request
    await send_async_request(url, data)

if __name__ == "__main__":
    asyncio.run(main())
