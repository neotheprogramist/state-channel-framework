#!/usr/bin/env python

import aiohttp
import asyncio
import json
import sys

async def send_async_request(url,  data):

    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2)) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)
            print(response_data)

async def test_get_slowed():
    url = "http://localhost:3618/slowed"
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)

            print(response_data)

async def test_get_slow():
    url = "http://localhost:3618/slow"
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)

            print(response_data)

async def main():
    # Read JSON data from stdin
    input_json = sys.stdin.read()
    data = json.loads(input_json)
    public_key = "1234"
    # URL of the local endpoint

    # Send the async request
    await test_get_slowed()

if __name__ == "__main__":
    asyncio.run(main())
