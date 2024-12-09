#[doc = "Register `CLEARCHNLPRIALT` reader"]
pub type R = crate::R<ClearchnlprialtSpec>;
#[doc = "Register `CLEARCHNLPRIALT` writer"]
pub type W = crate::W<ClearchnlprialtSpec>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Clears the appropriate bit to select the primary data structure for the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIALT.CHNLS to select the alternate data structure. Bit \\[Ch\\]
= 1: Selects the primary data structure for channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clears the appropriate bit to select the primary data structure for the corresponding uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIALT.CHNLS to select the alternate data structure. Bit \\[Ch\\]
= 1: Selects the primary data structure for channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ClearchnlprialtSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Clear Primary-Alternate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchnlprialt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchnlprialt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearchnlprialtSpec;
impl crate::RegisterSpec for ClearchnlprialtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearchnlprialt::R`](R) reader structure"]
impl crate::Readable for ClearchnlprialtSpec {}
#[doc = "`write(|w| ..)` method takes [`clearchnlprialt::W`](W) writer structure"]
impl crate::Writable for ClearchnlprialtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARCHNLPRIALT to value 0"]
impl crate::Resettable for ClearchnlprialtSpec {
    const RESET_VALUE: u32 = 0;
}
