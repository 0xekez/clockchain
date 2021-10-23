use std::io::Read;

use zeke_contract as zc;

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!(
            "usage: {} <path to solana program keypair> <bytecode>?",
            args[0]
        );
        std::process::exit(-1);
    }
    let keypair_path = &args[1];
    let bytecode = match args.len() {
        2 => {
            let mut buf = Vec::new();
            std::io::stdin().read_to_end(&mut buf)?;
            buf
        }
        3 => std::fs::read(&args[2])?,
        _ => {
            eprintln!(
                "usage: {} <path to solana program keypair> <bytecode>?",
                args[0]
            );
            std::process::exit(-1);
        }
    };

    let connection = zc::client::establish_connection().unwrap();

    let balance_requirement = zc::client::get_balance_requirement(&connection).unwrap();

    let player = zc::utils::get_player().unwrap();
    let player_balance = zc::client::get_player_balance(&player, &connection).unwrap();

    if player_balance < balance_requirement {
        let request = balance_requirement - player_balance;
        zc::client::request_airdrop(&player, &connection, request).unwrap();
    }

    let program = zc::client::get_program(keypair_path, &connection).unwrap();

    zc::client::create_result_account(&player, &program, &connection).unwrap();

    zc::client::execute_program(&player, &program, &bytecode, &connection).unwrap();
    println!(
        "{}",
        zc::client::count_greetings(&player, &program, &connection).unwrap()
    );
    Ok(())
}
