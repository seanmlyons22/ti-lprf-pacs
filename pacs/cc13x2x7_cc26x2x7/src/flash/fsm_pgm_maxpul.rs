#[doc = "Register `FSM_PGM_MAXPUL` reader"]
pub struct R(crate::R<FSM_PGM_MAXPUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PGM_MAXPUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PGM_MAXPUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PGM_MAXPUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PGM_MAXPUL` writer"]
pub struct W(crate::W<FSM_PGM_MAXPUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PGM_MAXPUL_SPEC>;
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
impl From<crate::W<FSM_PGM_MAXPUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PGM_MAXPUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSM_PGM_MAXPUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_PGM_MAXPUL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FSM_PGM_MAXPUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type FSM_PGM_MAXPUL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PGM_MAXPUL_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PGM_MAXPUL_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_pgm_maxpul(&self) -> FSM_PGM_MAXPUL_R {
        FSM_PGM_MAXPUL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_pgm_maxpul(&mut self) -> FSM_PGM_MAXPUL_W<0> {
        FSM_PGM_MAXPUL_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm_maxpul](index.html) module"]
pub struct FSM_PGM_MAXPUL_SPEC;
impl crate::RegisterSpec for FSM_PGM_MAXPUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pgm_maxpul::R](R) reader structure"]
impl crate::Readable for FSM_PGM_MAXPUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pgm_maxpul::W](W) writer structure"]
impl crate::Writable for FSM_PGM_MAXPUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_PGM_MAXPUL to value 0"]
impl crate::Resettable for FSM_PGM_MAXPUL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
