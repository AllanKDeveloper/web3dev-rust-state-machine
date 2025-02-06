use std::collections::BTreeMap;

#[derive(Default)]
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Define o saldo de um usuário.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    /// Obtém o saldo de um usuário.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    /// Transfere `amount` de uma conta para outra.
    /// Esta função verifica se `caller` tem pelo menos `amount` de saldo para transferir
    /// e impede que ocorram overflow/underflow matemáticos.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        // Obtém o saldo da conta `caller`
        let caller_balance = self.balance(&caller);

        // Verifica se o `caller` tem saldo suficiente
        if caller_balance < amount {
            return Err("Insufficient balance");
        }

        // Obtém o saldo da conta `to`
        let to_balance = self.balance(&to);

        // Calcula os novos saldos usando matemática segura
        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Underflow when debiting the sender's account")?;
        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when crediting the recipient's account")?;

        // Atualiza os saldos
        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&"dev0".to_string()), 0);
        balances.set_balance(&"dev0".to_string(), 100);
        assert_eq!(balances.balance(&"dev0".to_string()), 100);
        assert_eq!(balances.balance(&"dev1".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::default();

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
