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

# Naive method of contract settlements

## Generate Sample Agreements

To generate agreements please follow the steps below.

```bash
  cargo run --bin json_generator -- --agreements-count <number_of_agreements>
```

The outputs of the generator will be located at **resources/json_generator_out/** and will provide you with the data needed to deploy the contract and apply agreements.

## Setting up Starknet Devnet

To launch starknet devnet, use the command:

```bash
  starknet-devnet
```

## Creating and Deploying a New Account to Starknet Devnet

To create a new account, use (you can use sncast account create --help to see the available options):

1. **Create account**
```bash
  sncast --url http://localhost:5050/rpc account create --name new_account --class-hash  0x19...8dd6 --add-profile
```
Where the --clash-hash comes from the output of starknet-devnet
Note: --add-profile creates profile in snfoundry.toml file.
Example:
```bash
  Predeployed accounts using class with hash: 0x61dac032f228abef9c6626f995015233097ae253a7f72d68552db02f2971b8f
```

2. **Fund the account**
```bash
  curl -d '{"amount":8646000000000000, "address":"0x6e...eadf"}' -H "Content-Type: application/json" -X POST http://127.0.0.1:5050/mint
```
3. **Account deployment**
Deploy the account to the starknet devnet local node to register it with the chain:

```bash
  sncast --url http://localhost:5050/rpc account deploy --name new_account
```

## Declaring and Deploying Contract

To declare agreement contract and apply it follow the steps below.

1. **Navigate to contract module**
```bash
  cd src/agreement_version_2/
```

2. **Bulding the contract**
```bash
  scarb build
```

3. **Declaring the contract**
```bash
  ./1-declare.sh # Args <profile>
```

4. **Deploying the contract**
```bash
  ./2-deploy.sh # Args  <profile> <class_hash> <client_public_key> <server_public_key>
```

5. **Apply agreement to contract**
```bash
  ./3-apply # Args <profile> <contract_address> <quantity> <nonce> <price> <server_signature_r> <server_signature_s> <client_signature_r> <client_signature_s>
```
