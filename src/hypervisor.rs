extern crate qx86;
use qx86::vm::*;
use crate::*;
use struct_deser::*;

pub struct NeutronHypervisor{
    api: dyn NeutronAPI
}
impl Hypervisor for NeutronHypervisor{
    fn interrupt(&mut self, vm: &mut VM, num: u8) -> Result<(), VMError>{
        let ctx = self.api.get_context();
        vm.set_reg32(Reg32::EAX, ctx.exec.nest_level);
        println!("Interrupt occurred! {}", num);
        Ok(())
    }
}



impl NeutronHypervisor{
    pub fn init_cpu(&mut self, vm: &mut VM) -> Result<(), VMError>{
        self.init_memory(vm)?;
        vm.gas_remaining = self.api.get_context().exec.gas_limit;
        vm.eip = 0x10000;
        Ok(())
    }
    pub fn init_memory(&mut self, vm: &mut VM) -> Result<(), VMError>{
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
        Ok(())
    }

    pub fn create_contract_from_sccs(&mut self, vm: &mut VM) -> Result<(), NeutronError>{
        let mut tmp = vec![];
        self.api.pop_sccs(&mut tmp)?;
        let _v = NeutronVersion::from_bytes(&tmp);
        //validate version later on..
        
        let mut tmp: Vec<u8> = vec![];
        self.api.pop_sccs(&mut tmp)?;

        let code_sections = tmp[0];
        assert!(code_sections == 1);
        let mut code: Vec<u8> = vec![];
        self.api.pop_sccs(&mut code)?;
        vm.copy_into_memory(0x10000, &code).unwrap();

        let data_sections = tmp[0];
        assert!(data_sections == 1);
        let mut data: Vec<u8> = vec![];
        self.api.pop_sccs(&mut data)?;
        vm.copy_into_memory(0x80020000, &data).unwrap();

        self.api.pop_sccs_toss(); //throw away extra data

        //todo: persist code and data...
        
        Ok(())
    }
    pub fn call_contract_from_sccs(&mut self, vm: &mut VM){

    }
}

