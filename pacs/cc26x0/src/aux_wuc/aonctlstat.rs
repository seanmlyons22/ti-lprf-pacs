#[doc = "Register `AONCTLSTAT` reader"]
pub struct R(crate::R<AONCTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AONCTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AONCTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AONCTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AONCTLSTAT` writer"]
pub struct W(crate::W<AONCTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AONCTLSTAT_SPEC>;
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
impl From<crate::W<AONCTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AONCTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCE_RUN_EN` reader - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub type SCE_RUN_EN_R = crate::BitReader<bool>;
#[doc = "Field `SCE_RUN_EN` writer - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub type SCE_RUN_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AONCTLSTAT_SPEC, bool, O>;
#[doc = "Field `AUX_FORCE_ON` reader - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub type AUX_FORCE_ON_R = crate::BitReader<bool>;
#[doc = "Field `AUX_FORCE_ON` writer - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub type AUX_FORCE_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, AONCTLSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SCE_RUN_EN_R {
        SCE_RUN_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AUX_FORCE_ON_R {
        AUX_FORCE_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    #[must_use]
    pub fn sce_run_en(&mut self) -> SCE_RUN_EN_W<0> {
        SCE_RUN_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    #[must_use]
    pub fn aux_force_on(&mut self) -> AUX_FORCE_ON_W<1> {
        AUX_FORCE_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aonctlstat](index.html) module"]
pub struct AONCTLSTAT_SPEC;
impl crate::RegisterSpec for AONCTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aonctlstat::R](R) reader structure"]
impl crate::Readable for AONCTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aonctlstat::W](W) writer structure"]
impl crate::Writable for AONCTLSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AONCTLSTAT to value 0"]
impl crate::Resettable for AONCTLSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
