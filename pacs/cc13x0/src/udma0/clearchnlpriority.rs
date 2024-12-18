#[doc = "Register `CLEARCHNLPRIORITY` reader"]
pub type R = crate::R<ClearchnlprioritySpec>;
#[doc = "Register `CLEARCHNLPRIORITY` writer"]
pub type W = crate::W<ClearchnlprioritySpec>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Clear the appropriate bit to select the default priority level for the specified uDMA channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETCHNLPRIORITY.CHNLS to set channel Ch to the high priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the default priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ClearchnlprioritySpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Clear Channel Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchnlpriority::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchnlpriority::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearchnlprioritySpec;
impl crate::RegisterSpec for ClearchnlprioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearchnlpriority::R`](R) reader structure"]
impl crate::Readable for ClearchnlprioritySpec {}
#[doc = "`write(|w| ..)` method takes [`clearchnlpriority::W`](W) writer structure"]
impl crate::Writable for ClearchnlprioritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARCHNLPRIORITY to value 0"]
impl crate::Resettable for ClearchnlprioritySpec {
    const RESET_VALUE: u32 = 0;
}
