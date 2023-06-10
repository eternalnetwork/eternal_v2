use std::process;

use eternal_core::{
    block::Block,
    blockchain::Blockchain,
    transaction::{Transaction, TransactionData},
};

use eternal_vm::smart_contract::{SmartContract, SmartContractApi, SmartContractStanderd};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() {
    let mut bc = Blockchain::new();
    run_blockchain_actions(&mut bc);

    println!("{}", serde_json::to_string(&bc).unwrap().as_str());
}

fn run_blockchain_actions(bc: &mut Blockchain) {
    let (bob, alice) = bc.get_bob_alice();

    // Block 1
    let mut block = Block::new(None);
    {
        block.add_transaction(Transaction::new(
            bob.clone(),
            TransactionData::MintTokens {
                receiver: bob.clone(),
                amount: 100,
            },
            10,
        ));
        block.add_transaction(Transaction::new(
            bob.clone(),
            TransactionData::Transfer {
                to: alice.clone(),
                amount: 1000,
            },
            10,
        ));
        block.add_transaction(Transaction::new(
            alice.clone(),
            TransactionData::Transfer {
                to: bob.clone(),
                amount: 999,
            },
            10,
        ));
    }
    bc.append_block(block.clone()).unwrap();

    // Block 2
    let mut block = Block::new(block.hash.clone());
    {
        block.add_transaction(Transaction::new(
            alice.clone(),
            TransactionData::CreateUserAccount,
            10,
        ));

        let sc: SmartContract = smart_contract();

        block.add_transaction(Transaction::new(
            alice.clone(),
            TransactionData::DeploySmartContract {
                publisher: alice.clone(),
                sc: Some(sc),
            },
            10,
        ));
    }
    bc.append_block(block.clone()).unwrap();

    println!("{}", bc.temp[4]);

    let mut block = Block::new(block.hash.clone());
    {
        let (token, amount, to) = get_input();
        block.add_transaction(Transaction::new(
            alice,
            TransactionData::TransferToken {
                token,
                to,
                amount: amount.parse().unwrap(),
            },
            10,
        ));
    }
    bc.append_block(block.clone()).unwrap();
}

fn get_input() -> (String, String, String) {
    let mut rl = DefaultEditor::new().unwrap();
    let token = match rl.readline("Token: ") {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => process::exit(1),
        Err(ReadlineError::Eof) => process::exit(1),
        Err(err) => {
            eprintln!("{:#?}", err);
            process::exit(1)
        }
    };
    let amount = rl.readline("Amount: ").unwrap();
    let to = rl.readline("To: ").unwrap();

    (token, amount, to)
}

fn smart_contract() -> SmartContract {
    SmartContract::new(
        SmartContractStanderd::from("ESC20"),
        SmartContractApi::ESC20 {
            transfer: |from, to, amount| Ok(println!("{} =Transfered {}> {}", from, amount, to)),
            publisher: String::new(),
            total_suply: 100_000,
        },
    )
}
