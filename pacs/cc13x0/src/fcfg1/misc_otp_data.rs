#[doc = "Register `MISC_OTP_DATA` reader"]
pub type R = crate::R<MiscOtpDataSpec>;
#[doc = "Register `MISC_OTP_DATA` writer"]
pub type W = crate::W<MiscOtpDataSpec>;
#[doc = "Field `TEST_PROGRAM_REV` reader - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
pub type TestProgramRevR = crate::FieldReader;
#[doc = "Field `MIN_ALLOWED_RTRIM_DIV5` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type MinAllowedRtrimDiv5R = crate::FieldReader;
#[doc = "Field `PER_E` reader - 14:12\\]
Internal. Only to be used through TI provided API."]
pub type PerER = crate::FieldReader;
#[doc = "Field `PER_M` reader - 19:15\\]
Internal. Only to be used through TI provided API."]
pub type PerMR = crate::FieldReader;
#[doc = "Field `RCOSC_HF_CRIM` reader - 27:20\\]
Internal. Only to be used through TI provided API."]
pub type RcoscHfCrimR = crate::FieldReader;
#[doc = "Field `RCOSC_HF_ITUNE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RcoscHfItuneR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The revision of the test program used in the production process when FCFG1 was programmed. Value migth change without warning."]
    #[inline(always)]
    pub fn test_program_rev(&self) -> TestProgramRevR {
        TestProgramRevR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim_div5(&self) -> MinAllowedRtrimDiv5R {
        MinAllowedRtrimDiv5R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - 14:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_e(&self) -> PerER {
        PerER::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:19 - 19:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn per_m(&self) -> PerMR {
        PerMR::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:27 - 27:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_crim(&self) -> RcoscHfCrimR {
        RcoscHfCrimR::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_hf_itune(&self) -> RcoscHfItuneR {
        RcoscHfItuneR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Misc OTP Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc_otp_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_otp_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscOtpDataSpec;
impl crate::RegisterSpec for MiscOtpDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc_otp_data::R`](R) reader structure"]
impl crate::Readable for MiscOtpDataSpec {}
#[doc = "`write(|w| ..)` method takes [`misc_otp_data::W`](W) writer structure"]
impl crate::Writable for MiscOtpDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC_OTP_DATA to value 0xc000"]
impl crate::Resettable for MiscOtpDataSpec {
    const RESET_VALUE: u32 = 0xc000;
}
