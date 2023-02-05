#[doc = "Register `FSM_EXECUTE` reader"]
pub struct R(crate::R<FSM_EXECUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_EXECUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_EXECUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_EXECUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_EXECUTE` writer"]
pub struct W(crate::W<FSM_EXECUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_EXECUTE_SPEC>;
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
impl From<crate::W<FSM_EXECUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_EXECUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSMEXECUTE` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type FSMEXECUTE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSMEXECUTE` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type FSMEXECUTE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_EXECUTE_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED5` reader - 15:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED5` writer - 15:5\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_EXECUTE_SPEC, u16, u16, 11, O>;
#[doc = "Field `SUSPEND_NOW` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type SUSPEND_NOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUSPEND_NOW` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type SUSPEND_NOW_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_EXECUTE_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_EXECUTE_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmexecute(&self) -> FSMEXECUTE_R {
        FSMEXECUTE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn suspend_now(&self) -> SUSPEND_NOW_R {
        SUSPEND_NOW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsmexecute(&mut self) -> FSMEXECUTE_W<0> {
        FSMEXECUTE_W::new(self)
    }
    #[doc = "Bits 5:15 - 15:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn suspend_now(&mut self) -> SUSPEND_NOW_W<16> {
        SUSPEND_NOW_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_execute](index.html) module"]
pub struct FSM_EXECUTE_SPEC;
impl crate::RegisterSpec for FSM_EXECUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_execute::R](R) reader structure"]
impl crate::Readable for FSM_EXECUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_execute::W](W) writer structure"]
impl crate::Writable for FSM_EXECUTE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_EXECUTE to value 0x000a_000a"]
impl crate::Resettable for FSM_EXECUTE_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_000a;
}
