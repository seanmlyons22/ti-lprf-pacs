#[doc = "Register `FPDSCR` reader"]
pub struct R(crate::R<FPDSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPDSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPDSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPDSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPDSCR` writer"]
pub struct W(crate::W<FPDSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPDSCR_SPEC>;
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
impl From<crate::W<FPDSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPDSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED0` writer - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPDSCR_SPEC, u32, u32, 22, O>;
#[doc = "Field `RMODE` reader - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
pub type RMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RMODE` writer - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
pub type RMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPDSCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FZ` reader - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
pub type FZ_R = crate::BitReader<bool>;
#[doc = "Field `FZ` writer - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
pub type FZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, bool, O>;
#[doc = "Field `DN` reader - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
pub type DN_R = crate::BitReader<bool>;
#[doc = "Field `DN` writer - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
pub type DN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, bool, O>;
#[doc = "Field `AHP` reader - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
pub type AHP_R = crate::BitReader<bool>;
#[doc = "Field `AHP` writer - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
pub type AHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FPDSCR_SPEC, bool, O>;
#[doc = "Field `RESERVED27` reader - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED27` writer - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPDSCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
    #[inline(always)]
    pub fn rmode(&self) -> RMODE_R {
        RMODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
    #[inline(always)]
    pub fn fz(&self) -> FZ_R {
        FZ_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
    #[inline(always)]
    pub fn dn(&self) -> DN_R {
        DN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
    #[inline(always)]
    pub fn ahp(&self) -> AHP_R {
        AHP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Default value for Rounding Mode control field. (The encoding for this field is: 0b00 Round to Nearest (RN) mode 0b01 Round towards Plus Infinity (RP) mode 0b10 Round towards Minus Infinity (RM) mode 0b11 Round towards Zero (RZ) mode. The specified rounding mode is used by almost all floating-point instructions)."]
    #[inline(always)]
    #[must_use]
    pub fn rmode(&mut self) -> RMODE_W<22> {
        RMODE_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Default value for Flush-to-Zero mode bit. (If this bit is set to 1 then Flush-to-zero mode is enabled)."]
    #[inline(always)]
    #[must_use]
    pub fn fz(&mut self) -> FZ_W<24> {
        FZ_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Default value for Default NaN mode bit. (If this bit is set to 1 then any operation involving one or more NaNs returns the Default NaN)."]
    #[inline(always)]
    #[must_use]
    pub fn dn(&mut self) -> DN_W<25> {
        DN_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Default value for Alternative Half Precision bit. (If this bit is set to 1 then Alternative half-precision format is selected)."]
    #[inline(always)]
    #[must_use]
    pub fn ahp(&mut self) -> AHP_W<26> {
        AHP_W::new(self)
    }
    #[doc = "Bits 27:31 - 31:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> RESERVED27_W<27> {
        RESERVED27_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Floating Point Default Status Control This register holds the default values for the floating-point status control data that the processor assigns to the FPSCR when it creates a new floating-point context. Accessible only by privileged software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpdscr](index.html) module"]
pub struct FPDSCR_SPEC;
impl crate::RegisterSpec for FPDSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpdscr::R](R) reader structure"]
impl crate::Readable for FPDSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpdscr::W](W) writer structure"]
impl crate::Writable for FPDSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPDSCR to value 0"]
impl crate::Resettable for FPDSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
