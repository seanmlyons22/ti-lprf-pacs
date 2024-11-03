#[doc = "Register `WDTLOCK` reader"]
pub type R = crate::R<WdtlockSpec>;
#[doc = "Register `WDTLOCK` writer"]
pub type W = crate::W<WdtlockSpec>;
#[doc = "Field `STAT` reader - 31:0\\]
A write with value 0x1ACCE551 unlocks the watchdog registers for write access. A write with any other value locks the watchdog registers for write access. Writing the WDTCNT register will also lock the watchdog registers. A read of this field returns the state of the lock (0=unlocked, 1=locked)."]
pub type StatR = crate::FieldReader<u32>;
#[doc = "Field `STAT` writer - 31:0\\]
A write with value 0x1ACCE551 unlocks the watchdog registers for write access. A write with any other value locks the watchdog registers for write access. Writing the WDTCNT register will also lock the watchdog registers. A read of this field returns the state of the lock (0=unlocked, 1=locked)."]
pub type StatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
A write with value 0x1ACCE551 unlocks the watchdog registers for write access. A write with any other value locks the watchdog registers for write access. Writing the WDTCNT register will also lock the watchdog registers. A read of this field returns the state of the lock (0=unlocked, 1=locked)."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
A write with value 0x1ACCE551 unlocks the watchdog registers for write access. A write with any other value locks the watchdog registers for write access. Writing the WDTCNT register will also lock the watchdog registers. A read of this field returns the state of the lock (0=unlocked, 1=locked)."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<WdtlockSpec> {
        StatW::new(self, 0)
    }
}
#[doc = "WDT lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtlockSpec;
impl crate::RegisterSpec for WdtlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtlock::R`](R) reader structure"]
impl crate::Readable for WdtlockSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtlock::W`](W) writer structure"]
impl crate::Writable for WdtlockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTLOCK to value 0x01"]
impl crate::Resettable for WdtlockSpec {
    const RESET_VALUE: u32 = 0x01;
}
