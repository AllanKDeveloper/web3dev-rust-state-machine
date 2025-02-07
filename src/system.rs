use core::ops::AddAssign;
use num::traits::{One, Zero};
use std::collections::BTreeMap;

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: One + Zero + AddAssign + Copy;
    type Nonce: One + Zero + Copy;
}

/// Este é o Pallet do Sistema.
/// Ele lida com o estado de baixo nível necessário para o blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// O número do bloco atual.
    block_number: T::BlockNumber,
    /// Um mapa de uma conta até seu nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

// Atualize todas essas funções para usar seu novo traço de configuração.
impl<T: Config> Pallet<T> {
    /// Cria uma nova instância do Pallet de Sistema.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Obtém o número atual do bloco.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    /// Esta função pode ser usada para incrementar o número do bloco.
    /// Aumenta o número do bloco em um.
    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    /// Incrementa o nonce de uma conta. Isso nos ajuda a acompanhar quantas transações cada conta fez.
    pub fn inc_nonce(&mut self, account: &T::AccountId) {
        let current_nonce = *self.nonce.get(account).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(account.clone(), current_nonce);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();

        // Verifica se o número do bloco inicial é 0
        assert_eq!(system.block_number(), 0);

        // Verifica se o nonce de dev0 é 0 (ou seja, não foi definido)
        assert_eq!(*system.nonce.get(&"dev0".to_string()).unwrap_or(&0), 0);

        system.inc_block_number();
        system.inc_nonce(&"dev0".to_string());

        assert_eq!(system.block_number(), 1);
        assert_eq!(*system.nonce.get(&"dev0".to_string()).unwrap_or(&0), 1);

        system.inc_block_number();
        system.inc_nonce(&"dev0".to_string());
        system.inc_nonce(&"dev1".to_string());

        assert_eq!(system.block_number(), 2);
        assert_eq!(*system.nonce.get(&"dev0".to_string()).unwrap_or(&0), 2);
        assert_eq!(*system.nonce.get(&"dev1".to_string()).unwrap_or(&0), 1);
    }
}
