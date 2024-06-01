#[doc = "Register `CLEARREQMASK` reader"]
pub type R = crate::R<ClearreqmaskSpec>;
#[doc = "Register `CLEARREQMASK` writer"]
pub type W = crate::W<ClearreqmaskSpec>;
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type ChnlsR = crate::FieldReader<u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ClearreqmaskSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Clear Channel Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearreqmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearreqmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearreqmaskSpec;
impl crate::RegisterSpec for ClearreqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearreqmask::R`](R) reader structure"]
impl crate::Readable for ClearreqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`clearreqmask::W`](W) writer structure"]
impl crate::Writable for ClearreqmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARREQMASK to value 0"]
impl crate::Resettable for ClearreqmaskSpec {
    const RESET_VALUE: u32 = 0;
}
