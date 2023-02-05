#[doc = "Register `TEMPUPD` reader"]
pub struct R(crate::R<TEMPUPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPUPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPUPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPUPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPUPD` writer"]
pub struct W(crate::W<TEMPUPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPUPD_SPEC>;
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
impl From<crate::W<TEMPUPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPUPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
pub type STAT_R = crate::BitReader<bool>;
#[doc = "Field `STAT` writer - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
pub type STAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TEMPUPD_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEMPUPD_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No update since last clear 1: New temperature is present. Write 1 to clear the status."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature Update Indicates TEMP Updates\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempupd](index.html) module"]
pub struct TEMPUPD_SPEC;
impl crate::RegisterSpec for TEMPUPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempupd::R](R) reader structure"]
impl crate::Readable for TEMPUPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempupd::W](W) writer structure"]
impl crate::Writable for TEMPUPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPUPD to value 0"]
impl crate::Resettable for TEMPUPD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
