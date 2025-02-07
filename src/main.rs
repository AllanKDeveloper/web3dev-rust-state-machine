mod balances;
mod system;

mod types {
    // Definições de tipo para AccountId e Balance.
    pub type AccountId = String;
    pub type Balance = u128;
    // Definições de tipo para `BlockNumber` e `Nonce`.
    pub type BlockNumber = u32;
    pub type Nonce = u32;
}

// Este é o nosso Runtime principal.
// Acumula todos os diferentes pallets que queremos utilizar.
#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
}

// Implementação da característica `system::Config` para o `Runtime`.
impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

// Implementação da característica `balances::Config` para o `Runtime`.
impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();

    // Variáveis para as contas
    let dev0 = "dev0".to_string();
    let dev1 = "dev1".to_string();
    let azuki = "azuki".to_string();

    // Usando as variáveis
    runtime.balances.set_balance(&dev0, 100);

    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&dev0);
    let _res = runtime
        .balances
        .transfer(dev0.clone(), dev1, 30) // Clonando dev0 para a transferência
        .map_err(|e| println!("Error on transfer: {}", e));

    runtime.system.inc_nonce(&dev0);
    let _res = runtime
        .balances
        .transfer(dev0, azuki, 20) // dev0 é movido aqui
        .map_err(|e| println!("Error on transfer: {}", e));

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

    println!("{:?}", runtime);
}
