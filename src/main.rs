mod balances;
mod proof_of_existence;
mod support;
mod system;

use crate::support::Dispatch;

mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
    pub type Content = &'static str;
}

// Este é o nosso Runtime principal.
// Acumula todos os diferentes pallets que queremos utilizar.
#[derive(Debug)]
#[macros::runtime]
pub struct Runtime {
    system: system::Pallet<Self>,
    balances: balances::Pallet<Self>,
    proof_of_existence: proof_of_existence::Pallet<Self>,
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

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

fn main() {
    let mut runtime = Runtime::new();

    // Variáveis para as contas
    let dev0 = "dev0".to_string();
    let dev1 = "dev1".to_string();
    let azuki = "azuki".to_string();

    // Usando as variáveis
    runtime.balances.set_balance(&dev0, 100);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::balances(balances::Call::transfer {
                    to: dev1,
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "oi",
                }),
            },
        ],
    };

    // Bloco 2
    let block_2 = types::Block {
        header: types::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: azuki.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "tchau",
                }),
            },
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "oi",
                }),
            },
        ],
    };

    // Bloco 3
    let block_3 = types::Block {
        header: types::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::revoke_claim {
                    claim: "Hello, world!",
                }),
            },
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::proof_of_existence(proof_of_existence::Call::create_claim {
                    claim: "Hello, world!",
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block 1");
    runtime.execute_block(block_2).expect("invalid block 2");
    runtime.execute_block(block_3).expect("invalid block 3");

    println!("{:?}", runtime);
}
