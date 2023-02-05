#[doc = "Register `FSM_ERA_PW` reader"]
pub struct R(crate::R<FSM_ERA_PW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ERA_PW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ERA_PW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ERA_PW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ERA_PW` writer"]
pub struct W(crate::W<FSM_ERA_PW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ERA_PW_SPEC>;
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
impl From<crate::W<FSM_ERA_PW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ERA_PW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSM_ERA_PW` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ERA_PW_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FSM_ERA_PW` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ERA_PW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ERA_PW_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_era_pw(&self) -> FSM_ERA_PW_R {
        FSM_ERA_PW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_era_pw(&mut self) -> FSM_ERA_PW_W<0> {
        FSM_ERA_PW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_pw](index.html) module"]
pub struct FSM_ERA_PW_SPEC;
impl crate::RegisterSpec for FSM_ERA_PW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_era_pw::R](R) reader structure"]
impl crate::Readable for FSM_ERA_PW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_era_pw::W](W) writer structure"]
impl crate::Writable for FSM_ERA_PW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ERA_PW to value 0"]
impl crate::Resettable for FSM_ERA_PW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
