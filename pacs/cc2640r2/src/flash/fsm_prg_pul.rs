#[doc = "Register `FSM_PRG_PUL` reader"]
pub struct R(crate::R<FSM_PRG_PUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PRG_PUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PRG_PUL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PRG_PUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PRG_PUL` writer"]
pub struct W(crate::W<FSM_PRG_PUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PRG_PUL_SPEC>;
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
impl From<crate::W<FSM_PRG_PUL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PRG_PUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAX_PRG_PUL` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PRG_PUL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MAX_PRG_PUL` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type MAX_PRG_PUL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PRG_PUL_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED12` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED12` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PRG_PUL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BEG_EC_LEVEL` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type BEG_EC_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BEG_EC_LEVEL` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type BEG_EC_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PRG_PUL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_PRG_PUL_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn max_prg_pul(&self) -> MAX_PRG_PUL_R {
        MAX_PRG_PUL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn beg_ec_level(&self) -> BEG_EC_LEVEL_R {
        BEG_EC_LEVEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn max_prg_pul(&mut self) -> MAX_PRG_PUL_W<0> {
        MAX_PRG_PUL_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn beg_ec_level(&mut self) -> BEG_EC_LEVEL_W<16> {
        BEG_EC_LEVEL_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_prg_pul](index.html) module"]
pub struct FSM_PRG_PUL_SPEC;
impl crate::RegisterSpec for FSM_PRG_PUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_prg_pul::R](R) reader structure"]
impl crate::Readable for FSM_PRG_PUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_prg_pul::W](W) writer structure"]
impl crate::Writable for FSM_PRG_PUL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_PRG_PUL to value 0x0004_0032"]
impl crate::Resettable for FSM_PRG_PUL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0032;
}
