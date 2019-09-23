
struct NeutronAddress{
    pub version: u32,
    pub data: Vec<u8>
}
struct NeutronVMResult{
    pub gasUsed: u64,
    pub shouldRevert: bool,
    pub errorCode: u32,
    pub errorLocation: u64,
    pub extraData: u64
}

struct NeutronContext{
    pub exec: ExecContext,
    pub tx: TransactionContext,
    pub block: BlockContext,
    pub internal: usize
}

struct ExecContext{
    pub flags: u64,
    pub sender: NeutronAddress,
    pub gasLimit: u64,
    pub valueSent: u64,
    pub origin: NeutronAddress,
    pub selfAddress: NeutronAddress,
    pub nestLevel: u32
}
struct TransactionContext{
    pub inputs: Vec<TxItem>,
    pub outputs: Vec<TxItem>
}
struct TxItem{
    pub sender: NeutronAddress,
    pub value: u64
}

struct BlockContext{
    pub creator: NeutronAddress,
    pub gasLimit: u64,
    pub difficulty: u64,
    pub height: u32,
    pub previousTime: u64,
    pub previousHashes: Vec<[u8; 32]>
}

trait NeutronAPI{

}

