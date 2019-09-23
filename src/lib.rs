
pub struct NeutronAddress{
    pub version: u32,
    pub data: Vec<u8>
}
pub struct NeutronVMResult{
    pub gas_used: u64,
    pub should_revert: bool,
    pub error_code: u32,
    pub error_location: u64,
    pub extra_data: u64
}

pub struct NeutronContext{
    pub exec: ExecContext,
    pub tx: TransactionContext,
    pub block: BlockContext,
    pub internal: usize
}

pub struct ExecContext{
    pub flags: u64,
    pub sender: NeutronAddress,
    pub gas_limit: u64,
    pub value_sent: u64,
    pub origin: NeutronAddress,
    pub self_address: NeutronAddress,
    pub nest_level: u32
}
pub struct TransactionContext{
    pub inputs: Vec<TxItem>,
    pub outputs: Vec<TxItem>
}
pub struct TxItem{
    pub sender: NeutronAddress,
    pub value: u64
}

pub struct BlockContext{
    pub creator: NeutronAddress,
    pub gas_limit: u64,
    pub difficulty: u64,
    pub height: u32,
    pub previous_time: u64,
    pub previous_hashes: Vec<[u8; 32]>
}

trait NeutronAPI{

}

