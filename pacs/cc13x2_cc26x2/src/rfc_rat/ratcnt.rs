#[doc = "Register `RATCNT` reader"]
pub type R = crate::R<RatcntSpec>;
#[doc = "Register `RATCNT` writer"]
pub type W = crate::W<RatcntSpec>;
#[doc = "Field `CNT` reader - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
pub type CntR = crate::FieldReader<u32>;
#[doc = "Field `CNT` writer - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<RatcntSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "Radio Timer Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RatcntSpec;
impl crate::RegisterSpec for RatcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratcnt::R`](R) reader structure"]
impl crate::Readable for RatcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ratcnt::W`](W) writer structure"]
impl crate::Writable for RatcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATCNT to value 0"]
impl crate::Resettable for RatcntSpec {
    const RESET_VALUE: u32 = 0;
}
