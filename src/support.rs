/// A representação mais primitiva de um bloco de blockchain.
pub struct Block<Header, Extrinsic> {
    /// O cabeçalho do bloco contém metadados sobre o bloco.
    pub header: Header,
    /// Os extrínsecos representam as transições de estado a serem executadas neste bloco.
    pub extrinsics: Vec<Extrinsic>,
}

/// Estamos usando um cabeçalho extremamente simplificado que contém apenas o número atual do bloco.
/// Em uma blockchain real, você esperaria encontrar também:
/// - hash do bloco pai
/// - raiz do estado
/// - raiz dos extrínsecos
/// - etc...
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

/// Este é um "extrínseco": literalmente uma mensagem externa de fora da blockchain.
/// Esta versão simplificada de um extrínseco nos diz quem está fazendo a chamada e qual chamada eles estão fazendo.
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

/// O tipo Result para nosso runtime. Quando tudo é concluído com sucesso, retornamos `Ok(())`,
/// caso contrário, retornamos uma mensagem de erro estática.
pub type DispatchResult = Result<(), &'static str>;

/// Uma trait que nos permite despachar um extrínseco recebido para a chamada de função de transição de estado apropriada.
pub trait Dispatch {
    /// O tipo usado para identificar o chamador da função.
    type Caller;
    /// A chamada de função de transição de estado que o chamador está tentando acessar.
    type Call;

    /// Esta função recebe um `caller` e a `call` que eles querem fazer, e retorna um `Result`
    /// com base no resultado dessa chamada de função.
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}
