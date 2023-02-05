#[doc = "Register `FSM_TIMER` reader"]
pub struct R(crate::R<FSM_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_TIMER` writer"]
pub struct W(crate::W<FSM_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_TIMER_SPEC>;
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
impl From<crate::W<FSM_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSM_TIMER` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_TIMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FSM_TIMER` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_TIMER_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_timer(&self) -> FSM_TIMER_R {
        FSM_TIMER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_timer(&mut self) -> FSM_TIMER_W<0> {
        FSM_TIMER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_timer](index.html) module"]
pub struct FSM_TIMER_SPEC;
impl crate::RegisterSpec for FSM_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_timer::R](R) reader structure"]
impl crate::Readable for FSM_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_timer::W](W) writer structure"]
impl crate::Writable for FSM_TIMER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_TIMER to value 0"]
impl crate::Resettable for FSM_TIMER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
