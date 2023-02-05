#[doc = "Register `MMIS` reader"]
pub struct R(crate::R<MMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMIS` writer"]
pub struct W(crate::W<MMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMIS_SPEC>;
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
impl From<crate::W<MMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIS` reader - 0:0\\]
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
pub type MIS_R = crate::BitReader<bool>;
#[doc = "Field `MIS` writer - 0:0\\]
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
pub type MIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMIS_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMIS_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 1) != 0)
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
Masked interrupt status 0: An interrupt has not occurred or is masked. 1: A master interrupt is pending. This bit is cleared by writing 1 to the MICR.IC bit ."]
    #[inline(always)]
    #[must_use]
    pub fn mis(&mut self) -> MIS_W<0> {
        MIS_W::new(self)
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
#[doc = "Master Masked Interrupt Status This register show which interrupt is active (based on result from MRIS and MIMR).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmis](index.html) module"]
pub struct MMIS_SPEC;
impl crate::RegisterSpec for MMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmis::R](R) reader structure"]
impl crate::Readable for MMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmis::W](W) writer structure"]
impl crate::Writable for MMIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMIS to value 0"]
impl crate::Resettable for MMIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
