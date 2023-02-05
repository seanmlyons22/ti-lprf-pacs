#[doc = "Register `FLASH_P_R_PV` reader"]
pub struct R(crate::R<FLASH_P_R_PV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_P_R_PV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_P_R_PV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_P_R_PV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_P_R_PV` writer"]
pub struct W(crate::W<FLASH_P_R_PV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_P_R_PV_SPEC>;
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
impl From<crate::W<FLASH_P_R_PV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_P_R_PV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PVH2` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type PVH2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PVH2` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type PVH2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_P_R_PV_SPEC, u8, u8, 8, O>;
#[doc = "Field `PVH` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PVH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PVH` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PVH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_P_R_PV_SPEC, u8, u8, 8, O>;
#[doc = "Field `RH` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type RH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RH` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type RH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_P_R_PV_SPEC, u8, u8, 8, O>;
#[doc = "Field `PH` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PH` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type PH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_P_R_PV_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh2(&self) -> PVH2_R {
        PVH2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pvh(&self) -> PVH_R {
        PVH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rh(&self) -> RH_R {
        RH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ph(&self) -> PH_R {
        PH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pvh2(&mut self) -> PVH2_W<0> {
        PVH2_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pvh(&mut self) -> PVH_W<8> {
        PVH_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rh(&mut self) -> RH_W<16> {
        RH_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ph(&mut self) -> PH_W<24> {
        PH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_p_r_pv](index.html) module"]
pub struct FLASH_P_R_PV_SPEC;
impl crate::RegisterSpec for FLASH_P_R_PV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_p_r_pv::R](R) reader structure"]
impl crate::Readable for FLASH_P_R_PV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_p_r_pv::W](W) writer structure"]
impl crate::Writable for FLASH_P_R_PV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_P_R_PV to value 0x02c1_0200"]
impl crate::Resettable for FLASH_P_R_PV_SPEC {
    const RESET_VALUE: Self::Ux = 0x02c1_0200;
}
