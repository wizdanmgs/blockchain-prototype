mod block;
mod blockchain;
mod hash;
mod pow;
mod transaction;

use blockchain::Blockchain;
use transaction::Transaction;

fn main() {
    let mut blockchain = Blockchain::new(4);

    let tx1 = Transaction {
        from: "Alice".into(),
        to: "Bob".into(),
        amount: 50,
    };

    let tx2 = Transaction {
        from: "Bob".into(),
        to: "Charlie".into(),
        amount: 25,
    };

    blockchain.add_block(vec![tx1, tx2]);

    println!("Blockchain valid: {}", blockchain.is_valid());
}
