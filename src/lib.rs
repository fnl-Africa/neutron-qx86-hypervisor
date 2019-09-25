
extern crate struct_deser;
#[macro_use]
extern crate struct_deser_derive;
pub mod hypervisor;


#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct NeutronAddress{
    pub version: u32,
    pub data: Vec<u8>
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct NeutronVMResult{
    pub gas_used: u64,
    pub should_revert: bool,
    pub error_code: u32,
    pub error_location: u64,
    pub extra_data: u64
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct NeutronContext{
    pub exec: ExecContext,
    pub tx: TransactionContext,
    pub block: BlockContext,
    pub internal: usize
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ExecContext{
    pub flags: u64,
    pub sender: NeutronAddress,
    pub gas_limit: u64,
    pub value_sent: u64,
    pub origin: NeutronAddress,
    pub self_address: NeutronAddress,
    pub nest_level: u32
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TransactionContext{
    pub inputs: Vec<TxItem>,
    pub outputs: Vec<TxItem>
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct TxItem{
    pub sender: NeutronAddress,
    pub value: u64
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BlockContext{
    pub creator: NeutronAddress,
    pub gas_limit: u64,
    pub difficulty: u64,
    pub height: u32,
    pub previous_time: u64,
    pub previous_hashes: Vec<[u8; 32]>
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeutronError{
    Success,
    RecoverableFailure,
    UnrecoverableFailure
}

/*
typedef struct{
    uint8_t format;
    uint8_t rootVM;
    uint8_t vmVersion;
    uint16_t flagOptions;
    uint32_t qtumVersion;
} NeutronVersion;
*/
#[derive(StructDeser, Debug, Eq, PartialEq, Default)]
pub struct NeutronVersion{
    pub format: u8,
    pub root_vm: u8,
    pub vm_version: u8,
    #[le]
    pub flags: u16,
    #[le]
    pub qtum_version: u32
}



/// This is the primary NeutronAPI interface. It is loosely based on the C Neutron API, but uses Rust paradigms and features
/// This will require a heavier translation layer, but makes Rust usage significantly simpler
pub trait NeutronAPI{
    fn get_context(&self) -> &NeutronContext;
    fn push_sccs(&mut self, data: &Vec<u8>) -> Result<(), NeutronError>;
    fn pop_sccs(&mut self, data: &mut Vec<u8>) -> Result<(), NeutronError>;
    fn pop_sccs_toss(&mut self) -> Result<(), NeutronError>; //returns no data, for throwing away the item
    fn peek_sccs(&mut self, data: &mut Vec<u8>) -> Result<(), NeutronError>;
    fn peek_sccs_size(&mut self) -> Result<usize, NeutronError>;

    fn log_error(&mut self, msg: &str);
    fn log_info(&mut self, msg: &str);
    fn log_debug(&mut self, msg: &str);
}

