#[doc = "Register `FSM_SAV_PPUL` reader"]
pub struct R(crate::R<FSM_SAV_PPUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_SAV_PPUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_SAV_PPUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_SAV_PPUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_SAV_PPUL` writer"]
pub struct W(crate::W<FSM_SAV_PPUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_SAV_PPUL_SPEC>;
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
impl From<crate::W<FSM_SAV_PPUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_SAV_PPUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAV_P_PUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SAV_P_PUL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAV_P_PUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type SAV_P_PUL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_SAV_PPUL_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_SAV_PPUL_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sav_p_pul(&self) -> SAV_P_PUL_R {
        SAV_P_PUL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sav_p_pul(&mut self) -> SAV_P_PUL_W<0> {
        SAV_P_PUL_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_sav_ppul](index.html) module"]
pub struct FSM_SAV_PPUL_SPEC;
impl crate::RegisterSpec for FSM_SAV_PPUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_sav_ppul::R](R) reader structure"]
impl crate::Readable for FSM_SAV_PPUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_sav_ppul::W](W) writer structure"]
impl crate::Writable for FSM_SAV_PPUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_SAV_PPUL to value 0"]
impl crate::Resettable for FSM_SAV_PPUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
