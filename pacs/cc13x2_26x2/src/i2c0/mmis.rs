#[doc = "Register `MMIS` reader"]
pub type R = crate::R<MmisSpec>;
#[doc = "Register `MMIS` writer"]
pub type W = crate::W<MmisSpec>;
#[doc = "Field `MIS` reader - 0:0\\]
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
pub type MisR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
    #[inline(always)]
    pub fn mis(&self) -> MisR {
        MisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmisSpec;
impl crate::RegisterSpec for MmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmis::R`](R) reader structure"]
impl crate::Readable for MmisSpec {}
#[doc = "`write(|w| ..)` method takes [`mmis::W`](W) writer structure"]
impl crate::Writable for MmisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MMIS to value 0"]
impl crate::Resettable for MmisSpec {
    const RESET_VALUE: u32 = 0;
}
