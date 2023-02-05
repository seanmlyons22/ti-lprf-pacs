#[doc = "Register `MRIS` reader"]
pub struct R(crate::R<MRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRIS` writer"]
pub struct W(crate::W<MRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRIS_SPEC>;
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
impl From<crate::W<MRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RIS` reader - 0:0\\]
Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
pub type RIS_R = crate::BitReader<bool>;
#[doc = "Field `RIS` writer - 0:0\\]
Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
pub type RIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRIS_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRIS_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 1) != 0)
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
Raw interrupt status 0: No interrupt 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
    #[inline(always)]
    #[must_use]
    pub fn ris(&mut self) -> RIS_W<0> {
        RIS_W::new(self)
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
#[doc = "Master Raw Interrupt Status This register show the unmasked interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mris](index.html) module"]
pub struct MRIS_SPEC;
impl crate::RegisterSpec for MRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mris::R](R) reader structure"]
impl crate::Readable for MRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mris::W](W) writer structure"]
impl crate::Writable for MRIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRIS to value 0"]
impl crate::Resettable for MRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
