#[doc = "Register `SETCHANNELEN` reader"]
pub type R = crate::R<SetchannelenSpec>;
#[doc = "Register `SETCHANNELEN` writer"]
pub type W = crate::W<SetchannelenSpec>;
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\]
= 0: Channel Ch is disabled. Bit \\[Ch\\]
= 1: Channel Ch is enabled. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect"]
pub type ChnlsR = crate::FieldReader<u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\]
= 0: Channel Ch is disabled. Bit \\[Ch\\]
= 1: Channel Ch is enabled. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\]
= 0: Channel Ch is disabled. Bit \\[Ch\\]
= 1: Channel Ch is enabled. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the enable status of the channels, or enables the corresponding channels. Read as: Bit \\[Ch\\]
= 0: Channel Ch is disabled. Bit \\[Ch\\]
= 1: Channel Ch is enabled. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHANNELEN.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Enables channel Ch Writing to a bit where a DMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SetchannelenSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Set Channel Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchannelen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchannelen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetchannelenSpec;
impl crate::RegisterSpec for SetchannelenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setchannelen::R`](R) reader structure"]
impl crate::Readable for SetchannelenSpec {}
#[doc = "`write(|w| ..)` method takes [`setchannelen::W`](W) writer structure"]
impl crate::Writable for SetchannelenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETCHANNELEN to value 0"]
impl crate::Resettable for SetchannelenSpec {
    const RESET_VALUE: u32 = 0;
}
