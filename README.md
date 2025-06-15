<h3>Transaction for Ethereum or Bitcoin</h3>

* sign the transaction offline

<h3> Requirement </h3>
* Validation of signed transaction with testnet or mainnet 


<h3>usage</h3>
in Sign_transaction run

<h3> test in polygon test net(chain ID: 80002, url = https://rpc-amoy.polygon.technology)</h3>
1. transfer without data
```
cargo run -- --to 0x7EdD8b438Ba9Fcf460C9756231B98994bD61b2F6 --chain 80002 --gas 22_000 --gas_price 350_000_000_000 --value 1_000_000_000_000_000 --nonce 4 --pk <PK>
```
output:
```
0xf86e0485517da02c008255f0947edd8b438ba9fcf460c9756231b98994bd61b2f687038d7ea4c680008083027127a0c8d80aaf77005cc7f8188ff951b13af4dfc90ba456c93f8faec562959c2e6687a05e6d9dc50b755352660e2259b41108e0c48d0b3e654a743acd89bb7e0d44daa5
```

2. transfer with data: help
```
cargo run -- --to 0x7EdD8b438Ba9Fcf460C9756231B98994bD61b2F6 --chain 80002 --gas 22_000 --gas_price 350_000_000_000 --value 1_000_000_000_000_000 --nonce 5 --data help --pk <PK>
```
output:
```
0xf86e0485517da02c008255f0947edd8b438ba9fcf460c9756231b98994bd61b2f687038d7ea4c680008083027127a0c8d80aaf77005cc7f8188ff951b13af4dfc90ba456c93f8faec562959c2e6687a05e6d9dc50b755352660e2259b41108e0c48d0b3e654a743acd89bb7e0d44daa5 
```

3. transfer with decimal data: 0x12345
```
cargo run -- --to 0x7EdD8b438Ba9Fcf460C9756231B98994bD61b2F6 --chain 80002 --gas 22_000 --gas_price 350_000_000_000 --value 1_000_000_000_000_000 --nonce 6 --date 12345 --pk <PK>
```
output:
```
0xf86e0485517da02c008255f0947edd8b438ba9fcf460c9756231b98994bd61b2f687038d7ea4c680008083027127a0c8d80aaf77005cc7f8188ff951b13af4dfc90ba456c93f8faec562959c2e6687a05e6d9dc50b755352660e2259b41108e0c48d0b3e654a743acd89bb7e0d44daa5
```

<h3>mix json file and arugments</h3>
1. create info.json
{
  "to": "0x7EdD8b438Ba9Fcf460C9756231B98994bD61b2F6",
  "chain": "80002",
  "gas": "22_000",
  "gas_price": "350_000_000_000",
  "value": "1_000_000_000_000_000",
  "pk": "<PK>"
}

2. execute command
```
cargo run -- --json info.json --data 12345 --nonce 6
```

<h3> test the outputs</h3>
1. goto Simple_Commit_Transaction
2. execute
```
cargo run -- https://rpc-amoy.polygon.technology 0xf86e0485517da02c008255f0947edd8b438ba9fcf460c9756231b98994bd61b2f687038d7ea4c680008083027127a0c8d80aaf77005cc7f8188ff951b13af4dfc90ba456c93f8faec562959c2e6687a05e6d9dc50b755352660e2259b41108e0c48d0b3e654a743acd89bb7e0d44daa5
```
