#[doc = "Register `FSM_WR_ENA` reader"]
pub struct R(crate::R<FSM_WR_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_WR_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_WR_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_WR_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_WR_ENA` writer"]
pub struct W(crate::W<FSM_WR_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_WR_ENA_SPEC>;
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
impl From<crate::W<FSM_WR_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_WR_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WR_ENA` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type WR_ENA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WR_ENA` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type WR_ENA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_WR_ENA_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_WR_ENA_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wr_ena(&self) -> WR_ENA_R {
        WR_ENA_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wr_ena(&mut self) -> WR_ENA_W<0> {
        WR_ENA_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_wr_ena](index.html) module"]
pub struct FSM_WR_ENA_SPEC;
impl crate::RegisterSpec for FSM_WR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_wr_ena::R](R) reader structure"]
impl crate::Readable for FSM_WR_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_wr_ena::W](W) writer structure"]
impl crate::Writable for FSM_WR_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_WR_ENA to value 0x02"]
impl crate::Resettable for FSM_WR_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
