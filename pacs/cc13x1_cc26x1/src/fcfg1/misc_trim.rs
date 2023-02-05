#[doc = "Register `MISC_TRIM` reader"]
pub struct R(crate::R<MISC_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_TRIM` writer"]
pub struct W(crate::W<MISC_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_TRIM_SPEC>;
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
impl From<crate::W<MISC_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPVSLOPE` reader - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TEMPVSLOPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMPVSLOPE` writer - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
pub type TEMPVSLOPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MISC_TRIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `TRIM_RECHARGE_COMP_REFLEVEL` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_RECHARGE_COMP_REFLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_RECHARGE_COMP_REFLEVEL` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_RECHARGE_COMP_REFLEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRIM_RECHARGE_COMP_OFFSET` reader - 16:12\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_RECHARGE_COMP_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM_RECHARGE_COMP_OFFSET` writer - 16:12\\]
Internal. Only to be used through TI provided API."]
pub type TRIM_RECHARGE_COMP_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCOSCHF_FT_RETRIM_FINE_R` reader - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_FT_RETRIM_FINE_R_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHF_FT_RETRIM_FINE_R` writer - 30:29\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_FT_RETRIM_FINE_R_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MISC_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `RCOSCHF_FT_RETRIM_N` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_FT_RETRIM_N_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHF_FT_RETRIM_N` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_FT_RETRIM_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MISC_TRIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    pub fn tempvslope(&self) -> TEMPVSLOPE_R {
        TEMPVSLOPE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_reflevel(&self) -> TRIM_RECHARGE_COMP_REFLEVEL_R {
        TRIM_RECHARGE_COMP_REFLEVEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trim_recharge_comp_offset(&self) -> TRIM_RECHARGE_COMP_OFFSET_R {
        TRIM_RECHARGE_COMP_OFFSET_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ft_retrim_fine_r(&self) -> RCOSCHF_FT_RETRIM_FINE_R_R {
        RCOSCHF_FT_RETRIM_FINE_R_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ft_retrim_n(&self) -> RCOSCHF_FT_RETRIM_N_R {
        RCOSCHF_FT_RETRIM_N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Signed byte value representing the TEMP slope with battery voltage, in degrees C / V, with four fractional bits."]
    #[inline(always)]
    #[must_use]
    pub fn tempvslope(&mut self) -> TEMPVSLOPE_W<0> {
        TEMPVSLOPE_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_recharge_comp_reflevel(&mut self) -> TRIM_RECHARGE_COMP_REFLEVEL_W<8> {
        TRIM_RECHARGE_COMP_REFLEVEL_W::new(self)
    }
    #[doc = "Bits 12:16 - 16:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trim_recharge_comp_offset(&mut self) -> TRIM_RECHARGE_COMP_OFFSET_W<12> {
        TRIM_RECHARGE_COMP_OFFSET_W::new(self)
    }
    #[doc = "Bits 29:30 - 30:29\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_ft_retrim_fine_r(&mut self) -> RCOSCHF_FT_RETRIM_FINE_R_W<29> {
        RCOSCHF_FT_RETRIM_FINE_R_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_ft_retrim_n(&mut self) -> RCOSCHF_FT_RETRIM_N_W<31> {
        RCOSCHF_FT_RETRIM_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous Trim Parameters\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_trim](index.html) module"]
pub struct MISC_TRIM_SPEC;
impl crate::RegisterSpec for MISC_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_trim::R](R) reader structure"]
impl crate::Readable for MISC_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_trim::W](W) writer structure"]
impl crate::Writable for MISC_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MISC_TRIM to value 0xfffe_003b"]
impl crate::Resettable for MISC_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffe_003b;
}
