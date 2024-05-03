#[doc = "Register `SETCHNLPRIALT` reader"]
pub type R = crate::R<SetchnlprialtSpec>;
#[doc = "Register `SETCHNLPRIALT` writer"]
pub type W = crate::W<SetchnlprialtSpec>;
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsR = crate::FieldReader<u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the channel control data structure status, or selects the alternate data structure for the corresponding uDMA channel. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the primary data structure. Bit \\[Ch\\]
= 1: uDMA channel Ch is using the alternate data structure. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIALT.CHNLS to disable a channel Bit \\[Ch\\]
= 1: Selects the alternate data structure for channel Ch Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SetchnlprialtSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Set Primary-Alternate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchnlprialt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchnlprialt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetchnlprialtSpec;
impl crate::RegisterSpec for SetchnlprialtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setchnlprialt::R`](R) reader structure"]
impl crate::Readable for SetchnlprialtSpec {}
#[doc = "`write(|w| ..)` method takes [`setchnlprialt::W`](W) writer structure"]
impl crate::Writable for SetchnlprialtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETCHNLPRIALT to value 0"]
impl crate::Resettable for SetchnlprialtSpec {
    const RESET_VALUE: u32 = 0;
}
