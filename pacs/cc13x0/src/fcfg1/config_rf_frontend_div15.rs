#[doc = "Register `CONFIG_RF_FRONTEND_DIV15` reader"]
pub struct R(crate::R<CONFIG_RF_FRONTEND_DIV15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_RF_FRONTEND_DIV15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_RF_FRONTEND_DIV15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_RF_FRONTEND_DIV15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_RF_FRONTEND_DIV15` writer"]
pub struct W(crate::W<CONFIG_RF_FRONTEND_DIV15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_RF_FRONTEND_DIV15_SPEC>;
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
impl From<crate::W<CONFIG_RF_FRONTEND_DIV15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_RF_FRONTEND_DIV15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFLDO_TRIM_OUTPUT` reader - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type RFLDO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFLDO_TRIM_OUTPUT` writer - 6:0\\]
Internal. Only to be used through TI provided API."]
pub type RFLDO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_FRONTEND_DIV15_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTL_PA0_TRIM` reader - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTL_PA0_TRIM` writer - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA0_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_FRONTEND_DIV15_SPEC, u8, u8, 5, O>;
#[doc = "Field `IFAMP_TRIM` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFAMP_TRIM` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_FRONTEND_DIV15_SPEC, u8, u8, 5, O>;
#[doc = "Field `LNA_IB` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LNA_IB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNA_IB` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LNA_IB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_FRONTEND_DIV15_SPEC, u8, u8, 4, O>;
#[doc = "Field `IFAMP_IB` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_IB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFAMP_IB` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_IB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_FRONTEND_DIV15_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CTL_PA0_TRIM_R {
        CTL_PA0_TRIM_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IFAMP_TRIM_R {
        IFAMP_TRIM_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&self) -> LNA_IB_R {
        LNA_IB_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IFAMP_IB_R {
        IFAMP_IB_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rfldo_trim_output(&mut self) -> RFLDO_TRIM_OUTPUT_W<0> {
        RFLDO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctl_pa0_trim(&mut self) -> CTL_PA0_TRIM_W<14> {
        CTL_PA0_TRIM_W::new(self)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifamp_trim(&mut self) -> IFAMP_TRIM_W<19> {
        IFAMP_TRIM_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lna_ib(&mut self) -> LNA_IB_W<24> {
        LNA_IB_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ifamp_ib(&mut self) -> IFAMP_IB_W<28> {
        IFAMP_IB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_frontend_div15](index.html) module"]
pub struct CONFIG_RF_FRONTEND_DIV15_SPEC;
impl crate::RegisterSpec for CONFIG_RF_FRONTEND_DIV15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_rf_frontend_div15::R](R) reader structure"]
impl crate::Readable for CONFIG_RF_FRONTEND_DIV15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_rf_frontend_div15::W](W) writer structure"]
impl crate::Writable for CONFIG_RF_FRONTEND_DIV15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_RF_FRONTEND_DIV15 to value 0x7000_3f80"]
impl crate::Resettable for CONFIG_RF_FRONTEND_DIV15_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_3f80;
}
