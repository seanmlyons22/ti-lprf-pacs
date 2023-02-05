#[doc = "Register `FSM_ERA_OH` reader"]
pub struct R(crate::R<FSM_ERA_OH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ERA_OH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ERA_OH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ERA_OH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ERA_OH` writer"]
pub struct W(crate::W<FSM_ERA_OH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ERA_OH_SPEC>;
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
impl From<crate::W<FSM_ERA_OH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ERA_OH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERA_OH` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_OH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ERA_OH` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_OH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ERA_OH_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_ERA_OH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_oh(&self) -> ERA_OH_R {
        ERA_OH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_oh(&mut self) -> ERA_OH_W<0> {
        ERA_OH_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era_oh](index.html) module"]
pub struct FSM_ERA_OH_SPEC;
impl crate::RegisterSpec for FSM_ERA_OH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_era_oh::R](R) reader structure"]
impl crate::Readable for FSM_ERA_OH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_era_oh::W](W) writer structure"]
impl crate::Writable for FSM_ERA_OH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ERA_OH to value 0x01"]
impl crate::Resettable for FSM_ERA_OH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
