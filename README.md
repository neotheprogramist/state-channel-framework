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

## Applying contract

To declare agreement contract and apply it follow the steps below.

```bash
  cd src/agreement_version_2/
  scarb build

  # Change the snforge profile name if necessary/
  ./1-declare.sh
  ./2-deploy.sh # Args  <class_hash> <client_balance> <server_balance> <client_public_key> <server_public_key> <a> <b>
  ./3-apply # Args <contract_address> <quantity> <nonce> <price> <server_signature_r> <server_signature_s> <client_signature_r> <client_signature_s>
```
