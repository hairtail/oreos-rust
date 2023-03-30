use clap::Parser;

#[derive(Debug, Parser, Clone)]
#[clap(name = "oreos", author = "hairtail")]
#[clap(author, version, about, long_about = None)]
pub struct CLI {
    /// Specify a subcommand.
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser, Clone)]
pub enum Command {
    #[clap(subcommand)]
    Account(Account),
    #[clap(subcommand)]
    Transaction(Transaction),
}

/// Commands to manage Ironfish accounts.
#[derive(Clone, Debug, Parser)]
pub enum Account {
    /// Generates a new Ironfish account
    New {
        /// Generates from a mnemonic
        #[clap(long)]
        mnemonic: Option<String>,
        /// Specifys mnemonic language
        #[clap(short, long, default_value_t=String::from("en"))]
        language: String,
        /// Generates from a spendingKey
        #[clap(long)]
        key: Option<String>,
    },
}

/// Commands to manage Ironfish transactions.
#[derive(Clone, Debug, Parser)]
pub enum Transaction {
    /// Decrypts an Ironfish transaction
    Decrypt {
        /// Transaction hash
        #[clap(long)]
        hash: String,
        /// Hex encoded account incoming view key
        #[clap(short, long)]
        incoming_viewkey: String,
        /// Hex encoded account outgoing view key
        #[clap(short, long)]
        outgoing_viewkey: String,
        /// Rpc endpoint (http)
        #[clap(long)]
        endpoint: String,
    },
    /// Ironfish transaction causal send
    Send {
        /// Received transaction hash
        #[clap(long)]
        hash: String,
        /// Hex encoded account incoming view key
        #[clap(short, long)]
        incoming_viewkey: String,
        /// Hex encoded account outgoing view key
        #[clap(short, long)]
        outgoing_viewkey: String,
        /// Hex encoded account spending key
        #[clap(short, long)]
        spending_key: String,
        /// Rpc endpoint (http)
        #[clap(long)]
        endpoint: String,
        /// Receiver address
        #[clap(long)]
        receiver: String,
        /// Amount sent to receiver
        #[clap(long)]
        amount: u64,
        /// Gas fee for this transaction
        #[clap(long, default_value_t = 1u64)]
        fee: u64,
        /// Expiration sequence for this transaction
        #[clap(long)]
        expiration: u32,
        /// Memo in transaction
        #[clap(long, default_value_t=String::from("oreos-causal-send"))]
        memo: String,
    },
}
