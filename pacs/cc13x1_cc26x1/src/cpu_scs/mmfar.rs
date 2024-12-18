#[doc = "Register `MMFAR` reader"]
pub type R = crate::R<MmfarSpec>;
#[doc = "Register `MMFAR` writer"]
pub type W = crate::W<MmfarSpec>;
#[doc = "Field `ADDRESS` reader - 31:0\\]
Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - 31:0\\]
Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Mem Manage fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the actual address that faulted. Because an access can be split into multiple parts, each aligned, this address can be any offset in the range of the requested size. Flags CFSR.IACCVIOL, CFSR.DACCVIOL ,CFSR.MUNSTKERR and CFSR.MSTKERR in combination with CFSR.MMARVALIDindicate the cause of the fault."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<MmfarSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Mem Manage Fault Address This register is used to read the address of the location that caused a Memory Manage Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmfar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmfar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmfarSpec;
impl crate::RegisterSpec for MmfarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmfar::R`](R) reader structure"]
impl crate::Readable for MmfarSpec {}
#[doc = "`write(|w| ..)` method takes [`mmfar::W`](W) writer structure"]
impl crate::Writable for MmfarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMFAR to value 0"]
impl crate::Resettable for MmfarSpec {
    const RESET_VALUE: u32 = 0;
}
