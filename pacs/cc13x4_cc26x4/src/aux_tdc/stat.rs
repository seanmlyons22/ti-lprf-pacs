#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - 5:0\\]
TDC state machine status."]
pub type STATE_R = crate::FieldReader<u8, STATE_A>;
#[doc = "5:0\\]
TDC state machine status.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "46: Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    FORCE_STOP = 46,
    #[doc = "30: Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    START_FALL = 30,
    #[doc = "22: Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    WAIT_CLR_CNT_DONE = 22,
    #[doc = "15: Current state is TDC_STATE_POR. This is the reset state."]
    POR = 15,
    #[doc = "14: Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    GET_RESULT = 14,
    #[doc = "12: Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WAIT_STOP_CNTDWN = 12,
    #[doc = "8: Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    WAIT_STOP = 8,
    #[doc = "7: Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    CLR_CNT = 7,
    #[doc = "6: Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    IDLE = 6,
    #[doc = "4: Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START_STOP_CNT_EN = 4,
    #[doc = "0: Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WAIT_START = 0,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
impl STATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            46 => Some(STATE_A::FORCE_STOP),
            30 => Some(STATE_A::START_FALL),
            22 => Some(STATE_A::WAIT_CLR_CNT_DONE),
            15 => Some(STATE_A::POR),
            14 => Some(STATE_A::GET_RESULT),
            12 => Some(STATE_A::WAIT_STOP_CNTDWN),
            8 => Some(STATE_A::WAIT_STOP),
            7 => Some(STATE_A::CLR_CNT),
            6 => Some(STATE_A::IDLE),
            4 => Some(STATE_A::WAIT_START_STOP_CNT_EN),
            0 => Some(STATE_A::WAIT_START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_STOP`"]
    #[inline(always)]
    pub fn is_force_stop(&self) -> bool {
        *self == STATE_A::FORCE_STOP
    }
    #[doc = "Checks if the value of the field is `START_FALL`"]
    #[inline(always)]
    pub fn is_start_fall(&self) -> bool {
        *self == STATE_A::START_FALL
    }
    #[doc = "Checks if the value of the field is `WAIT_CLR_CNT_DONE`"]
    #[inline(always)]
    pub fn is_wait_clr_cnt_done(&self) -> bool {
        *self == STATE_A::WAIT_CLR_CNT_DONE
    }
    #[doc = "Checks if the value of the field is `POR`"]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == STATE_A::POR
    }
    #[doc = "Checks if the value of the field is `GET_RESULT`"]
    #[inline(always)]
    pub fn is_get_result(&self) -> bool {
        *self == STATE_A::GET_RESULT
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP_CNTDWN`"]
    #[inline(always)]
    pub fn is_wait_stop_cntdwn(&self) -> bool {
        *self == STATE_A::WAIT_STOP_CNTDWN
    }
    #[doc = "Checks if the value of the field is `WAIT_STOP`"]
    #[inline(always)]
    pub fn is_wait_stop(&self) -> bool {
        *self == STATE_A::WAIT_STOP
    }
    #[doc = "Checks if the value of the field is `CLR_CNT`"]
    #[inline(always)]
    pub fn is_clr_cnt(&self) -> bool {
        *self == STATE_A::CLR_CNT
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == STATE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT_START_STOP_CNT_EN`"]
    #[inline(always)]
    pub fn is_wait_start_stop_cnt_en(&self) -> bool {
        *self == STATE_A::WAIT_START_STOP_CNT_EN
    }
    #[doc = "Checks if the value of the field is `WAIT_START`"]
    #[inline(always)]
    pub fn is_wait_start(&self) -> bool {
        *self == STATE_A::WAIT_START
    }
}
#[doc = "Field `STATE` writer - 5:0\\]
TDC state machine status."]
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u8, STATE_A, 6, O>;
impl<'a, const O: u8> STATE_W<'a, O> {
    #[doc = "Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    #[inline(always)]
    pub fn force_stop(self) -> &'a mut W {
        self.variant(STATE_A::FORCE_STOP)
    }
    #[doc = "Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    #[inline(always)]
    pub fn start_fall(self) -> &'a mut W {
        self.variant(STATE_A::START_FALL)
    }
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    #[inline(always)]
    pub fn wait_clr_cnt_done(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_CLR_CNT_DONE)
    }
    #[doc = "Current state is TDC_STATE_POR. This is the reset state."]
    #[inline(always)]
    pub fn por(self) -> &'a mut W {
        self.variant(STATE_A::POR)
    }
    #[doc = "Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    #[inline(always)]
    pub fn get_result(self) -> &'a mut W {
        self.variant(STATE_A::GET_RESULT)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    #[inline(always)]
    pub fn wait_stop_cntdwn(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_STOP_CNTDWN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    #[inline(always)]
    pub fn wait_stop(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_STOP)
    }
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    #[inline(always)]
    pub fn clr_cnt(self) -> &'a mut W {
        self.variant(STATE_A::CLR_CNT)
    }
    #[doc = "Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(STATE_A::IDLE)
    }
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn wait_start_stop_cnt_en(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_START_STOP_CNT_EN)
    }
    #[doc = "Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn wait_start(self) -> &'a mut W {
        self.variant(STATE_A::WAIT_START)
    }
}
#[doc = "Field `DONE` reader - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `SAT` reader - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
pub type SAT_R = crate::BitReader<bool>;
#[doc = "Field `SAT` writer - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
pub type SAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
TDC state machine status."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    pub fn sat(&self) -> SAT_R {
        SAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
TDC state machine status."]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<0> {
        STATE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<6> {
        DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    #[must_use]
    pub fn sat(&mut self) -> SAT_W<7> {
        SAT_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x06"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
