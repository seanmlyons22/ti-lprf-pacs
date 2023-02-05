#[doc = "Register `FSM_PE_VH` reader"]
pub struct R(crate::R<FSM_PE_VH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PE_VH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PE_VH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PE_VH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PE_VH` writer"]
pub struct W(crate::W<FSM_PE_VH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PE_VH_SPEC>;
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
impl From<crate::W<FSM_PE_VH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PE_VH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERA_VH` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_VH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERA_VH` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ERA_VH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_PE_VH_SPEC, u8, u8, 8, O>;
#[doc = "Field `PGM_VH` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PGM_VH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PGM_VH` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type PGM_VH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_PE_VH_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PE_VH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn era_vh(&self) -> ERA_VH_R {
        ERA_VH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pgm_vh(&self) -> PGM_VH_R {
        PGM_VH_R::new(((self.bits >> 8) & 0xff) as u8)
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
    pub fn era_vh(&mut self) -> ERA_VH_W<0> {
        ERA_VH_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_vh(&mut self) -> PGM_VH_W<8> {
        PGM_VH_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pe_vh](index.html) module"]
pub struct FSM_PE_VH_SPEC;
impl crate::RegisterSpec for FSM_PE_VH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pe_vh::R](R) reader structure"]
impl crate::Readable for FSM_PE_VH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pe_vh::W](W) writer structure"]
impl crate::Writable for FSM_PE_VH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_PE_VH to value 0x0100"]
impl crate::Resettable for FSM_PE_VH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
