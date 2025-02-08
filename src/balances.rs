use num::traits::{CheckedAdd, CheckedSub, Zero};
use std::collections::BTreeMap;

/// A característica de configuração do Módulo Balances.
/// Contém os tipos básicos necessários para lidar com saldos.
pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Ord;
}

/// Este é o Módulo de Saldos.
/// É um módulo simples que monitora quanto saldo cada conta tem nesta máquina de estados.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// Um mapeamento simples de armazenamento de contas para seus saldos.
    balances: BTreeMap<T::AccountId, T::Balance>,
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfere `amount` de uma conta para outra.
    /// Esta função verifica se `caller` tem pelo menos `amount` de saldo para transferir
    /// e impede que ocorram overflow/underflow matemáticos.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> crate::support::DispatchResult {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Insufficient balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Define o saldo de um utilizador.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        self.balances.insert(who.clone(), amount);
    }

    /// Obtém o saldo de um utilizador.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::system;

    struct TestConfig;

    impl Config for TestConfig {
        type Balance = u128;
    }

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        // Instancia Pallet usando TestConfig
        let mut balances = Pallet::<TestConfig>::new();

        assert_eq!(balances.balance(&"dev0".to_string()), 0);
        balances.set_balance(&"dev0".to_string(), 100);
        assert_eq!(balances.balance(&"dev0".to_string()), 100);
        assert_eq!(balances.balance(&"dev1".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        // Instancia Pallet usando TestConfig
        let mut balances = Pallet::<TestConfig>::new();

        // Inicializa os saldos de dev0 e dev1
        balances.set_balance(&"dev0".to_string(), 100);
        balances.set_balance(&"dev1".to_string(), 50);

        // Tenta uma transferência inválida (saldo insuficiente) e verifica o erro esperado
        assert_eq!(
            balances.transfer("dev0".to_string(), "dev1".to_string(), 150),
            Err("Insufficient balance")
        );

        // Realiza uma transferência válida e verifica o sucesso
        assert!(balances
            .transfer("dev0".to_string(), "dev1".to_string(), 30)
            .is_ok());

        // Verifica os saldos após a transferência bem-sucedida
        assert_eq!(balances.balance(&"dev0".to_string()), 70); // 100 - 30
        assert_eq!(balances.balance(&"dev1".to_string()), 80); // 50 + 30
    }
}
