# Note:
- To make work you need to configuare the `Anchor.toml` file
    - In `Anchor.toml` there is the wallet attribute which is need to configure with your write keypair path
    - Also note that if you want to run the smart contract on `localnet` then its also need to configure.
    - After the deploing the smart contract its gives the public key of the deployed smart contract so make sure that you have this public is same in 
        * `Anchor.toml` -> program = "public key"
        * if it's not then correct it. 