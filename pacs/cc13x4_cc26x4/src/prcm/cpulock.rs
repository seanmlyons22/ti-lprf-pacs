#[doc = "Register `CPULOCK` reader"]
pub struct R(crate::R<CPULOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPULOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPULOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPULOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPULOCK` writer"]
pub struct W(crate::W<CPULOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPULOCK_SPEC>;
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
impl From<crate::W<CPULOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPULOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKSMPU` reader - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSMPU` writer - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
#[doc = "Field `LOCKNSMPU` reader - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKNSMPU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSMPU` writer - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKNSMPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
#[doc = "Field `LOCKSAU` reader - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSAU_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSAU` writer - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
#[doc = "Field `LOCKSVTAIRCR` reader - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSVTAIRCR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKSVTAIRCR` writer - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKSVTAIRCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
#[doc = "Field `LOCKNSVTOR` reader - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKNSVTOR_R = crate::BitReader<bool>;
#[doc = "Field `LOCKNSVTOR` writer - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
pub type LOCKNSVTOR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
#[doc = "Field `RESERVED5` reader - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPULOCK_SPEC, u32, u32, 26, O>;
#[doc = "Field `PARITY` reader - 31:31\\]
Register parity bit"]
pub type PARITY_R = crate::BitReader<bool>;
#[doc = "Field `PARITY` writer - 31:31\\]
Register parity bit"]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPULOCK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locknsmpu(&self) -> LOCKNSMPU_R {
        LOCKNSMPU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    pub fn locknsvtor(&self) -> LOCKNSVTOR_R {
        LOCKNSVTOR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:30 - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x03ff_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
When set will lock secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W<0> {
        LOCKSMPU_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
When set will lock non-secure MPU Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locknsmpu(&mut self) -> LOCKNSMPU_W<1> {
        LOCKNSMPU_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
When set will lock SAU regions Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksau(&mut self) -> LOCKSAU_W<2> {
        LOCKSAU_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
When set will lock - Secure vector table base address - Secure interrupt priority - Busfault - Hardfault NMI security target Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W<3> {
        LOCKSVTAIRCR_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
When set will lock non-secure vector table base address Writing this field when BUSSECCFG.VALID is set may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn locknsvtor(&mut self) -> LOCKNSVTOR_W<4> {
        LOCKNSVTOR_W::new(self)
    }
    #[doc = "Bits 5:30 - 30:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Register parity bit"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<31> {
        PARITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpulock](index.html) module"]
pub struct CPULOCK_SPEC;
impl crate::RegisterSpec for CPULOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpulock::R](R) reader structure"]
impl crate::Readable for CPULOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpulock::W](W) writer structure"]
impl crate::Writable for CPULOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPULOCK to value 0x8000_001f"]
impl crate::Resettable for CPULOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_001f;
}
