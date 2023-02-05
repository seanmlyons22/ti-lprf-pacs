#[doc = "Register `CONFIG_MISC_ADC` reader"]
pub struct R(crate::R<CONFIG_MISC_ADC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_MISC_ADC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_MISC_ADC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_MISC_ADC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_MISC_ADC` writer"]
pub struct W(crate::W<CONFIG_MISC_ADC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_MISC_ADC_SPEC>;
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
impl From<crate::W<CONFIG_MISC_ADC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_MISC_ADC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DACTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACTRIM` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DACTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_MISC_ADC_SPEC, u8, u8, 6, O>;
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QUANTCTLTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QUANTCTLTHRES` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QUANTCTLTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_MISC_ADC_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSSI_OFFSET` reader - 16:9\\]
Internal. Only to be used through TI provided API."]
pub type RSSI_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSSI_OFFSET` writer - 16:9\\]
Internal. Only to be used through TI provided API."]
pub type RSSI_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_MISC_ADC_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSSITRIMCOMPLETE_N` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RSSITRIMCOMPLETE_N_R = crate::BitReader<bool>;
#[doc = "Field `RSSITRIMCOMPLETE_N` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RSSITRIMCOMPLETE_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_MISC_ADC_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RSSITRIMCOMPLETE_N_R {
        RSSITRIMCOMPLETE_N_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dactrim(&mut self) -> DACTRIM_W<0> {
        DACTRIM_W::new(self)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn quantctlthres(&mut self) -> QUANTCTLTHRES_W<6> {
        QUANTCTLTHRES_W::new(self)
    }
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rssi_offset(&mut self) -> RSSI_OFFSET_W<9> {
        RSSI_OFFSET_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rssitrimcomplete_n(&mut self) -> RSSITRIMCOMPLETE_N_W<17> {
        RSSITRIMCOMPLETE_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_misc_adc](index.html) module"]
pub struct CONFIG_MISC_ADC_SPEC;
impl crate::RegisterSpec for CONFIG_MISC_ADC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_misc_adc::R](R) reader structure"]
impl crate::Readable for CONFIG_MISC_ADC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_misc_adc::W](W) writer structure"]
impl crate::Writable for CONFIG_MISC_ADC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_MISC_ADC to value 0xfffc_014d"]
impl crate::Resettable for CONFIG_MISC_ADC_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffc_014d;
}
