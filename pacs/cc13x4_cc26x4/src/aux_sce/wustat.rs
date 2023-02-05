#[doc = "Register `WUSTAT` reader"]
pub struct R(crate::R<WUSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUSTAT` writer"]
pub struct W(crate::W<WUSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUSTAT_SPEC>;
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
impl From<crate::W<WUSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV_SIGNALS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EV_SIGNALS_R = crate::FieldReader<u8, EV_SIGNALS_A>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_SIGNALS_A {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    SCEWEV_PROG = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AUX_ADC_FIFO_NOT_EMPTY = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AUX_TIMER1_EV_OR_IDLE = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AUX_TIMER0_EV_OR_IDLE = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AUX_TDC_DONE = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AUX_COMPB = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AUX_COMPA = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AUX_PROG_DLY_IDLE = 1,
}
impl From<EV_SIGNALS_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_SIGNALS_A) -> Self {
        variant as _
    }
}
impl EV_SIGNALS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_SIGNALS_A> {
        match self.bits {
            128 => Some(EV_SIGNALS_A::SCEWEV_PROG),
            64 => Some(EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY),
            32 => Some(EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE),
            16 => Some(EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE),
            8 => Some(EV_SIGNALS_A::AUX_TDC_DONE),
            4 => Some(EV_SIGNALS_A::AUX_COMPB),
            2 => Some(EV_SIGNALS_A::AUX_COMPA),
            1 => Some(EV_SIGNALS_A::AUX_PROG_DLY_IDLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCEWEV_PROG`"]
    #[inline(always)]
    pub fn is_scewev_prog(&self) -> bool {
        *self == EV_SIGNALS_A::SCEWEV_PROG
    }
    #[doc = "Checks if the value of the field is `AUX_ADC_FIFO_NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER1_EV_OR_IDLE`"]
    #[inline(always)]
    pub fn is_aux_timer1_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TIMER0_EV_OR_IDLE`"]
    #[inline(always)]
    pub fn is_aux_timer0_ev_or_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE
    }
    #[doc = "Checks if the value of the field is `AUX_TDC_DONE`"]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_TDC_DONE
    }
    #[doc = "Checks if the value of the field is `AUX_COMPB`"]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_COMPB
    }
    #[doc = "Checks if the value of the field is `AUX_COMPA`"]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_COMPA
    }
    #[doc = "Checks if the value of the field is `AUX_PROG_DLY_IDLE`"]
    #[inline(always)]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EV_SIGNALS_A::AUX_PROG_DLY_IDLE
    }
}
#[doc = "Field `EV_SIGNALS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EV_SIGNALS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, EV_SIGNALS_A, 8, O>;
impl<'a, const O: u8> EV_SIGNALS_W<'a, O> {
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scewev_prog(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::SCEWEV_PROG)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_ADC_FIFO_NOT_EMPTY)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer1_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TIMER1_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer0_ev_or_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TIMER0_EV_OR_IDLE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_TDC_DONE)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_COMPB)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_COMPA)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_prog_dly_idle(self) -> &'a mut W {
        self.variant(EV_SIGNALS_A::AUX_PROG_DLY_IDLE)
    }
}
#[doc = "Field `WU_SIGNAL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WU_SIGNAL_R = crate::BitReader<bool>;
#[doc = "Field `WU_SIGNAL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WU_SIGNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, u8, 7, O>;
#[doc = "Field `EXC_VECTOR` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type EXC_VECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXC_VECTOR` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type EXC_VECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED20` reader - 31:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WUSTAT_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EV_SIGNALS_R {
        EV_SIGNALS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WU_SIGNAL_R {
        WU_SIGNAL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> EXC_VECTOR_R {
        EXC_VECTOR_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ev_signals(&mut self) -> EV_SIGNALS_W<0> {
        EV_SIGNALS_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wu_signal(&mut self) -> WU_SIGNAL_W<8> {
        WU_SIGNAL_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exc_vector(&mut self) -> EXC_VECTOR_W<16> {
        EXC_VECTOR_W::new(self)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<19> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wustat](index.html) module"]
pub struct WUSTAT_SPEC;
impl crate::RegisterSpec for WUSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wustat::R](R) reader structure"]
impl crate::Readable for WUSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wustat::W](W) writer structure"]
impl crate::Writable for WUSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WUSTAT to value 0"]
impl crate::Resettable for WUSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
