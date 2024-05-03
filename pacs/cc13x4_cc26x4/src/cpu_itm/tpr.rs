#[doc = "Register `TPR` reader"]
pub type R = crate::R<TprSpec>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TprSpec>;
#[doc = "Field `PRIVMASK` reader - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
pub type PrivmaskR = crate::FieldReader<u32>;
#[doc = "Field `PRIVMASK` writer - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
pub type PrivmaskW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
    #[inline(always)]
    pub fn privmask(&self) -> PrivmaskR {
        PrivmaskR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
    #[inline(always)]
    #[must_use]
    pub fn privmask(&mut self) -> PrivmaskW<TprSpec> {
        PrivmaskW::new(self, 0)
    }
}
#[doc = "Controls which stimulus ports can be accessed by unprivileged code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TprSpec;
impl crate::RegisterSpec for TprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TprSpec {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TprSpec {
    const RESET_VALUE: u32 = 0;
}
