#[doc = "Register `BFAR` reader"]
pub type R = crate::R<BfarSpec>;
#[doc = "Register `BFAR` writer"]
pub type W = crate::W<BfarSpec>;
#[doc = "Field `ADDRESS` reader - 31:0\\]
Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - 31:0\\]
Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Bus fault address field. This field is the data address of a faulted load or store attempt. When an unaligned access faults, the address is the address requested by the instruction, even if that is not the address that faulted. Flags CFSR.IBUSERR, CFSR.PRECISERR, CFSR.IMPRECISERR, CFSR.UNSTKERR and CFSR.STKERR in combination with CFSR.BFARVALID indicate the cause of the fault."]
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> AddressW<BfarSpec> {
        AddressW::new(self, 0)
    }
}
#[doc = "Bus Fault Address This register is used to read the address of the location that generated a Bus Fault.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bfar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bfar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BfarSpec;
impl crate::RegisterSpec for BfarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bfar::R`](R) reader structure"]
impl crate::Readable for BfarSpec {}
#[doc = "`write(|w| ..)` method takes [`bfar::W`](W) writer structure"]
impl crate::Writable for BfarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BFAR to value 0"]
impl crate::Resettable for BfarSpec {
    const RESET_VALUE: u32 = 0;
}
