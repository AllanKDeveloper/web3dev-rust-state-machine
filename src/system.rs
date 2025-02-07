use std::collections::BTreeMap;

/// Este é o Pallet de Sistema.
/// Ele lida com o estado de baixo nível necessário para o blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// O número atual do bloco.
    block_number: u32,
    /// Um mapa de uma conta para seu nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    /// Cria uma nova instância do Pallet de Sistema.
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    /// Obtém o número atual do bloco.
    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    /// Esta função pode ser usada para incrementar o número do bloco.
    /// Aumenta o número do bloco em um.
    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    /// Incrementa o nonce de uma conta. Isso nos ajuda a acompanhar quantas transações cada conta fez.
    pub fn inc_nonce(&mut self, who: &String) {
        let current_nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), current_nonce + 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_system() {
        let mut system = Pallet::new();

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
