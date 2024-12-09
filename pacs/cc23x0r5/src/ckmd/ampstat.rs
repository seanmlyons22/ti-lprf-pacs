#[doc = "Register `AMPSTAT` reader"]
pub type R = crate::R<AmpstatSpec>;
#[doc = "Register `AMPSTAT` writer"]
pub type W = crate::W<AmpstatSpec>;
#[doc = "Field `AMPGOOD` reader - 0:0\\]
HFXT amplitude good"]
pub type AmpgoodR = crate::BitReader;
#[doc = "Field `CTRLATTARGET` reader - 1:1\\]
HFXT control values match target values. This applies to IREF, Q1CAP, Q2CAP values."]
pub type CtrlattargetR = crate::BitReader;
#[doc = "Field `Q1CAP` reader - 7:2\\]
Current Q1CAP control value."]
pub type Q1capR = crate::FieldReader;
#[doc = "Field `Q2CAP` reader - 13:8\\]
Current Q2CAP control value."]
pub type Q2capR = crate::FieldReader;
#[doc = "Field `IREF` reader - 17:14\\]
Current IREF control value."]
pub type IrefR = crate::FieldReader;
#[doc = "Field `IDAC` reader - 24:18\\]
Current IDAC control value."]
pub type IdacR = crate::FieldReader;
#[doc = "28:25\\]
Current AMPCOMP FSM state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    #[doc = "15: Settled state"]
    Settled = 15,
    #[doc = "14: Amplitude up correction"]
    Updateup = 14,
    #[doc = "12: TCXO settled state"]
    Txcomode = 12,
    #[doc = "10: First shutdown state"]
    Shutdn0 = 10,
    #[doc = "7: Post injection settle wait"]
    Injwait = 7,
    #[doc = "6: Amplitude down correction"]
    Updatedn = 6,
    #[doc = "5: Initial amplitude ramping with HFXTINIT values"]
    Ramp0 = 5,
    #[doc = "4: Transition to HFXTTARG values"]
    Ramp1 = 4,
    #[doc = "3: Injecting HFOSC for fast startup"]
    Inject = 3,
    #[doc = "2: Second shutdown state"]
    Shutdn1 = 2,
    #[doc = "1: Starting LDO"]
    Ldostart = 1,
    #[doc = "0: FSM in idle state"]
    Idle = 0,
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
#[doc = "Field `STATE` reader - 28:25\\]
Current AMPCOMP FSM state."]
pub type StateR = crate::FieldReader<State>;
impl StateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<State> {
        match self.bits {
            15 => Some(State::Settled),
            14 => Some(State::Updateup),
            12 => Some(State::Txcomode),
            10 => Some(State::Shutdn0),
            7 => Some(State::Injwait),
            6 => Some(State::Updatedn),
            5 => Some(State::Ramp0),
            4 => Some(State::Ramp1),
            3 => Some(State::Inject),
            2 => Some(State::Shutdn1),
            1 => Some(State::Ldostart),
            0 => Some(State::Idle),
            _ => None,
        }
    }
    #[doc = "Settled state"]
    #[inline(always)]
    pub fn is_settled(&self) -> bool {
        *self == State::Settled
    }
    #[doc = "Amplitude up correction"]
    #[inline(always)]
    pub fn is_updateup(&self) -> bool {
        *self == State::Updateup
    }
    #[doc = "TCXO settled state"]
    #[inline(always)]
    pub fn is_txcomode(&self) -> bool {
        *self == State::Txcomode
    }
    #[doc = "First shutdown state"]
    #[inline(always)]
    pub fn is_shutdn0(&self) -> bool {
        *self == State::Shutdn0
    }
    #[doc = "Post injection settle wait"]
    #[inline(always)]
    pub fn is_injwait(&self) -> bool {
        *self == State::Injwait
    }
    #[doc = "Amplitude down correction"]
    #[inline(always)]
    pub fn is_updatedn(&self) -> bool {
        *self == State::Updatedn
    }
    #[doc = "Initial amplitude ramping with HFXTINIT values"]
    #[inline(always)]
    pub fn is_ramp0(&self) -> bool {
        *self == State::Ramp0
    }
    #[doc = "Transition to HFXTTARG values"]
    #[inline(always)]
    pub fn is_ramp1(&self) -> bool {
        *self == State::Ramp1
    }
    #[doc = "Injecting HFOSC for fast startup"]
    #[inline(always)]
    pub fn is_inject(&self) -> bool {
        *self == State::Inject
    }
    #[doc = "Second shutdown state"]
    #[inline(always)]
    pub fn is_shutdn1(&self) -> bool {
        *self == State::Shutdn1
    }
    #[doc = "Starting LDO"]
    #[inline(always)]
    pub fn is_ldostart(&self) -> bool {
        *self == State::Ldostart
    }
    #[doc = "FSM in idle state"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == State::Idle
    }
}
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
HFXT amplitude good"]
    #[inline(always)]
    pub fn ampgood(&self) -> AmpgoodR {
        AmpgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
HFXT control values match target values. This applies to IREF, Q1CAP, Q2CAP values."]
    #[inline(always)]
    pub fn ctrlattarget(&self) -> CtrlattargetR {
        CtrlattargetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Current Q1CAP control value."]
    #[inline(always)]
    pub fn q1cap(&self) -> Q1capR {
        Q1capR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Current Q2CAP control value."]
    #[inline(always)]
    pub fn q2cap(&self) -> Q2capR {
        Q2capR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Current IREF control value."]
    #[inline(always)]
    pub fn iref(&self) -> IrefR {
        IrefR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:24 - 24:18\\]
Current IDAC control value."]
    #[inline(always)]
    pub fn idac(&self) -> IdacR {
        IdacR::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bits 25:28 - 28:25\\]
Current AMPCOMP FSM state."]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {}
#[doc = "HFXT Amplitude Compensation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpstatSpec;
impl crate::RegisterSpec for AmpstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampstat::R`](R) reader structure"]
impl crate::Readable for AmpstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ampstat::W`](W) writer structure"]
impl crate::Writable for AmpstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPSTAT to value 0"]
impl crate::Resettable for AmpstatSpec {
    const RESET_VALUE: u32 = 0;
}
