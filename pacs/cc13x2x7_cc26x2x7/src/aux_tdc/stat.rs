#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "5:0\\]
TDC state machine status.\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "46: Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    ForceStop = 46,
    #[doc = "30: Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    StartFall = 30,
    #[doc = "22: Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    WaitClrCntDone = 22,
    #[doc = "15: Current state is TDC_STATE_POR. This is the reset state."]
    Por = 15,
    #[doc = "14: Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    GetResult = 14,
    #[doc = "12: Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    WaitStopCntdwn = 12,
    #[doc = "8: Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    WaitStop = 8,
    #[doc = "7: Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    ClrCnt = 7,
    #[doc = "6: Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    Idle = 6,
    #[doc = "4: Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WaitStartStopCntEn = 4,
    #[doc = "0: Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    WaitStart = 0,
}
impl From<State> for u8 {
    #[inline(always)]
    fn from(variant: State) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for State {
    type Ux = u8;
}
impl crate::IsEnum for State {}
#[doc = "Field `STATE` reader - 5:0\\]
TDC state machine status."]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            46 => Some(State::ForceStop),
            30 => Some(State::StartFall),
            22 => Some(State::WaitClrCntDone),
            15 => Some(State::Por),
            14 => Some(State::GetResult),
            12 => Some(State::WaitStopCntdwn),
            8 => Some(State::WaitStop),
            7 => Some(State::ClrCnt),
            6 => Some(State::Idle),
            4 => Some(State::WaitStartStopCntEn),
            0 => Some(State::WaitStart),
            _ => None,
        }
    }
    #[doc = "Current state is TDC_FORCESTOP. You wrote ABORT to CTL.CMD to abort the TDC measurement."]
    #[inline(always)]
    pub fn is_force_stop(&self) -> bool {
        *self == State::ForceStop
    }
    #[doc = "Current state is TDC_WAIT_STARTFALL. The fast-counter circuit waits for a falling edge on the start event."]
    #[inline(always)]
    pub fn is_start_fall(&self) -> bool {
        *self == State::StartFall
    }
    #[doc = "Current state is TDC_STATE_WAIT_CLRCNT_DONE. The state machine waits for fast-counter circuit to finish reset."]
    #[inline(always)]
    pub fn is_wait_clr_cnt_done(&self) -> bool {
        *self == State::WaitClrCntDone
    }
    #[doc = "Current state is TDC_STATE_POR. This is the reset state."]
    #[inline(always)]
    pub fn is_por(&self) -> bool {
        *self == State::Por
    }
    #[doc = "Current state is TDC_STATE_GETRESULTS. The state machine copies the counter value from the fast-counter circuit."]
    #[inline(always)]
    pub fn is_get_result(&self) -> bool {
        *self == State::GetResult
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOPCNTDOWN. The fast-counter circuit looks for the stop condition. It will ignore a number of stop events configured in TRIGCNTLOAD.CNT."]
    #[inline(always)]
    pub fn is_wait_stop_cntdwn(&self) -> bool {
        *self == State::WaitStopCntdwn
    }
    #[doc = "Current state is TDC_STATE_WAIT_STOP. The state machine waits for the fast-counter circuit to stop."]
    #[inline(always)]
    pub fn is_wait_stop(&self) -> bool {
        *self == State::WaitStop
    }
    #[doc = "Current state is TDC_STATE_CLRCNT. The fast-counter circuit is reset."]
    #[inline(always)]
    pub fn is_clr_cnt(&self) -> bool {
        *self == State::ClrCnt
    }
    #[doc = "Current state is TDC_STATE_IDLE. This is the default state after reset and abortion. State will change when you write CTL.CMD to either RUN_SYNC_START or RUN."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
    #[doc = "Current state is TDC_STATE_WAIT_STARTSTOPCNTEN. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn is_wait_start_stop_cnt_en(&self) -> bool {
        *self == State::WaitStartStopCntEn
    }
    #[doc = "Current state is TDC_STATE_WAIT_START. The fast-counter circuit looks for the start condition. The state machine waits for the fast-counter to increment."]
    #[inline(always)]
    pub fn is_wait_start(&self) -> bool {
        *self == State::WaitStart
    }
}
#[doc = "Field `DONE` reader - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
pub type DoneR = crate::BitReader;
#[doc = "Field `SAT` reader - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
pub type SatR = crate::BitReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
TDC state machine status."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
TDC measurement complete flag. 0: TDC measurement has not yet completed. 1: TDC measurement has completed. This field clears when a new TDC measurement starts or when you write CLR_RESULT to CTL.CMD."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
TDC measurement saturation flag. 0: Conversion has not saturated. 1: Conversion stopped due to saturation. This field is cleared when a new measurement is started or when CLR_RESULT is written to CTL.CMD."]
    #[inline(always)]
    pub fn sat(&self) -> SatR {
        SatR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x06"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x06;
}
