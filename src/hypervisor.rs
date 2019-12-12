extern crate qx86;
extern crate neutron_star_constants;
extern crate num;
use qx86::vm::*;
use neutron_interface::*;
use neutron_star_constants::*;
use std::convert::TryFrom;

impl Hypervisor for dyn NeutronHypervisor{
    fn interrupt(&mut self, vm: &mut VM, num: u8) -> Result<(), VMError>{
        use NeutronSyscalls::*;
        if num == EXIT_INTERRUPT{
            self.log_debug("Exit interrupt triggered\n");
            return Err(VMError::InternalVMStop);
        }
        if num != NEUTRON_INTERRUPT{
            self.log_error("Invalid interrupt triggered");
            return Ok(());
        }
        let ctx = self.get_context();
        let syscall = NeutronSyscalls::try_from(vm.reg32(Reg32::EAX)); // FromPrimitive::from_u32(vm.reg32(Reg32::EAX));
        if syscall.is_err(){
            self.log_error("Invalid syscall triggered");
            return Ok(());
        }
        let syscall = syscall.unwrap();

        let status = match syscall{
            IsCreate => {
                if ctx.exec.flags & 0x01 > 0{
                    1
                }else{
                    0
                }
            },
            PushSCCS => {
                //data, length
                let data = vm.copy_from_memory(vm.reg32(Reg32::EBX), vm.reg32(Reg32::ECX))?;
                if self.push_sccs(&data).is_err(){
                    return Err(VMError::NotYetImplemented)
                }else{
                    0
                }
            },
            PopSCCS => {
                //data, max_size, &actual_size
                let mut data = vec![];
                let res = self.pop_sccs(&mut data);
                if res.is_err(){
                    return Err(VMError::NotYetImplemented);
                }
                let len = std::cmp::min(vm.reg32(Reg32::ECX), data.len() as u32);
                let s = &data[0..len as usize];
                vm.copy_into_memory(vm.reg32(Reg32::EBX), s)?;
                len
            },
            GasUsed => {
                //&gas_used
                let used = self.get_context().exec.gas_limit - vm.gas_remaining;
                //todo: 64bit needed here? 
                used as u32
            }
            _ => {
                self.log_error("Unimplemented syscall!");
                0
            }
        };
        vm.set_reg32(Reg32::EAX, status);

        Ok(())
    }
}

pub trait NeutronHypervisor : NeutronAPI{
    fn init_cpu(&mut self, vm: &mut VM) -> Result<(), VMError>{
        self.init_memory(vm)?;
        vm.gas_remaining = self.get_context().exec.gas_limit;
        vm.eip = 0x10000;
        Ok(())
    }
    fn init_memory(&mut self, vm: &mut VM) -> Result<(), VMError>{
        //for now, just make all memories max size
        //code memories
        vm.memory.add_memory(0x10000, 0xFFFF)?;
        vm.memory.add_memory(0x20000, 0xFFFF)?;
        vm.memory.add_memory(0x30000, 0xFFFF)?;
        vm.memory.add_memory(0x40000, 0xFFFF)?;
        vm.memory.add_memory(0x50000, 0xFFFF)?;
        vm.memory.add_memory(0x60000, 0xFFFF)?;
        vm.memory.add_memory(0x70000, 0xFFFF)?;

        //exec data
        vm.memory.add_memory(0x70000000, 0xFFFF)?;
        //tx data
        vm.memory.add_memory(0x70010000, 0xFFFF)?;
        //blockchain data
        vm.memory.add_memory(0x70020000, 0xFFFF)?;

        //stack memory
        vm.memory.add_memory(0x80010000, 1024 * 8)?;
        //primary memory
        vm.memory.add_memory(0x80020000, 0xFFFF)?;
        //aux memory
        vm.memory.add_memory(0x80030000, 0xFFFF)?;
        
        //TODO: later add memory mapping of data areas.. This is currently.. non-trivial in pure Rust
        Ok(())
    }

    fn create_contract_from_sccs(&mut self, vm: &mut VM) -> Result<(), NeutronError>{
        let mut tmp = vec![];
        self.pop_sccs(&mut tmp)?;
//        let _v = NeutronVersion::from_bytes(&tmp);
        //validate version later on..
        
        let mut tmp: Vec<u8> = vec![];
        self.pop_sccs(&mut tmp)?;

        let code_sections = tmp[0];
        assert!(code_sections == 1);
        let mut code: Vec<u8> = vec![];
        self.pop_sccs(&mut code)?;
        vm.copy_into_memory(0x10000, &code).unwrap();

        let data_sections = tmp[0];
        assert!(data_sections == 1);
        let mut data: Vec<u8> = vec![];
        self.pop_sccs(&mut data)?;
        vm.copy_into_memory(0x80020000, &data).unwrap();

        self.pop_sccs_toss()?; //throw away extra data

        //todo: persist code and data...
        
        Ok(())
    }
    fn call_contract_from_sccs(&mut self, _vm: &mut VM){

    }
}

