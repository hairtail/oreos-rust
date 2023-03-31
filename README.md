## 1. Overview

oreo-rust is a small tool for ironfish account creation, recovery (from `spendingKey`, from `mnemonic`), `transaction` decryption and `transaction causal send`.

## 2. Build Guide

Before beginning, please ensure your machine has `Rust` installed. Instructions to [install Rust can be found here.](https://www.rust-lang.org/tools/install)


Start by cloning this Github repository:
```
git clone https://github.com/hairtail/oreos-rust.git --depth 1
```

Next, move into the `oreos-rust` directory:
```
cd oreos-rust
```

Then, install necessary dependency:
```
./install_dep.sh
```

Lastly, 

```
cargo build --release
```

## 3. Account


### 3.1 Create

```
./target/release/oreos account new
     Mnemonic  invest trap equip course sweet crack slot youth once deposit enforce gas mammal teach latin cherry laugh you copy tattoo real fame wealth top
 Spending Key  75fce93098adba63b307fb9a876128b0086bbd1f593b7d9fe4c06f2b2ea4fe0f
     View Key  7b595402b5fbe834a0b4b240c1616d3a803879fb0a6376fc9c517497e6ef61bd460a6e8586ab2583080a811500d8d641f0108b1f99f54c18d7cbf32f896b60e3
  Incoming View Key  a3aaca44fca9395e103d6b2814d1c7e0b3ccea1d03027a12b1c63a51a4276207
  Outgoing View Key  5a89f5654eae9a9c9498e25697cd32f262006575ef6625f81215802be943c3b0
      Address  478eb2d09f6c0362673ceaa38416cd727b43ae11716a4bb9bd01cc5522abbba8
```

### 3.2 Recover with SpendingKey

```
./target/release/oreos account new --key 75fce93098adba63b307fb9a876128b0086bbd1f593b7d9fe4c06f2b2ea4fe0f
     Mnemonic  invest trap equip course sweet crack slot youth once deposit enforce gas mammal teach latin cherry laugh you copy tattoo real fame wealth top
 Spending Key  75fce93098adba63b307fb9a876128b0086bbd1f593b7d9fe4c06f2b2ea4fe0f
     View Key  7b595402b5fbe834a0b4b240c1616d3a803879fb0a6376fc9c517497e6ef61bd460a6e8586ab2583080a811500d8d641f0108b1f99f54c18d7cbf32f896b60e3
  Incoming View Key  a3aaca44fca9395e103d6b2814d1c7e0b3ccea1d03027a12b1c63a51a4276207
  Outgoing View Key  5a89f5654eae9a9c9498e25697cd32f262006575ef6625f81215802be943c3b0
      Address  478eb2d09f6c0362673ceaa38416cd727b43ae11716a4bb9bd01cc5522abbba8
```

### 3.3 Recover with Mnemonic

```
./target/release/oreos account new --mnemonic "invest trap equip course sweet crack slot youth once deposit enforce gas mammal teach latin cherry laugh you copy tattoo real fame wealth top"
     Mnemonic  invest trap equip course sweet crack slot youth once deposit enforce gas mammal teach latin cherry laugh you copy tattoo real fame wealth top
 Spending Key  75fce93098adba63b307fb9a876128b0086bbd1f593b7d9fe4c06f2b2ea4fe0f
     View Key  7b595402b5fbe834a0b4b240c1616d3a803879fb0a6376fc9c517497e6ef61bd460a6e8586ab2583080a811500d8d641f0108b1f99f54c18d7cbf32f896b60e3
  Incoming View Key  a3aaca44fca9395e103d6b2814d1c7e0b3ccea1d03027a12b1c63a51a4276207
  Outgoing View Key  5a89f5654eae9a9c9498e25697cd32f262006575ef6625f81215802be943c3b0
      Address  478eb2d09f6c0362673ceaa38416cd727b43ae11716a4bb9bd01cc5522abbba8
```

## 4. Transaction

### 4.1 Transaction Decryption
Transaction decryption with `transactionHash`, `incomingViewKey`, `outgoingViewKey`. A full synced Ironfish rpc endpoint is needed (http).
- Get transaction info (blockHash) with oreoscan.info api
- Get raw transaction info (encrypted note) with endpoint (Ironfish rpc)
- Decrypt all encrypted note locally

Known AssetId

`d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6` --  Ironfish Native Token

```
./target/release/oreos transaction decrypt --hash <HASH> --incoming-viewkey <INCOMING_VIEWKEY> --outgoing-viewkey <OUTGOING_VIEWKEY> --endpoint <ENDPOINT>
// Sender address
Sender: 123xsdawegjkljsd
// Receiver: receiver address, value in ore, asset id, memo
Receiver: xxx, 100, d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6, hello
Receiver: xxx, 100, d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6, hello
```

### 4.2 What Causal Send Means
`Alice` received 100 coins from `Bob` in transactionX, `Alice` exactly knows that she has never spent this 100 coins. Then she can send the received coins from transactionX to `Amy`. So she can create a transaction locally without a fully syncd ironfish node, and post this transaction via a public rpc. The ironfish blockchain consensus is responsible for the transaction validation (no double spend).

### 4.3 Transaction Causal Send

- Get transaction info (blockHash) with oreoscan.info api
- Get raw transaction info (encrypted note) with endpoint (Ironfish rpc)
- Decrypt all encrypted note locally
- Transaction creation and signing locally
- Transaction broadcast via rpc

```
./target/release/oreos transaction send --hash <HASH> --incoming-viewkey <INCOMING_VIEWKEY> --outgoing-viewkey <OUTGOING_VIEWKEY> --spending-key <SPENDING_KEY> --endpoint <ENDPOINT> --receiver <RECEIVER> --amount <AMOUNT> --expiration <EXPIRATION> --memo <MEMO>
```
