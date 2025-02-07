mod balances;
mod system;
use std::fmt::Debug;

// Este é o nosso Runtime principal.
// Ele acumula todos os diferentes pallets que queremos usar.
#[derive(Debug)]
pub struct Runtime {
    pub system: system::Pallet,
    pub balances: balances::Pallet,
}

impl Runtime {
    // Cria uma nova instância do Runtime principal, criando uma nova instância de cada pallet.
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    // Cria uma variável mutável `runtime`, que é uma nova instância de `Runtime`.
    let mut runtime = Runtime::new();

    // Define o saldo de `dev0` para 100, permitindo-nos executar outras transações.
    runtime.balances.set_balance(&"dev0".to_string(), 100);

    // Começa a emular um bloco
    // Aumenta o número do bloco no sistema.
    runtime.system.inc_block_number();

    // Afirmar que o número do bloco é o que esperamos.
    assert_eq!(runtime.system.block_number(), 1);

    // Primeira transação
    // Aumenta o nonce de `dev0`.
    runtime.system.inc_nonce(&"dev0".to_string());

    // Executa uma transferência de `dev0` para `dev1` por 30 tokens.
    let _res = runtime
        .balances
        .transfer("dev0".to_string(), "dev1".to_string(), 30)
        .map_err(|e| println!("Error on transfer: {}", e));

    // Segunda transação
    // Aumenta o nonce de `dev0` novamente.
    runtime.system.inc_nonce(&"dev0".to_string());

    // Executa outra transferência de saldo, desta vez de `dev0` para `azuki` por 20.
    let _res = runtime
        .balances
        .transfer("dev0".to_string(), "azuki".to_string(), 20)
        .map_err(|e| println!("Error on transfer: {}", e));

    // Imprime os saldos para verificar o resultado das transações
    println!(
        "Debit balance of dev0: {}",
        runtime.balances.balance(&"dev0".to_string())
    );
    println!(
        "Debit balance of dev1: {}",
        runtime.balances.balance(&"dev1".to_string())
    );
    println!(
        "Debit balance of azuki: {}",
        runtime.balances.balance(&"azuki".to_string())
    );

    // Imprime o estado final do tempo de execução após todas as transações.
    println!("{:?}", runtime);
}
