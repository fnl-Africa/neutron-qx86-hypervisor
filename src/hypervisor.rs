extern crate qx86;
use qx86::vm::*;
use crate::*;

pub struct NeutronHypervisor{
    api: dyn NeutronAPI
}
impl Hypervisor for NeutronHypervisor{
    fn interrupt(&mut self, vm: &mut VM, num: u8) -> Result<(), VMError>{
        let ctx = self.api.get_context();
        vm.set_reg32(Reg32::EAX, ctx.exec.nest_level);
        Ok(())
    }
}

