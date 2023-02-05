#[doc = "Register `CONFIG_CC26_FE` reader"]
pub struct R(crate::R<CONFIG_CC26_FE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_CC26_FE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_CC26_FE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_CC26_FE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_CC26_FE` writer"]
pub struct W(crate::W<CONFIG_CC26_FE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_CC26_FE_SPEC>;
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
impl From<crate::W<CONFIG_CC26_FE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_CC26_FE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSSI_OFFSET` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RSSI_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSSI_OFFSET` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RSSI_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_CC26_FE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RSSITRIMCOMPLETE_N` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RSSITRIMCOMPLETE_N_R = crate::BitReader<bool>;
#[doc = "Field `RSSITRIMCOMPLETE_N` writer - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RSSITRIMCOMPLETE_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_CC26_FE_SPEC, bool, O>;
#[doc = "Field `PATRIMCOMPLETE_N` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type PATRIMCOMPLETE_N_R = crate::BitReader<bool>;
#[doc = "Field `PATRIMCOMPLETE_N` writer - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type PATRIMCOMPLETE_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_CC26_FE_SPEC, bool, O>;
#[doc = "Field `CTL_PA0_TRIM` reader - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA0_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTL_PA0_TRIM` writer - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA0_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_CC26_FE_SPEC, u8, u8, 5, O>;
#[doc = "Field `IFAMP_TRIM` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFAMP_TRIM` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_CC26_FE_SPEC, u8, u8, 5, O>;
#[doc = "Field `LNA_IB` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LNA_IB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNA_IB` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LNA_IB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONFIG_CC26_FE_SPEC, u8, u8, 4, O>;
#[doc = "Field `IFAMP_IB` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_IB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IFAMP_IB` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IFAMP_IB_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_CC26_FE_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RSSI_OFFSET_R {
        RSSI_OFFSET_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RSSITRIMCOMPLETE_N_R {
        RSSITRIMCOMPLETE_N_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn patrimcomplete_n(&self) -> PATRIMCOMPLETE_N_R {
        PATRIMCOMPLETE_N_R::new(((self.bits >> 13) & 1) != 0)
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
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rssi_offset(&mut self) -> RSSI_OFFSET_W<0> {
        RSSI_OFFSET_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rssitrimcomplete_n(&mut self) -> RSSITRIMCOMPLETE_N_W<12> {
        RSSITRIMCOMPLETE_N_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn patrimcomplete_n(&mut self) -> PATRIMCOMPLETE_N_W<13> {
        PATRIMCOMPLETE_N_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_cc26_fe](index.html) module"]
pub struct CONFIG_CC26_FE_SPEC;
impl crate::RegisterSpec for CONFIG_CC26_FE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_cc26_fe::R](R) reader structure"]
impl crate::Readable for CONFIG_CC26_FE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_cc26_fe::W](W) writer structure"]
impl crate::Writable for CONFIG_CC26_FE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_CC26_FE to value 0x7000_0f00"]
impl crate::Resettable for CONFIG_CC26_FE_SPEC {
    const RESET_VALUE: Self::Ux = 0x7000_0f00;
}
