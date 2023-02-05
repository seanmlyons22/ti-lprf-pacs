#[doc = "Register `FSM_PUL_CNTR` reader"]
pub struct R(crate::R<FSM_PUL_CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PUL_CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PUL_CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PUL_CNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PUL_CNTR` writer"]
pub struct W(crate::W<FSM_PUL_CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PUL_CNTR_SPEC>;
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
impl From<crate::W<FSM_PUL_CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PUL_CNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUL_CNTR` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type PUL_CNTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PUL_CNTR` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type PUL_CNTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PUL_CNTR_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PUL_CNTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUR_EC_LEVEL` reader - 24:16\\]
Internal. Only to be used through TI provided API."]
pub type CUR_EC_LEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CUR_EC_LEVEL` writer - 24:16\\]
Internal. Only to be used through TI provided API."]
pub type CUR_EC_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PUL_CNTR_SPEC, u16, u16, 9, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PUL_CNTR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pul_cntr(&self) -> PUL_CNTR_R {
        PUL_CNTR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cur_ec_level(&self) -> CUR_EC_LEVEL_R {
        CUR_EC_LEVEL_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pul_cntr(&mut self) -> PUL_CNTR_W<0> {
        PUL_CNTR_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:24 - 24:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cur_ec_level(&mut self) -> CUR_EC_LEVEL_W<16> {
        CUR_EC_LEVEL_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pul_cntr](index.html) module"]
pub struct FSM_PUL_CNTR_SPEC;
impl crate::RegisterSpec for FSM_PUL_CNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pul_cntr::R](R) reader structure"]
impl crate::Readable for FSM_PUL_CNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pul_cntr::W](W) writer structure"]
impl crate::Writable for FSM_PUL_CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_PUL_CNTR to value 0"]
impl crate::Resettable for FSM_PUL_CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
