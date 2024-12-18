#[doc = "Register `CLEARCHANNELEN` reader"]
pub type R = crate::R<ClearchannelenSpec>;
#[doc = "Register `CLEARCHANNELEN` writer"]
pub type W = crate::W<ClearchannelenSpec>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to disable the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHANNELEN.CHNLS to enable uDMA channels. Bit \\[Ch\\]
= 1: Disables channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ClearchannelenSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Clear Channel Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchannelen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchannelen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearchannelenSpec;
impl crate::RegisterSpec for ClearchannelenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearchannelen::R`](R) reader structure"]
impl crate::Readable for ClearchannelenSpec {}
#[doc = "`write(|w| ..)` method takes [`clearchannelen::W`](W) writer structure"]
impl crate::Writable for ClearchannelenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARCHANNELEN to value 0"]
impl crate::Resettable for ClearchannelenSpec {
    const RESET_VALUE: u32 = 0;
}
