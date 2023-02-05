#[doc = "Register `DMACHB` reader"]
pub type R = crate::R<DmachbSpec>;
#[doc = "Register `DMACHB` writer"]
pub type W = crate::W<DmachbSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Value transferred through DMA Channel B"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Value transferred through DMA Channel B"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value transferred through DMA Channel B"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Value transferred through DMA Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<DmachbSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "DMA Channel B data transfer DMA accesses this register to read or write contents from sequential addresses specifed by DMA.ADRCHB.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmachb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmachb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmachbSpec;
impl crate::RegisterSpec for DmachbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmachb::R`](R) reader structure"]
impl crate::Readable for DmachbSpec {}
#[doc = "`write(|w| ..)` method takes [`dmachb::W`](W) writer structure"]
impl crate::Writable for DmachbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACHB to value 0"]
impl crate::Resettable for DmachbSpec {
    const RESET_VALUE: u32 = 0;
}
