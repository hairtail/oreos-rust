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