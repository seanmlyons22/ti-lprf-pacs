#[doc = "Register `WDTCNT` reader"]
pub type R = crate::R<WdtcntSpec>;
#[doc = "Register `WDTCNT` writer"]
pub type W = crate::W<WdtcntSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Counter value. A write to this field immediately starts (or restarts) the counter. It will count down from the written value. If the counter reaches 0, a reset will be generated. A write value of 0 immediately generates a reset. This field is only writable if not locked. See WDTLOCK register. Writing this field will automatically activate the lock. A read returns the current value of the counter."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Counter value. A write to this field immediately starts (or restarts) the counter. It will count down from the written value. If the counter reaches 0, a reset will be generated. A write value of 0 immediately generates a reset. This field is only writable if not locked. See WDTLOCK register. Writing this field will automatically activate the lock. A read returns the current value of the counter."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. A write to this field immediately starts (or restarts) the counter. It will count down from the written value. If the counter reaches 0, a reset will be generated. A write value of 0 immediately generates a reset. This field is only writable if not locked. See WDTLOCK register. Writing this field will automatically activate the lock. A read returns the current value of the counter."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. A write to this field immediately starts (or restarts) the counter. It will count down from the written value. If the counter reaches 0, a reset will be generated. A write value of 0 immediately generates a reset. This field is only writable if not locked. See WDTLOCK register. Writing this field will automatically activate the lock. A read returns the current value of the counter."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<WdtcntSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "WDT counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcntSpec;
impl crate::RegisterSpec for WdtcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtcnt::R`](R) reader structure"]
impl crate::Readable for WdtcntSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcnt::W`](W) writer structure"]
impl crate::Writable for WdtcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTCNT to value 0"]
impl crate::Resettable for WdtcntSpec {
    const RESET_VALUE: u32 = 0;
}
