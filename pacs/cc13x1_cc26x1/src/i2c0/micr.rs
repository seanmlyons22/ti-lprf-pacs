#[doc = "Register `MICR` reader"]
pub struct R(crate::R<MICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MICR` writer"]
pub struct W(crate::W<MICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MICR_SPEC>;
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
impl From<crate::W<MICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC` reader - 0:0\\]
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
pub type IC_R = crate::BitReader<bool>;
#[doc = "Field `IC` writer - 0:0\\]
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
pub type IC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MICR_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MICR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 1) != 0)
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
Interrupt clear Writing 1 to this bit clears MRIS.RIS and MMIS.MIS . Reading this register returns no meaningful data."]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IC_W<0> {
        IC_W::new(self)
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
#[doc = "Master Interrupt Clear This register clears the raw and masked interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](index.html) module"]
pub struct MICR_SPEC;
impl crate::RegisterSpec for MICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [micr::R](R) reader structure"]
impl crate::Readable for MICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [micr::W](W) writer structure"]
impl crate::Writable for MICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MICR to value 0"]
impl crate::Resettable for MICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
