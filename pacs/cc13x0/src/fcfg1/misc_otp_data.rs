#[doc = "Register `MISC_OTP_DATA` reader"]
pub struct R(crate::R<MISC_OTP_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_OTP_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_OTP_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_OTP_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_OTP_DATA` writer"]
pub struct W(crate::W<MISC_OTP_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_OTP_DATA_SPEC>;
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
impl From<crate::W<MISC_OTP_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_OTP_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_PROGRAM_REV` reader - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
pub type TEST_PROGRAM_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_PROGRAM_REV` writer - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
pub type TEST_PROGRAM_REV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `MIN_ALLOWED_RTRIM_DIV5` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type MIN_ALLOWED_RTRIM_DIV5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_ALLOWED_RTRIM_DIV5` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type MIN_ALLOWED_RTRIM_DIV5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 4, O>;
#[doc = "Field `PER_E` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type PER_E_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_E` writer - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type PER_E_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 3, O>;
#[doc = "Field `PER_M` reader - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type PER_M_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER_M` writer - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type PER_M_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCOSC_HF_CRIM` reader - 27:20\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_HF_CRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_HF_CRIM` writer - 27:20\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_HF_CRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 8, O>;
#[doc = "Field `RCOSC_HF_ITUNE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_HF_ITUNE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_HF_ITUNE` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_HF_ITUNE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_OTP_DATA_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&self) -> TEST_PROGRAM_REV_R {
        TEST_PROGRAM_REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim_div5(&self) -> MIN_ALLOWED_RTRIM_DIV5_R {
        MIN_ALLOWED_RTRIM_DIV5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PER_E_R {
        PER_E_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PER_M_R {
        PER_M_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&self) -> RCOSC_HF_CRIM_R {
        RCOSC_HF_CRIM_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&self) -> RCOSC_HF_ITUNE_R {
        RCOSC_HF_ITUNE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    #[must_use]
    pub fn test_program_rev(&mut self) -> TEST_PROGRAM_REV_W<0> {
        TEST_PROGRAM_REV_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn min_allowed_rtrim_div5(&mut self) -> MIN_ALLOWED_RTRIM_DIV5_W<8> {
        MIN_ALLOWED_RTRIM_DIV5_W::new(self)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_e(&mut self) -> PER_E_W<12> {
        PER_E_W::new(self)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn per_m(&mut self) -> PER_M_W<15> {
        PER_M_W::new(self)
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_crim(&mut self) -> RCOSC_HF_CRIM_W<20> {
        RCOSC_HF_CRIM_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_hf_itune(&mut self) -> RCOSC_HF_ITUNE_W<28> {
        RCOSC_HF_ITUNE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Misc OTP Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_otp_data](index.html) module"]
pub struct MISC_OTP_DATA_SPEC;
impl crate::RegisterSpec for MISC_OTP_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_otp_data::R](R) reader structure"]
impl crate::Readable for MISC_OTP_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_otp_data::W](W) writer structure"]
impl crate::Writable for MISC_OTP_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_OTP_DATA to value 0xc000"]
impl crate::Resettable for MISC_OTP_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0xc000;
}
