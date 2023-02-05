#[doc = "Register `FSM_CMD` reader"]
pub struct R(crate::R<FSM_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_CMD` writer"]
pub struct W(crate::W<FSM_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_CMD_SPEC>;
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
impl From<crate::W<FSM_CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSMCMD` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FSMCMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FSMCMD` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type FSMCMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_CMD_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_CMD_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsmcmd(&self) -> FSMCMD_R {
        FSMCMD_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsmcmd(&mut self) -> FSMCMD_W<0> {
        FSMCMD_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_cmd](index.html) module"]
pub struct FSM_CMD_SPEC;
impl crate::RegisterSpec for FSM_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_cmd::R](R) reader structure"]
impl crate::Readable for FSM_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_cmd::W](W) writer structure"]
impl crate::Writable for FSM_CMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_CMD to value 0"]
impl crate::Resettable for FSM_CMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
