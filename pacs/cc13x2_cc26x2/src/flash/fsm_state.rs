#[doc = "Register `FSM_STATE` reader"]
pub struct R(crate::R<FSM_STATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_STATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_STATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_STATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_STATE` writer"]
pub struct W(crate::W<FSM_STATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_STATE_SPEC>;
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
impl From<crate::W<FSM_STATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_STATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSM_STATE_SPEC, u8, u8, 6, O>;
#[doc = "Field `OTP_ACT` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type OTP_ACT_R = crate::BitReader<bool>;
#[doc = "Field `OTP_ACT` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type OTP_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `TIOTP_ACT` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type TIOTP_ACT_R = crate::BitReader<bool>;
#[doc = "Field `TIOTP_ACT` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type TIOTP_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `FSM_ACT` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ACT_R = crate::BitReader<bool>;
#[doc = "Field `FSM_ACT` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FSM_ACT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED9` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `EXECUTEZ` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EXECUTEZ_R = crate::BitReader<bool>;
#[doc = "Field `EXECUTEZ` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type EXECUTEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `CTRLENZ` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type CTRLENZ_R = crate::BitReader<bool>;
#[doc = "Field `CTRLENZ` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type CTRLENZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_STATE_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FSM_STATE_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp_act(&self) -> OTP_ACT_R {
        OTP_ACT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tiotp_act(&self) -> TIOTP_ACT_R {
        TIOTP_ACT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fsm_act(&self) -> FSM_ACT_R {
        FSM_ACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn executez(&self) -> EXECUTEZ_R {
        EXECUTEZ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn otp_act(&mut self) -> OTP_ACT_W<6> {
        OTP_ACT_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tiotp_act(&mut self) -> TIOTP_ACT_W<7> {
        TIOTP_ACT_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fsm_act(&mut self) -> FSM_ACT_W<8> {
        FSM_ACT_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn executez(&mut self) -> EXECUTEZ_W<10> {
        EXECUTEZ_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlenz(&mut self) -> CTRLENZ_W<11> {
        CTRLENZ_W::new(self)
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_state](index.html) module"]
pub struct FSM_STATE_SPEC;
impl crate::RegisterSpec for FSM_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_state::R](R) reader structure"]
impl crate::Readable for FSM_STATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_state::W](W) writer structure"]
impl crate::Writable for FSM_STATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_STATE to value 0x0c00"]
impl crate::Resettable for FSM_STATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00;
}
