#[doc = "Register `FSM_ACC_PP` reader"]
pub struct R(crate::R<FSM_ACC_PP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ACC_PP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ACC_PP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ACC_PP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ACC_PP` writer"]
pub struct W(crate::W<FSM_ACC_PP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ACC_PP_SPEC>;
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
impl From<crate::W<FSM_ACC_PP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ACC_PP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSM_ACC_PP` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ACC_PP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FSM_ACC_PP` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ACC_PP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ACC_PP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_acc_pp(&self) -> FSM_ACC_PP_R {
        FSM_ACC_PP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_acc_pp(&mut self) -> FSM_ACC_PP_W<0> {
        FSM_ACC_PP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_acc_pp](index.html) module"]
pub struct FSM_ACC_PP_SPEC;
impl crate::RegisterSpec for FSM_ACC_PP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_acc_pp::R](R) reader structure"]
impl crate::Readable for FSM_ACC_PP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_acc_pp::W](W) writer structure"]
impl crate::Writable for FSM_ACC_PP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ACC_PP to value 0"]
impl crate::Resettable for FSM_ACC_PP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
