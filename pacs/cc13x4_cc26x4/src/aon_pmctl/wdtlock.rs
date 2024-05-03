#[doc = "Register `WDTLOCK` reader"]
pub type R = crate::R<WdtlockSpec>;
#[doc = "Register `WDTLOCK` writer"]
pub type W = crate::W<WdtlockSpec>;
#[doc = "Field `LOCK` reader - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type LockR = crate::FieldReader<u32>;
#[doc = "Field `LOCK` writer - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<WdtlockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets WDTLOCK to value 0"]
impl crate::Resettable for WdtlockSpec {
    const RESET_VALUE: u32 = 0;
}
