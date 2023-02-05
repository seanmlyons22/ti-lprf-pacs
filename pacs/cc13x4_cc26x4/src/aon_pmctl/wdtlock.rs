#[doc = "Register `WDTLOCK` reader"]
pub struct R(crate::R<WDTLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTLOCK` writer"]
pub struct W(crate::W<WDTLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDTLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type LOCK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOCK` writer - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WDTLOCK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
WDT Lock: A write of the value 0x1ACC.E551 unlocks the watchdog registers for write access. A write of any other value reapplies the lock, preventing any register updates A read of this register returns the following values: 0x0000.0000: Unlocked 0x0000.0001: Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtlock](index.html) module"]
pub struct WDTLOCK_SPEC;
impl crate::RegisterSpec for WDTLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtlock::R](R) reader structure"]
impl crate::Readable for WDTLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtlock::W](W) writer structure"]
impl crate::Writable for WDTLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTLOCK to value 0"]
impl crate::Resettable for WDTLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
