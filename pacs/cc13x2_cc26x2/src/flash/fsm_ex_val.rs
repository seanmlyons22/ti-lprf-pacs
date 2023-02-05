#[doc = "Register `FSM_EX_VAL` reader"]
pub struct R(crate::R<FSM_EX_VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_EX_VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_EX_VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_EX_VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_EX_VAL` writer"]
pub struct W(crate::W<FSM_EX_VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_EX_VAL_SPEC>;
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
impl From<crate::W<FSM_EX_VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_EX_VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXE_VALD` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EXE_VALD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXE_VALD` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EXE_VALD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_EX_VAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `REP_VSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type REP_VSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REP_VSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type REP_VSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_EX_VAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_EX_VAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exe_vald(&self) -> EXE_VALD_R {
        EXE_VALD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rep_vsu(&self) -> REP_VSU_R {
        REP_VSU_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exe_vald(&mut self) -> EXE_VALD_W<0> {
        EXE_VALD_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rep_vsu(&mut self) -> REP_VSU_W<8> {
        REP_VSU_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_ex_val](index.html) module"]
pub struct FSM_EX_VAL_SPEC;
impl crate::RegisterSpec for FSM_EX_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_ex_val::R](R) reader structure"]
impl crate::Readable for FSM_EX_VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_ex_val::W](W) writer structure"]
impl crate::Writable for FSM_EX_VAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_EX_VAL to value 0x0301"]
impl crate::Resettable for FSM_EX_VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301;
}
