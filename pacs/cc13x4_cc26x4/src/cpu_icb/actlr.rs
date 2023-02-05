#[doc = "Register `ACTLR` reader"]
pub struct R(crate::R<ACTLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACTLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACTLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACTLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACTLR` writer"]
pub struct W(crate::W<ACTLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACTLR_SPEC>;
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
impl From<crate::W<ACTLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACTLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMCYCINT` reader - 0:0\\]
Disable dual-issue."]
pub type DISMCYCINT_R = crate::BitReader<bool>;
#[doc = "Field `DISMCYCINT` writer - 0:0\\]
Disable dual-issue."]
pub type DISMCYCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISFOLD` reader - 2:2\\]
Disable dual-issue."]
pub type DISFOLD_R = crate::BitReader<bool>;
#[doc = "Field `DISFOLD` writer - 2:2\\]
Disable dual-issue."]
pub type DISFOLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DISOOFP` reader - 9:9\\]
Disable out-of-order FP instruction completion"]
pub type DISOOFP_R = crate::BitReader<bool>;
#[doc = "Field `DISOOFP` writer - 9:9\\]
Disable out-of-order FP instruction completion"]
pub type DISOOFP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `FPEXCODIS` reader - 10:10\\]
Disable FPU exception outputs"]
pub type FPEXCODIS_R = crate::BitReader<bool>;
#[doc = "Field `FPEXCODIS` writer - 10:10\\]
Disable FPU exception outputs"]
pub type FPEXCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED11` writer - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `DISITMATBFLUSH` reader - 12:12\\]
Disable ATB Flush"]
pub type DISITMATBFLUSH_R = crate::BitReader<bool>;
#[doc = "Field `DISITMATBFLUSH` writer - 12:12\\]
Disable ATB Flush"]
pub type DISITMATBFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED13` reader - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED13` writer - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u16, u16, 16, O>;
#[doc = "Field `EXTEXCLALL` reader - 29:29\\]
External Exclusives Allowed with no MPU"]
pub type EXTEXCLALL_R = crate::BitReader<bool>;
#[doc = "Field `EXTEXCLALL` writer - 29:29\\]
External Exclusives Allowed with no MPU"]
pub type EXTEXCLALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACTLR_SPEC, bool, O>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED30` writer - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACTLR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Disable dual-issue."]
    #[inline(always)]
    pub fn dismcycint(&self) -> DISMCYCINT_R {
        DISMCYCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable dual-issue."]
    #[inline(always)]
    pub fn disfold(&self) -> DISFOLD_R {
        DISFOLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 3) & 0x3f) as u8)
    }
    #[doc = "Bit 9 - 9:9\\]
Disable out-of-order FP instruction completion"]
    #[inline(always)]
    pub fn disoofp(&self) -> DISOOFP_R {
        DISOOFP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable FPU exception outputs"]
    #[inline(always)]
    pub fn fpexcodis(&self) -> FPEXCODIS_R {
        FPEXCODIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable ATB Flush"]
    #[inline(always)]
    pub fn disitmatbflush(&self) -> DISITMATBFLUSH_R {
        DISITMATBFLUSH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved13(&self) -> RESERVED13_R {
        RESERVED13_R::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
External Exclusives Allowed with no MPU"]
    #[inline(always)]
    pub fn extexclall(&self) -> EXTEXCLALL_R {
        EXTEXCLALL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> RESERVED30_R {
        RESERVED30_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn dismcycint(&mut self) -> DISMCYCINT_W<0> {
        DISMCYCINT_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Disable dual-issue."]
    #[inline(always)]
    #[must_use]
    pub fn disfold(&mut self) -> DISFOLD_W<2> {
        DISFOLD_W::new(self)
    }
    #[doc = "Bits 3:8 - 8:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Disable out-of-order FP instruction completion"]
    #[inline(always)]
    #[must_use]
    pub fn disoofp(&mut self) -> DISOOFP_W<9> {
        DISOOFP_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Disable FPU exception outputs"]
    #[inline(always)]
    #[must_use]
    pub fn fpexcodis(&mut self) -> FPEXCODIS_W<10> {
        FPEXCODIS_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Disable ATB Flush"]
    #[inline(always)]
    #[must_use]
    pub fn disitmatbflush(&mut self) -> DISITMATBFLUSH_W<12> {
        DISITMATBFLUSH_W::new(self)
    }
    #[doc = "Bits 13:28 - 28:13\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved13(&mut self) -> RESERVED13_W<13> {
        RESERVED13_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
External Exclusives Allowed with no MPU"]
    #[inline(always)]
    #[must_use]
    pub fn extexclall(&mut self) -> EXTEXCLALL_W<29> {
        EXTEXCLALL_W::new(self)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> RESERVED30_W<30> {
        RESERVED30_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides IMPLEMENTATION DEFINED configuration and control options\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actlr](index.html) module"]
pub struct ACTLR_SPEC;
impl crate::RegisterSpec for ACTLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [actlr::R](R) reader structure"]
impl crate::Readable for ACTLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [actlr::W](W) writer structure"]
impl crate::Writable for ACTLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACTLR to value 0"]
impl crate::Resettable for ACTLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
