#[doc = "Register `AIFOUTPTR` reader"]
pub type R = crate::R<AifoutptrSpec>;
#[doc = "Register `AIFOUTPTR` writer"]
pub type W = crate::W<AifoutptrSpec>;
#[doc = "Field `PTR` reader - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
pub type PtrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Value of the DMA output buffer pointer currently used by the DMA controller Incremented by 1 (byte) or 2 (word) for each AHB access."]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {}
#[doc = "DMA Output Buffer Current Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifoutptr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifoutptr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AifoutptrSpec;
impl crate::RegisterSpec for AifoutptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aifoutptr::R`](R) reader structure"]
impl crate::Readable for AifoutptrSpec {}
#[doc = "`write(|w| ..)` method takes [`aifoutptr::W`](W) writer structure"]
impl crate::Writable for AifoutptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AIFOUTPTR to value 0"]
impl crate::Resettable for AifoutptrSpec {
    const RESET_VALUE: u32 = 0;
}
