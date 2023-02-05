#[doc = "Register `ERASE_CONF_1` reader"]
pub struct R(crate::R<ERASE_CONF_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERASE_CONF_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERASE_CONF_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERASE_CONF_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERASE_CONF_1` writer"]
pub struct W(crate::W<ERASE_CONF_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERASE_CONF_1_SPEC>;
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
impl From<crate::W<ERASE_CONF_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERASE_CONF_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEPROT_CCFG_N` reader - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
pub type WEPROT_CCFG_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_CCFG_N` writer - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
pub type WEPROT_CCFG_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERASE_CONF_1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
    #[inline(always)]
    pub fn weprot_ccfg_n(&self) -> WEPROT_CCFG_N_R {
        WEPROT_CCFG_N_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
WriteErase protect the CCFG sector Setting this bit = 0 will set FLASH:WEPROT_AUX_BY1.WEPROT_B0_CCFG_BY1 = 1 during boot and hence WriteErase protect the CCFG"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_ccfg_n(&mut self) -> WEPROT_CCFG_N_W<0> {
        WEPROT_CCFG_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Erase Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erase_conf_1](index.html) module"]
pub struct ERASE_CONF_1_SPEC;
impl crate::RegisterSpec for ERASE_CONF_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erase_conf_1::R](R) reader structure"]
impl crate::Readable for ERASE_CONF_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erase_conf_1::W](W) writer structure"]
impl crate::Writable for ERASE_CONF_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERASE_CONF_1 to value 0xffff_ffff"]
impl crate::Resettable for ERASE_CONF_1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
