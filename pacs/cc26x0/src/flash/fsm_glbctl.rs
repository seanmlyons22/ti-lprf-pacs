#[doc = "Register `FSM_GLBCTL` reader"]
pub struct R(crate::R<FSM_GLBCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_GLBCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_GLBCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_GLBCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_GLBCTL` writer"]
pub struct W(crate::W<FSM_GLBCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_GLBCTL_SPEC>;
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
impl From<crate::W<FSM_GLBCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_GLBCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKSEL` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type CLKSEL_R = crate::BitReader<bool>;
#[doc = "Field `CLKSEL` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type CLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_GLBCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_GLBCTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn clksel(&mut self) -> CLKSEL_W<0> {
        CLKSEL_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_glbctl](index.html) module"]
pub struct FSM_GLBCTL_SPEC;
impl crate::RegisterSpec for FSM_GLBCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_glbctl::R](R) reader structure"]
impl crate::Readable for FSM_GLBCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_glbctl::W](W) writer structure"]
impl crate::Writable for FSM_GLBCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_GLBCTL to value 0x01"]
impl crate::Resettable for FSM_GLBCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
