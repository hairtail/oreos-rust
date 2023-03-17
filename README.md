## 1. Overview

oreo-rust is a small tool for ironfish account creation, recovery (from `spendingKey`, from `mnemonic`), and `encryptedNote` decryption.

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

Lastly, 

```
cargo build --release
```

## 3. Account


### 3.1 Create

```
./target/release/oreos create
     Mnemonic  xxx xxx xxx
 Spending Key  xxxxxxx
     View Key  xxxxx
  Incoming View Key  xxx
  Outgoing View Key  xxx
      Address  xxx
```

### 3.2 Recover with SpendingKey

```
./target/release/oreos recover --data xxxx
     Mnemonic  xxx xxx xxx
 Spending Key  xxxxxxx
     View Key  xxxxx
  Incoming View Key  xxx
  Outgoing View Key  xxx
      Address  xxx
```

### 3.2 Recover with Mnemonic

```
./target/release/oreos recover --data "xxx xxx ... xxx"
     Mnemonic  xxx xxx xxx
 Spending Key  xxxxxxx
     View Key  xxxxx
  Incoming View Key  xxx
  Outgoing View Key  xxx
      Address  xxx
```

## 4. Encrypted Note

Known AssetId

`d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6` --  Ironfish Native Token

### 4.1 Decrypt with IncomingViewKey and OutgoingViewKey

```
./target/release/oreos decrypt --data "encrypted note(hex encoded)" -i "incoimingViewKey" -o "outgoingViewKey"
       Sender  xxx
     Receiver  xxx
        Value  1000
       AssetId  d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6
         Memo  xx
```


## 5. Transaction
Transaction decryption with `transactionHash`, `incomingViewKey`, `outgoingViewKey`. A full synced Ironfish rpc endpoint is needed (http).
### 5.1 How?
- Get transaction info (blockHash) with oreoscan.info api
- Get raw transaction info (encrypted note) with endpoint (Ironfish rpc)
- Decrypt all encrypted note locally as `oreos decrypt`

Known AssetId

`d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6` --  Ironfish Native Token

```
./target/release/oreos watch -i xxx -o xxx --endpoint "127.0.0.1:8021" --hash xxx
// Sender address
Sender: 123xsdawegjkljsd
// Receiver: receiver address, value in ore, asset id, memo
Receiver: xxx, 100, d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6, hello
Receiver: xxx, 100, d7c86706f5817aa718cd1cfad03233bcd64a7789fd9422d3b17af6823a7e6ac6, hello
```

