#[doc = "Register `FMC_REV_ID` reader"]
pub struct R(crate::R<FMC_REV_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_REV_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_REV_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_REV_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_REV_ID` writer"]
pub struct W(crate::W<FMC_REV_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_REV_ID_SPEC>;
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
impl From<crate::W<FMC_REV_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_REV_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONFIG_CRC` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type CONFIG_CRC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CONFIG_CRC` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type CONFIG_CRC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMC_REV_ID_SPEC, u16, u16, 12, O>;
#[doc = "Field `MOD_VERSION` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type MOD_VERSION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MOD_VERSION` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type MOD_VERSION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FMC_REV_ID_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn config_crc(&self) -> CONFIG_CRC_R {
        CONFIG_CRC_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mod_version(&self) -> MOD_VERSION_R {
        MOD_VERSION_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn config_crc(&mut self) -> CONFIG_CRC_W<0> {
        CONFIG_CRC_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mod_version(&mut self) -> MOD_VERSION_W<12> {
        MOD_VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_rev_id](index.html) module"]
pub struct FMC_REV_ID_SPEC;
impl crate::RegisterSpec for FMC_REV_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_rev_id::R](R) reader structure"]
impl crate::Readable for FMC_REV_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_rev_id::W](W) writer structure"]
impl crate::Writable for FMC_REV_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_REV_ID to value 0"]
impl crate::Resettable for FMC_REV_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
