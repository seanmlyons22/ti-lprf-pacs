#[doc = "Register `FLASH_OTP_DATA3` reader"]
pub struct R(crate::R<FLASH_OTP_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_OTP_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_OTP_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_OTP_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_OTP_DATA3` writer"]
pub struct W(crate::W<FLASH_OTP_DATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_OTP_DATA3_SPEC>;
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
impl From<crate::W<FLASH_OTP_DATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_OTP_DATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WAIT_SYSCODE` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type WAIT_SYSCODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WAIT_SYSCODE` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type WAIT_SYSCODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA3_SPEC, u8, u8, 8, O>;
#[doc = "Field `FLASH_SIZE` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FLASH_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_SIZE` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type FLASH_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA3_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRIM_1P7` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_1P7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_1P7` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_1P7_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA3_SPEC, u8, u8, 2, O>;
#[doc = "Field `MAX_EC_LEVEL` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MAX_EC_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_EC_LEVEL` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MAX_EC_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA3_SPEC, u8, u8, 4, O>;
#[doc = "Field `DO_PRECOND` reader - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type DO_PRECOND_R = crate::BitReader<bool>;
#[doc = "Field `DO_PRECOND` writer - 22:22\\]
Internal. Only to be used through TI provided API."]
pub type DO_PRECOND_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLASH_OTP_DATA3_SPEC, bool, O>;
#[doc = "Field `EC_STEP_SIZE` reader - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type EC_STEP_SIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EC_STEP_SIZE` writer - 31:23\\]
Internal. Only to be used through TI provided API."]
pub type EC_STEP_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_OTP_DATA3_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wait_syscode(&self) -> WAIT_SYSCODE_R {
        WAIT_SYSCODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flash_size(&self) -> FLASH_SIZE_R {
        FLASH_SIZE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_1p7(&self) -> TRIM_1P7_R {
        TRIM_1P7_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_ec_level(&self) -> MAX_EC_LEVEL_R {
        MAX_EC_LEVEL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn do_precond(&self) -> DO_PRECOND_R {
        DO_PRECOND_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ec_step_size(&self) -> EC_STEP_SIZE_R {
        EC_STEP_SIZE_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wait_syscode(&mut self) -> WAIT_SYSCODE_W<0> {
        WAIT_SYSCODE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flash_size(&mut self) -> FLASH_SIZE_W<8> {
        FLASH_SIZE_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_1p7(&mut self) -> TRIM_1P7_W<16> {
        TRIM_1P7_W::new(self)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_ec_level(&mut self) -> MAX_EC_LEVEL_W<18> {
        MAX_EC_LEVEL_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn do_precond(&mut self) -> DO_PRECOND_W<22> {
        DO_PRECOND_W::new(self)
    }
    #[doc = "Bits 23:31 - 31:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ec_step_size(&mut self) -> EC_STEP_SIZE_W<23> {
        EC_STEP_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_otp_data3](index.html) module"]
pub struct FLASH_OTP_DATA3_SPEC;
impl crate::RegisterSpec for FLASH_OTP_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_otp_data3::R](R) reader structure"]
impl crate::Readable for FLASH_OTP_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_otp_data3::W](W) writer structure"]
impl crate::Writable for FLASH_OTP_DATA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_OTP_DATA3 to value 0x0011_0003"]
impl crate::Resettable for FLASH_OTP_DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0011_0003;
}
