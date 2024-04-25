#!/usr/bin/env python

import aiohttp
import asyncio
import json
import sys
from nacl.signing import SigningKey
from nacl.encoding import HexEncoder

async def send_async_request(url,  data):
    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2)) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)
            print(response_data)

async def test_get_slow():
    url = "http://localhost:7003/slow"
    async with aiohttp.ClientSession() as session:
        async with session.get(url) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)

            print(response_data)
async def test_generate_nonce():
    url = "http://localhost:7003/prove/state-diff-commitment/auth"
    public_key = "0x123123123123"  # Replace this with the actual public key you want to use

    async with aiohttp.ClientSession() as session:
        params = {'public_key': public_key}
        async with session.get(url, params=params) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", response.url) 
            print(response_data)


async def test_validate_signature():

    signing_key = SigningKey.generate()
    public_key = signing_key.verify_key.encode(encoder=HexEncoder)
    

    response_json = await get_nonce(public_key.decode())  # Get JSON response from the server

    if not response_json:
        print("No JSON response received to sign.")
        return

    # Extract the message and then the nonce
    full_message = response_json.get('message', '')
    if not full_message:
        print("No message found in JSON response.")
        return

    try:
        # Assuming the format "Confirm identity by signing random data:\n<nonce>"
        nonce = full_message.split('\n')[1]
    except IndexError:
        print("Failed to extract nonce from the message.")
        return
    

    # Sign the nonce received from the server
    signed_nonce = signing_key.sign(nonce.encode())

    url = "http://localhost:7003/prove/state-diff-commitment/auth"
    async with aiohttp.ClientSession() as session:
        # Prepare data to send (nonce and signature)
        data = {
            "public_key": public_key.decode(),
            "nonce": nonce,
            "signature": signed_nonce.signature.hex()
        }
        async with session.post(url, json=data) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)
            print("Data sent:", json.dumps(data, indent=2))
            print(response_data)


async def test_validate_signature_with_invalid_signature():
    signing_key = SigningKey.generate()
    public_key = signing_key.verify_key.encode(encoder=HexEncoder)

    response_json = await get_nonce(public_key.decode()) 

    if not response_json:
        print("No JSON response received to sign.")
        return

    full_message = response_json.get('message', '')
    if not full_message:
        print("No message found in JSON response.")
        return

    try:
        nonce = full_message.split('\n')[1]
    except IndexError:
        print("Failed to extract nonce from the message.")
        return
    
    signed_nonce = signing_key.sign(nonce.encode())

    invalid_signature = bytearray(signed_nonce.signature)
    invalid_signature[0] ^= 0xff  

    url = "http://localhost:7003/prove/state-diff-commitment/auth"
    async with aiohttp.ClientSession() as session:
        data = {
            "public_key": public_key.decode(),
            "nonce": nonce,
            "signature": invalid_signature.hex()
        }
        async with session.post(url, json=data) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url)
            print("Data sent:", json.dumps(data, indent=2))
            print(response_data)

async def get_nonce(public_key):
    url = "http://localhost:7003/prove/state-diff-commitment/auth"

    async with aiohttp.ClientSession() as session:
        params = {'public_key': public_key}
        async with session.get(url, params=params) as response:
            response_data = await response.json()
            print("Status Code:", response.status)
            print("Making a request to:", response.url) 
            print(response_data)
            return response_data
    
async def main():
    # Read JSON data from stdin
    input_json = sys.stdin.read()
    data = json.loads(input_json)
    
    await test_validate_signature()

if __name__ == "__main__":
    asyncio.run(main())
