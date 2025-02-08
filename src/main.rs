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
    pub type Content = String;
}

// Estas são todas as chamadas que estão expostas ao mundo.
// Observe que é apenas um acúmulo das chamadas expostas por cada módulo.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// Este é o nosso Runtime principal.
// Acumula todos os diferentes pallets que queremos utilizar.
#[derive(Debug)]
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

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
            proof_of_existence: proof_of_existence::Pallet::new(),
        }
    }

    // Executa um bloco de extrínsecos. Incrementa o número do bloco.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        // 1. Incrementar o número do bloco do sistema.
        self.system.inc_block_number();

        // 2. Verifique se o número do bloco de entrada corresponde ao número do bloco atual,
        // ou retornar um erro.
        if block.header.block_number != self.system.block_number() {
            return Err("Block number mismatch");
        }

        // 3. Iterar sobre os extrínsecos do bloco...
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.iter().enumerate() {
            self.system.inc_nonce(&caller);

            let _res = self.dispatch(caller.clone(), call).map_err(|e| {
                format!(
                    "Error in block {}: extrinsic {}: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;

    // Despacha uma chamada em nome de um chamador. Aumenta o nonce do chamador.
    //
    // Dispatch nos permite identificar qual chamada de módulo subjacente queremos executar.
    // Observe que extraímos o `chamador` do extrínseco e usamos essa informação
    // para determinar em nome de quem estamos executando a chamada.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => self.balances.dispatch(caller, call),
            RuntimeCall::ProofOfExistence(call) => self.proof_of_existence.dispatch(caller, call),
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

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::Balances(balances::Call::Transfer {
                    to: dev1,
                    amount: 20,
                }),
            },
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "oi".to_string(),
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
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::CreateClaim {
                    claim: "tchau".to_string(),
                }),
            },
            support::Extrinsic {
                caller: dev0.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::RevokeClaim {
                    claim: "oi".to_string(),
                }),
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block 1");
    runtime.execute_block(block_2).expect("invalid block 2");

    println!("{:?}", runtime);
}
