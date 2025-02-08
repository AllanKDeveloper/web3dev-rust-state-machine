use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// O tipo que representa o conteúdo que pode ser reivindicado usando este pallet.
    /// Pode ser o conteúdo diretamente como bytes, ou melhor ainda, o hash desse conteúdo.
    /// Deixamos essa decisão para o desenvolvedor do runtime.
    type Content: Debug + Ord + Clone;
}

/// Este é o Módulo de Prova de Existência.
/// É um módulo simples que permite que contas reivindiquem a existência de alguns dados.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// Um simples mapa de armazenamento de conteúdo para o proprietário desse conteúdo.
    /// As contas podem fazer várias reivindicações diferentes, mas cada reivindicação só pode ter um proprietário.
    claims: BTreeMap<T::Content, T::AccountId>,
}

// Um enum público que descreve as chamadas que queremos expor ao despachante.
// Devemos esperar que o chamador de cada chamada seja fornecido pelo despachante,
// e não incluído como um parâmetro da chamada.
pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> Pallet<T> {
    /// Cria uma nova instância do Módulo de Prova de Existência.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Obtém o proprietário (se houver) de uma reivindicação.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    /// Cria uma nova reivindicação em nome do `caller`.
    /// Esta função retornará um erro se alguém já tiver reivindicado esse conteúdo.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err("This content is already claimed.");
        }

        self.claims.insert(claim.clone(), caller);
        Ok(())
    }

    /// Revoga uma reivindicação existente em algum conteúdo.
    /// Esta função só deve ter sucesso se o chamador for o proprietário de uma reivindicação existente.
    /// Retornará um erro se a reivindicação não existir ou se o chamador não for o proprietário.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.claims.get(&claim);
        match owner {
            Some(account) if account == &caller => {
                self.claims.remove(&claim);
                Ok(())
            }
            _ => Err("This claim is owned by someone else."),
        }
    }
}

/// Implementação da lógica de dispatch, mapeando de `POECall` para a função subjacente apropriada que queremos executar.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::system;

    struct TestConfig;
    impl super::Config for TestConfig {
        type Content = &'static str;
    }
    impl system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut pallet = Pallet::<TestConfig>::new();

        // Estado inicial
        assert_eq!(pallet.get_claim(&"conteudo"), None);

        // Criação de reivindicação
        let res = pallet.create_claim("alice", "conteudo");
        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_claim(&"conteudo"), Some(&"alice"));

        // Tentativa de criar reivindicação duplicada
        let res = pallet.create_claim("bob", "conteudo");
        assert_eq!(res, Err("This content is already claimed."));
        assert_eq!(pallet.get_claim(&"conteudo"), Some(&"alice"));

        // Revogação de reivindicação por proprietário
        let res = pallet.revoke_claim("alice", "conteudo");
        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_claim(&"conteudo"), None);

        // Tentativa de revogar reivindicação inexistente
        let res = pallet.revoke_claim("alice", "conteudo");
        assert_eq!(res, Err("This claim is owned by someone else."));

        // Criação de nova reivindicação
        let res = pallet.create_claim("bob", "outro conteudo");
        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_claim(&"outro conteudo"), Some(&"bob"));

        // Tentativa de revogar reivindicação por não proprietário
        let res = pallet.revoke_claim("alice", "outro conteudo");
        assert_eq!(res, Err("This claim is owned by someone else."));

        // Revogação de reivindicação por proprietário
        let res = pallet.revoke_claim("bob", "outro conteudo");
        assert_eq!(res, Ok(()));
        assert_eq!(pallet.get_claim(&"outro conteudo"), None);
    }
}
