#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Field `WDTLOCK` reader - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type WdtlockR = crate::FieldReader<u32>;
#[doc = "Field `WDTLOCK` writer - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type WdtlockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    pub fn wdtlock(&self) -> WdtlockR {
        WdtlockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates (NOTE: TEST.TEST_EN bit is not lockable). A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    #[must_use]
    pub fn wdtlock(&mut self) -> WdtlockW<LockSpec> {
        WdtlockW::new(self, 0)
    }
}
#[doc = "Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {
    const RESET_VALUE: u32 = 0;
}
