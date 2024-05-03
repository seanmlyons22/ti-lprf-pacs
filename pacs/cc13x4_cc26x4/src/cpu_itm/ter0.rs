#[doc = "Register `TER0` reader"]
pub type R = crate::R<Ter0Spec>;
#[doc = "Register `TER0` writer"]
pub type W = crate::W<Ter0Spec>;
#[doc = "Field `STIMENA` reader - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type StimenaR = crate::FieldReader<u32>;
#[doc = "Field `STIMENA` writer - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
pub type StimenaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    pub fn stimena(&self) -> StimenaR {
        StimenaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For STIMENA\\[m\\]
in ITM_TER*n, controls whether ITM_STIM(32*n + m) is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn stimena(&mut self) -> StimenaW<Ter0Spec> {
        StimenaW::new(self, 0)
    }
}
#[doc = "Provide an individual enable bit for each ITM_STIM register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ter0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ter0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ter0Spec;
impl crate::RegisterSpec for Ter0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ter0::R`](R) reader structure"]
impl crate::Readable for Ter0Spec {}
#[doc = "`write(|w| ..)` method takes [`ter0::W`](W) writer structure"]
impl crate::Writable for Ter0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TER0 to value 0"]
impl crate::Resettable for Ter0Spec {
    const RESET_VALUE: u32 = 0;
}
