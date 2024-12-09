#[doc = "Register `DMASWRESET` reader"]
pub type R = crate::R<DmaswresetSpec>;
#[doc = "Register `DMASWRESET` writer"]
pub type W = crate::W<DmaswresetSpec>;
#[doc = "Field `RESET` writer - 0:0\\]
Software reset enable 0: Disable 1: Enable (self-cleared to zero). Note: Completion of the software reset must be checked in DMASTAT.CH0_ACTIVE and DMASTAT.CH1_ACTIVE."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset enable 0: Disable 1: Enable (self-cleared to zero). Note: Completion of the software reset must be checked in DMASTAT.CH0_ACTIVE and DMASTAT.CH1_ACTIVE."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<DmaswresetSpec> {
        ResetW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DmaswresetSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "DMA Controller Software Reset\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaswreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaswreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaswresetSpec;
impl crate::RegisterSpec for DmaswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaswreset::R`](R) reader structure"]
impl crate::Readable for DmaswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaswreset::W`](W) writer structure"]
impl crate::Writable for DmaswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASWRESET to value 0"]
impl crate::Resettable for DmaswresetSpec {
    const RESET_VALUE: u32 = 0;
}
