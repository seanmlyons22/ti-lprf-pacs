#[doc = "Register `FSM_ERA` reader"]
pub struct R(crate::R<FSM_ERA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_ERA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_ERA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_ERA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_ERA` writer"]
pub struct W(crate::W<FSM_ERA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_ERA_SPEC>;
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
impl From<crate::W<FSM_ERA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_ERA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERA_ADDR` reader - 22:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ERA_ADDR` writer - 22:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ERA_SPEC, u32, u32, 23, O>;
#[doc = "Field `ERA_BANK` reader - 25:23\\]
Internal. Only to be used through TI provided API."]
pub type ERA_BANK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERA_BANK` writer - 25:23\\]
Internal. Only to be used through TI provided API."]
pub type ERA_BANK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ERA_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED26` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED26` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_ERA_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_addr(&self) -> ERA_ADDR_R {
        ERA_ADDR_R::new(self.bits & 0x007f_ffff)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_bank(&self) -> ERA_BANK_R {
        ERA_BANK_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved26(&self) -> RESERVED26_R {
        RESERVED26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:22 - 22:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_addr(&mut self) -> ERA_ADDR_W<0> {
        ERA_ADDR_W::new(self)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn era_bank(&mut self) -> ERA_BANK_W<23> {
        ERA_BANK_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> RESERVED26_W<26> {
        RESERVED26_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_era](index.html) module"]
pub struct FSM_ERA_SPEC;
impl crate::RegisterSpec for FSM_ERA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_era::R](R) reader structure"]
impl crate::Readable for FSM_ERA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_era::W](W) writer structure"]
impl crate::Writable for FSM_ERA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_ERA to value 0"]
impl crate::Resettable for FSM_ERA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
