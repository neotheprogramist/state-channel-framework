# Project Setup and Execution Instructions

## Setting Up the Virtual Environment

1. **Create the Virtual Environment**:

   - Run the following command to create a virtual environment in the current directory:
     ```bash
     python -m venv .venv
     ```

2. **Activate the Virtual Environment**:

   - For Unix or MacOS, activate the virtual environment with:
     ```bash
     source .venv/bin/activate
     ```
   - For Windows, use:
     ```bash
     .venv\Scripts\activate
     ```

3. **Install Dependencies**:
   - Install the required packages from the `requirements.txt` file:
     ```bash
     pip install -r requirements.txt
     ```

## Preparing Required Directories and Files

1. **Create the Resources Directory**:

   - Execute the following command to create a `resources` directory:
     ```bash
     mkdir resources
     ```

2. **Add Configuration File**:
   - Create a file named `input.json` inside the `resources` directory and paste the following JSON content into it:
     ```json
     {
       "prev_state_root": 34343434343,
       "block_number": 123456,
       "block_hash": 1234567890,
       "config_hash": 1234567890,
       "world_da": [
         3488041066649332616440110253331181934927363442882040970594983370166361489161,
         633500000000000,
         2080372569135727803323277605537468839623406868880224375222092136867736091483,
         999999936
       ],
       "message_to_starknet_segment": [123, 456, 123, 128],
       "message_to_appchain_segment": [123, 456, 123, 128],
       "nonce_updates": {
         "1": 12,
         "2": 1337
       },
       "storage_updates": {
         "1": {
           "123456789": 89,
           "987654321": 98
         },
         "2": {
           "123456789": 899,
           "987654321": 98
         }
       },
       "contract_updates": {
         "3": 437267489
       },
       "declared_classes": {
         "1234": 12345,
         "12345": 123456,
         "123456": 1234567
       }
     }
     ```

## Running the HTTP Service

- To start the HTTP service, execute:
  ```bash
  cargo run -p prover
  ```

## Generating a Proof

- To send an HTTP request and generate a proof, run:
  ```bash
  ./scripts/prove.py < resources/input.json > resources/proof.json
  ```

This setup guide will help you to configure and run the necessary components for the project. Make sure you follow the steps in order to ensure everything functions as expected.

# Applier

## Overview

This guide provides instructions to generate sample agreements, set up Starknet Devnet, and deploy contracts on both Devnet and Sepolia.

## Generate Sample Agreements

To generate sample agreements, **ensure you have created the directory resources/json_generator_out/** to avoid a "no such file" error. Then, execute the following command:

```bash
cargo run --bin json_generator -- --agreements-count <number_of_agreements>
```

The generated outputs will be located in **resources/json_generator_out/**, which will contain the necessary data for deploying the contract and applying agreements.

## Setting Up Starknet Devnet

To launch Starknet Devnet, use the command:

```bash
starknet-devnet
```

## Declaring and Deploying Contract on Devnet and Sepolia

Follow these steps to declare and deploy the agreement contract on Devnet:

1. **Set Environment Variables**
    
    If non-existent, create file **.cargo/config.toml** to hold our enviroment variables.
    
    If non-existent, create file **.cargo/config.toml** to hold our enviroment variables.

    Add the necessary enviroment variables. For _DEVNET use one of the predeployed account from starknet-devnet.
   ```bash
   export ADDRESS_DEVNET = "0x18...69"
   export PRIVATE_KEY_DEVNET = "0x26..."

   export PRIVATE_KEY = "0x07...8a3"
   export ADDRESS = "0x028d...52DF30"
   ```
   
2. **Run the Program**

   ```bash
   cargo run --bin agreement_version_2_runner
   ```

This streamlined guide ensures that you have all the necessary steps to generate agreements and deploy contracts on Starknet Devnet and Sepolia.