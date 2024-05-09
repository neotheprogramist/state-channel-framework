#!/usr/bin/env python

import aiohttp
import asyncio
import json
import sys
from nacl.signing import SigningKey
from nacl.encoding import HexEncoder

async def send_async_request(url,  data):
    headers = {
        'Authorization': f'Bearer {"token"}',
        'Content-Type': 'application/json'  # Ensure content type is set for JSON data
    }
    url = "http://localhost:7003/prove/state-diff-commitment"

    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2)) as response:
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

            
async def test_generate_nonce():
    url = "http://localhost:7003/auth"
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
    response_json = await get_nonce(public_key.decode()) 

    if not response_json:
        print("No JSON response received to sign.")
        return

    nonce = response_json.get('nonce', '')
    if not nonce:
        print("No message found in JSON response.")
        return

    signed_nonce = signing_key.sign(nonce.encode())

    url = "http://localhost:7003/auth"
    async with aiohttp.ClientSession() as session:
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
  
async def test_validate_signature_unauthorized():
    signing_key = SigningKey.generate()
    public_key = signing_key.verify_key.encode(encoder=HexEncoder)

    response_json = await get_nonce(public_key.decode()) 

    if not response_json:
        print("No JSON response received to sign.")
        return

    nonce = response_json.get('nonce', '')
    if not nonce:
        print("No message found in JSON response.")
        return
    
    signed_nonce = signing_key.sign(nonce.encode())

    invalid_signature = bytearray(signed_nonce.signature)
    invalid_signature[0] ^= 0xff  

    url = "http://localhost:7003/auth"
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


# Get the nonce by passing public_key in auth url params
async def get_nonce(public_key):
    url = "http://localhost:7003/auth"
    print(url)
    async with aiohttp.ClientSession() as session:
        params = {'public_key': public_key}
        async with session.get(url, params=params) as response:
            if response.status != 200:
                raise Exception(f"Failed to get nonce. Status code: {response.status}")
            
            response_data = await response.json()
            print("Status Code:", response.status)
            print("Making a request to:", response.url) 
            print(response_data)
            return response_data
        
async def validate_signature(url):
    signing_key = SigningKey.generate()
    public_key = signing_key.verify_key.encode(encoder=HexEncoder)
    response_json = await get_nonce(public_key.decode()) 

    nonce = response_json.get('nonce', '')
    signed_nonce = signing_key.sign(nonce.encode())

    async with aiohttp.ClientSession() as session:
        data = {
            "public_key": public_key.decode(),
            "nonce": nonce,
            "signature": signed_nonce.signature.hex()
        }
        async with session.post(url, json=data) as response:
            response_data = await response.json()
            print("Status Code:", response.status)
            print("Making a request to:", response.url) 
      
            return response_data
        

async def test_prover_with_JWT(url,data):
    response_json=await validate_signature(url)
    token = response_json.get("jwt_token")
    headers = {
        'Authorization': f'Bearer {token}',
        'Content-Type': 'application/json'  # Ensure content type is set for JSON data
    }
    url = "http://localhost:7003/prove/state-diff-commitment"

    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2), headers=headers) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url) 
            print(response_data)

async def test_prover_with_invalid_JWT(url,data):
    response_json=await validate_signature(url)
    token = response_json.get("jwt_token")
    headers = {
        'Authorization': f'Bearer {"BLABLABLA"}',
        'Content-Type': 'application/json'  # Ensure content type is set for JSON data
    }
    url = "http://localhost:7003/prove/state-diff-commitment"

    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2), headers=headers) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url) 
            print(response_data)


# Set env variable expiration time to 1s
async def test_expiration_time(url,data):
    response_json=await validate_signature(url)
    token = response_json.get("jwt_token")
    headers = {
        'Authorization': f'Bearer {token}',
        'Content-Type': 'application/json'  # Ensure content type is set for JSON data
    }
    url = "http://localhost:7003/prove/state-diff-commitment"

    async with aiohttp.ClientSession() as session:
        async with session.post(url, data=json.dumps(data, indent=2), headers=headers) as response:
            response_data = await response.text()
            print("Status Code:", response.status)
            print("Making a request to:", url) 
            print(response_data)

async def main():
    # Read JSON data from stdin
    print("Maine")
    input_json = sys.stdin.read()
    data = json.loads(input_json)
    url = "http://localhost:7003/auth"
    # await test_prover_with_JWT(url,data)
    signing_key = SigningKey.generate()
    public_key = signing_key.verify_key.encode(encoder=HexEncoder)

    await get_nonce(public_key.decode())


if __name__ == "__main__":
    asyncio.run(main())
