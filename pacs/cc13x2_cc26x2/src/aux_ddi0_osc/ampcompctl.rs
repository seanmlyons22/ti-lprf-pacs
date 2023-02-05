#[doc = "Register `AMPCOMPCTL` reader"]
pub struct R(crate::R<AMPCOMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMPCTL` writer"]
pub struct W(crate::W<AMPCOMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMPCTL_SPEC>;
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
impl From<crate::W<AMPCOMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_HPTOLP_OL_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_HPTOLP_OL_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CAP_STEP` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CAP_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAP_STEP` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CAP_STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_FINAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_FINAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 8, O>;
#[doc = "Field `IBIAS_INIT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_INIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS_INIT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `IBIAS_OFFSET` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIAS_OFFSET` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IBIAS_OFFSET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED24` reader - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPCTL_SPEC, u8, u8, 2, O>;
#[doc = "Field `AMPCOMP_SW_EN` reader - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_SW_EN_R = crate::BitReader<bool>;
#[doc = "Field `AMPCOMP_SW_EN` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_SW_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMPCOMPCTL_SPEC, bool, O>;
#[doc = "Field `AMPCOMP_SW_CTRL` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_SW_CTRL_R = crate::BitReader<bool>;
#[doc = "Field `AMPCOMP_SW_CTRL` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_SW_CTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMPCOMPCTL_SPEC, bool, O>;
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` reader - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_FSM_UPDATE_RATE_R = crate::FieldReader<u8, AMPCOMP_FSM_UPDATE_RATE_A>;
#[doc = "29:28\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMPCOMP_FSM_UPDATE_RATE_A {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _250KHZ = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _500KHZ = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _1MHZ = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _2MHZ = 0,
}
impl From<AMPCOMP_FSM_UPDATE_RATE_A> for u8 {
    #[inline(always)]
    fn from(variant: AMPCOMP_FSM_UPDATE_RATE_A) -> Self {
        variant as _
    }
}
impl AMPCOMP_FSM_UPDATE_RATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AMPCOMP_FSM_UPDATE_RATE_A {
        match self.bits {
            3 => AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ,
            2 => AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ,
            1 => AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ,
            0 => AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_250KHZ`"]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ
    }
    #[doc = "Checks if the value of the field is `_500KHZ`"]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_2MHZ`"]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        *self == AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ
    }
}
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` writer - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_FSM_UPDATE_RATE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, AMPCOMPCTL_SPEC, u8, AMPCOMP_FSM_UPDATE_RATE_A, 2, O>;
impl<'a, const O: u8> AMPCOMP_FSM_UPDATE_RATE_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_250KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_500KHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_1MHZ)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut W {
        self.variant(AMPCOMP_FSM_UPDATE_RATE_A::_2MHZ)
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_REQ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AMPCOMP_REQ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMPCOMPCTL_SPEC, bool, O>;
#[doc = "Field `SPARE31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE31_R = crate::BitReader<bool>;
#[doc = "Field `SPARE31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, AMPCOMPCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IBIASCAP_HPTOLP_OL_CNT_R {
        IBIASCAP_HPTOLP_OL_CNT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CAP_STEP_R {
        CAP_STEP_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LPM_IBIAS_WAIT_CNT_FINAL_R {
        LPM_IBIAS_WAIT_CNT_FINAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IBIAS_INIT_R {
        IBIAS_INIT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IBIAS_OFFSET_R {
        IBIAS_OFFSET_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_en(&self) -> AMPCOMP_SW_EN_R {
        AMPCOMP_SW_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&self) -> AMPCOMP_SW_CTRL_R {
        AMPCOMP_SW_CTRL_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&self) -> AMPCOMP_FSM_UPDATE_RATE_R {
        AMPCOMP_FSM_UPDATE_RATE_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AMPCOMP_REQ_MODE_R {
        AMPCOMP_REQ_MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> SPARE31_R {
        SPARE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IBIASCAP_HPTOLP_OL_CNT_W<0> {
        IBIASCAP_HPTOLP_OL_CNT_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cap_step(&mut self) -> CAP_STEP_W<4> {
        CAP_STEP_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LPM_IBIAS_WAIT_CNT_FINAL_W<8> {
        LPM_IBIAS_WAIT_CNT_FINAL_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_init(&mut self) -> IBIAS_INIT_W<16> {
        IBIAS_INIT_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_offset(&mut self) -> IBIAS_OFFSET_W<20> {
        IBIAS_OFFSET_W::new(self)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_sw_en(&mut self) -> AMPCOMP_SW_EN_W<26> {
        AMPCOMP_SW_EN_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_sw_ctrl(&mut self) -> AMPCOMP_SW_CTRL_W<27> {
        AMPCOMP_SW_CTRL_W::new(self)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_fsm_update_rate(&mut self) -> AMPCOMP_FSM_UPDATE_RATE_W<28> {
        AMPCOMP_FSM_UPDATE_RATE_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req_mode(&mut self) -> AMPCOMP_REQ_MODE_W<30> {
        AMPCOMP_REQ_MODE_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare31(&mut self) -> SPARE31_W<31> {
        SPARE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Amplitude Compensation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompctl](index.html) module"]
pub struct AMPCOMPCTL_SPEC;
impl crate::RegisterSpec for AMPCOMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcompctl::R](R) reader structure"]
impl crate::Readable for AMPCOMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcompctl::W](W) writer structure"]
impl crate::Writable for AMPCOMPCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCOMPCTL to value 0"]
impl crate::Resettable for AMPCOMPCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
