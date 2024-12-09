#[doc = "Register `AIFINPTR` reader"]
pub type R = crate::R<AifinptrSpec>;
#[doc = "Register `AIFINPTR` writer"]
pub type W = crate::W<AifinptrSpec>;
#[doc = "Field `PTR` reader - 31:0\\]
Value of the DMA input buffer pointer currently used by the DMA controller. Incremented by 1 (byte) or 2 (word) for each AHB access."]
pub type PtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of the DMA input buffer pointer currently used by the DMA controller. Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {}
#[doc = "DMA Input Buffer Current Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifinptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifinptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifinptrSpec;
impl crate::RegisterSpec for AifinptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifinptr::R`](R) reader structure"]
impl crate::Readable for AifinptrSpec {}
#[doc = "`write(|w| ..)` method takes [`aifinptr::W`](W) writer structure"]
impl crate::Writable for AifinptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFINPTR to value 0"]
impl crate::Resettable for AifinptrSpec {
    const RESET_VALUE: u32 = 0;
}
