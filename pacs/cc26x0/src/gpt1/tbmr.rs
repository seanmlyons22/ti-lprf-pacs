#[doc = "Register `TBMR` reader"]
pub struct R(crate::R<TBMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBMR` writer"]
pub struct W(crate::W<TBMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBMR_SPEC>;
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
impl From<crate::W<TBMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBMR` reader - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TBMR_R = crate::FieldReader<u8, TBMR_A>;
#[doc = "1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TBMR_A {
    #[doc = "3: Capture mode"]
    CAPTURE = 3,
    #[doc = "2: Periodic Timer mode"]
    PERIODIC = 2,
    #[doc = "1: One-Shot Timer mode"]
    ONE_SHOT = 1,
}
impl From<TBMR_A> for u8 {
    #[inline(always)]
    fn from(variant: TBMR_A) -> Self {
        variant as _
    }
}
impl TBMR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TBMR_A> {
        match self.bits {
            3 => Some(TBMR_A::CAPTURE),
            2 => Some(TBMR_A::PERIODIC),
            1 => Some(TBMR_A::ONE_SHOT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CAPTURE`"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == TBMR_A::CAPTURE
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == TBMR_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == TBMR_A::ONE_SHOT
    }
}
#[doc = "Field `TBMR` writer - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TBMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBMR_SPEC, u8, TBMR_A, 2, O>;
impl<'a, const O: u8> TBMR_W<'a, O> {
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut W {
        self.variant(TBMR_A::CAPTURE)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TBMR_A::PERIODIC)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(TBMR_A::ONE_SHOT)
    }
}
#[doc = "Field `TBCM` reader - 2:2\\]
GPT Timer B Capture Mode"]
pub type TBCM_R = crate::BitReader<TBCM_A>;
#[doc = "2:2\\]
GPT Timer B Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCM_A {
    #[doc = "1: Edge-Time mode"]
    EDGTIME = 1,
    #[doc = "0: Edge-Count mode"]
    EDGCNT = 0,
}
impl From<TBCM_A> for bool {
    #[inline(always)]
    fn from(variant: TBCM_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCM_A {
        match self.bits {
            true => TBCM_A::EDGTIME,
            false => TBCM_A::EDGCNT,
        }
    }
    #[doc = "Checks if the value of the field is `EDGTIME`"]
    #[inline(always)]
    pub fn is_edgtime(&self) -> bool {
        *self == TBCM_A::EDGTIME
    }
    #[doc = "Checks if the value of the field is `EDGCNT`"]
    #[inline(always)]
    pub fn is_edgcnt(&self) -> bool {
        *self == TBCM_A::EDGCNT
    }
}
#[doc = "Field `TBCM` writer - 2:2\\]
GPT Timer B Capture Mode"]
pub type TBCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBCM_A, O>;
impl<'a, const O: u8> TBCM_W<'a, O> {
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn edgtime(self) -> &'a mut W {
        self.variant(TBCM_A::EDGTIME)
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn edgcnt(self) -> &'a mut W {
        self.variant(TBCM_A::EDGCNT)
    }
}
#[doc = "Field `TBAMS` reader - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
pub type TBAMS_R = crate::BitReader<TBAMS_A>;
#[doc = "3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBAMS_A {
    #[doc = "1: PWM mode is enabled"]
    PWM = 1,
    #[doc = "0: Capture/Compare mode is enabled."]
    CAP_COMP = 0,
}
impl From<TBAMS_A> for bool {
    #[inline(always)]
    fn from(variant: TBAMS_A) -> Self {
        variant as u8 != 0
    }
}
impl TBAMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBAMS_A {
        match self.bits {
            true => TBAMS_A::PWM,
            false => TBAMS_A::CAP_COMP,
        }
    }
    #[doc = "Checks if the value of the field is `PWM`"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == TBAMS_A::PWM
    }
    #[doc = "Checks if the value of the field is `CAP_COMP`"]
    #[inline(always)]
    pub fn is_cap_comp(&self) -> bool {
        *self == TBAMS_A::CAP_COMP
    }
}
#[doc = "Field `TBAMS` writer - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
pub type TBAMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBAMS_A, O>;
impl<'a, const O: u8> TBAMS_W<'a, O> {
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut W {
        self.variant(TBAMS_A::PWM)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn cap_comp(self) -> &'a mut W {
        self.variant(TBAMS_A::CAP_COMP)
    }
}
#[doc = "Field `TBCDIR` reader - 4:4\\]
GPT Timer B Count Direction"]
pub type TBCDIR_R = crate::BitReader<TBCDIR_A>;
#[doc = "4:4\\]
GPT Timer B Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCDIR_A {
    #[doc = "1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    UP = 1,
    #[doc = "0: The timer counts down."]
    DOWN = 0,
}
impl From<TBCDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TBCDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCDIR_A {
        match self.bits {
            true => TBCDIR_A::UP,
            false => TBCDIR_A::DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `UP`"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == TBCDIR_A::UP
    }
    #[doc = "Checks if the value of the field is `DOWN`"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == TBCDIR_A::DOWN
    }
}
#[doc = "Field `TBCDIR` writer - 4:4\\]
GPT Timer B Count Direction"]
pub type TBCDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBCDIR_A, O>;
impl<'a, const O: u8> TBCDIR_W<'a, O> {
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn up(self) -> &'a mut W {
        self.variant(TBCDIR_A::UP)
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut W {
        self.variant(TBCDIR_A::DOWN)
    }
}
#[doc = "Field `TBMIE` reader - 5:5\\]
GPT Timer B Match Interrupt Enable."]
pub type TBMIE_R = crate::BitReader<TBMIE_A>;
#[doc = "5:5\\]
GPT Timer B Match Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBMIE_A {
    #[doc = "1: An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    EN = 1,
    #[doc = "0: The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    DIS = 0,
}
impl From<TBMIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TBMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBMIE_A {
        match self.bits {
            true => TBMIE_A::EN,
            false => TBMIE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBMIE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBMIE_A::DIS
    }
}
#[doc = "Field `TBMIE` writer - 5:5\\]
GPT Timer B Match Interrupt Enable."]
pub type TBMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBMIE_A, O>;
impl<'a, const O: u8> TBMIE_W<'a, O> {
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBMIE_A::EN)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBMIE_A::DIS)
    }
}
#[doc = "Field `TBWOT` reader - 6:6\\]
GPT Timer B Wait-On-Trigger"]
pub type TBWOT_R = crate::BitReader<TBWOT_A>;
#[doc = "6:6\\]
GPT Timer B Wait-On-Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBWOT_A {
    #[doc = "1: If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    WAIT = 1,
    #[doc = "0: Timer B begins counting as soon as it is enabled."]
    NOWAIT = 0,
}
impl From<TBWOT_A> for bool {
    #[inline(always)]
    fn from(variant: TBWOT_A) -> Self {
        variant as u8 != 0
    }
}
impl TBWOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBWOT_A {
        match self.bits {
            true => TBWOT_A::WAIT,
            false => TBWOT_A::NOWAIT,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == TBWOT_A::WAIT
    }
    #[doc = "Checks if the value of the field is `NOWAIT`"]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == TBWOT_A::NOWAIT
    }
}
#[doc = "Field `TBWOT` writer - 6:6\\]
GPT Timer B Wait-On-Trigger"]
pub type TBWOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBWOT_A, O>;
impl<'a, const O: u8> TBWOT_W<'a, O> {
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(TBWOT_A::WAIT)
    }
    #[doc = "Timer B begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut W {
        self.variant(TBWOT_A::NOWAIT)
    }
}
#[doc = "Field `TBSNAPS` reader - 7:7\\]
GPT Timer B Snap-Shot Mode"]
pub type TBSNAPS_R = crate::BitReader<TBSNAPS_A>;
#[doc = "7:7\\]
GPT Timer B Snap-Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBSNAPS_A {
    #[doc = "1: If Timer B is configured in the periodic mode"]
    EN = 1,
    #[doc = "0: Snap-shot mode is disabled."]
    DIS = 0,
}
impl From<TBSNAPS_A> for bool {
    #[inline(always)]
    fn from(variant: TBSNAPS_A) -> Self {
        variant as u8 != 0
    }
}
impl TBSNAPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBSNAPS_A {
        match self.bits {
            true => TBSNAPS_A::EN,
            false => TBSNAPS_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBSNAPS_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBSNAPS_A::DIS
    }
}
#[doc = "Field `TBSNAPS` writer - 7:7\\]
GPT Timer B Snap-Shot Mode"]
pub type TBSNAPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBSNAPS_A, O>;
impl<'a, const O: u8> TBSNAPS_W<'a, O> {
    #[doc = "If Timer B is configured in the periodic mode"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBSNAPS_A::EN)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBSNAPS_A::DIS)
    }
}
#[doc = "Field `TBILD` reader - 8:8\\]
GPT Timer B PWM Interval Load Write"]
pub type TBILD_R = crate::BitReader<TBILD_A>;
#[doc = "8:8\\]
GPT Timer B PWM Interval Load Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBILD_A {
    #[doc = "1: Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    TOUPDATE = 1,
    #[doc = "0: Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    CYCLEUPDATE = 0,
}
impl From<TBILD_A> for bool {
    #[inline(always)]
    fn from(variant: TBILD_A) -> Self {
        variant as u8 != 0
    }
}
impl TBILD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBILD_A {
        match self.bits {
            true => TBILD_A::TOUPDATE,
            false => TBILD_A::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == TBILD_A::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TBILD_A::CYCLEUPDATE
    }
}
#[doc = "Field `TBILD` writer - 8:8\\]
GPT Timer B PWM Interval Load Write"]
pub type TBILD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBILD_A, O>;
impl<'a, const O: u8> TBILD_W<'a, O> {
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TBILD_A::TOUPDATE)
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TBILD_A::CYCLEUPDATE)
    }
}
#[doc = "Field `TBPWMIE` reader - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TBPWMIE_R = crate::BitReader<TBPWMIE_A>;
#[doc = "9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBPWMIE_A {
    #[doc = "1: Interrupt is enabled. This bit is only valid in PWM mode."]
    EN = 1,
    #[doc = "0: Interrupt is disabled."]
    DIS = 0,
}
impl From<TBPWMIE_A> for bool {
    #[inline(always)]
    fn from(variant: TBPWMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TBPWMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBPWMIE_A {
        match self.bits {
            true => TBPWMIE_A::EN,
            false => TBPWMIE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TBPWMIE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TBPWMIE_A::DIS
    }
}
#[doc = "Field `TBPWMIE` writer - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TBPWMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBPWMIE_A, O>;
impl<'a, const O: u8> TBPWMIE_W<'a, O> {
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TBPWMIE_A::EN)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TBPWMIE_A::DIS)
    }
}
#[doc = "Field `TBMRSU` reader - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
pub type TBMRSU_R = crate::BitReader<TBMRSU_A>;
#[doc = "10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBMRSU_A {
    #[doc = "1: Update TBMATCHR and TBPR, if used, on the next time-out."]
    TOUPDATE = 1,
    #[doc = "0: Update TBMATCHR and TBPR, if used, on the next cycle."]
    CYCLEUPDATE = 0,
}
impl From<TBMRSU_A> for bool {
    #[inline(always)]
    fn from(variant: TBMRSU_A) -> Self {
        variant as u8 != 0
    }
}
impl TBMRSU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBMRSU_A {
        match self.bits {
            true => TBMRSU_A::TOUPDATE,
            false => TBMRSU_A::CYCLEUPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `TOUPDATE`"]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == TBMRSU_A::TOUPDATE
    }
    #[doc = "Checks if the value of the field is `CYCLEUPDATE`"]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == TBMRSU_A::CYCLEUPDATE
    }
}
#[doc = "Field `TBMRSU` writer - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
pub type TBMRSU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBMRSU_A, O>;
impl<'a, const O: u8> TBMRSU_W<'a, O> {
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut W {
        self.variant(TBMRSU_A::TOUPDATE)
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut W {
        self.variant(TBMRSU_A::CYCLEUPDATE)
    }
}
#[doc = "Field `TBPLO` reader - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TBPLO_R = crate::BitReader<TBPLO_A>;
#[doc = "11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBPLO_A {
    #[doc = "1: CCP output pin is set to 1 on time-out"]
    CCP_ON_TO = 1,
    #[doc = "0: Legacy operation"]
    LEGACY = 0,
}
impl From<TBPLO_A> for bool {
    #[inline(always)]
    fn from(variant: TBPLO_A) -> Self {
        variant as u8 != 0
    }
}
impl TBPLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBPLO_A {
        match self.bits {
            true => TBPLO_A::CCP_ON_TO,
            false => TBPLO_A::LEGACY,
        }
    }
    #[doc = "Checks if the value of the field is `CCP_ON_TO`"]
    #[inline(always)]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == TBPLO_A::CCP_ON_TO
    }
    #[doc = "Checks if the value of the field is `LEGACY`"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == TBPLO_A::LEGACY
    }
}
#[doc = "Field `TBPLO` writer - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TBPLO_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBPLO_A, O>;
impl<'a, const O: u8> TBPLO_W<'a, O> {
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn ccp_on_to(self) -> &'a mut W {
        self.variant(TBPLO_A::CCP_ON_TO)
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut W {
        self.variant(TBPLO_A::LEGACY)
    }
}
#[doc = "Field `TBCINTD` reader - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
pub type TBCINTD_R = crate::BitReader<TBCINTD_A>;
#[doc = "12:12\\]
One-Shot/Periodic Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBCINTD_A {
    #[doc = "1: Mask Time-Out Interrupt"]
    DIS_TO_INTR = 1,
    #[doc = "0: Normal Time-Out Interrupt"]
    EN_TO_INTR = 0,
}
impl From<TBCINTD_A> for bool {
    #[inline(always)]
    fn from(variant: TBCINTD_A) -> Self {
        variant as u8 != 0
    }
}
impl TBCINTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TBCINTD_A {
        match self.bits {
            true => TBCINTD_A::DIS_TO_INTR,
            false => TBCINTD_A::EN_TO_INTR,
        }
    }
    #[doc = "Checks if the value of the field is `DIS_TO_INTR`"]
    #[inline(always)]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == TBCINTD_A::DIS_TO_INTR
    }
    #[doc = "Checks if the value of the field is `EN_TO_INTR`"]
    #[inline(always)]
    pub fn is_en_to_intr(&self) -> bool {
        *self == TBCINTD_A::EN_TO_INTR
    }
}
#[doc = "Field `TBCINTD` writer - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
pub type TBCINTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TBMR_SPEC, TBCINTD_A, O>;
impl<'a, const O: u8> TBCINTD_W<'a, O> {
    #[doc = "Mask Time-Out Interrupt"]
    #[inline(always)]
    pub fn dis_to_intr(self) -> &'a mut W {
        self.variant(TBCINTD_A::DIS_TO_INTR)
    }
    #[doc = "Normal Time-Out Interrupt"]
    #[inline(always)]
    pub fn en_to_intr(self) -> &'a mut W {
        self.variant(TBCINTD_A::EN_TO_INTR)
    }
}
#[doc = "Field `TCACT` reader - 15:13\\]
Timer Compare Action Select"]
pub type TCACT_R = crate::FieldReader<u8, TCACT_A>;
#[doc = "15:13\\]
Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCACT_A {
    #[doc = "7: Clear CCP output pin immediately and set on Time-Out"]
    CLRSET_ON_TO = 7,
    #[doc = "6: Set CCP output pin immediately and clear on Time-Out"]
    SETCLR_ON_TO = 6,
    #[doc = "5: Clear CCP output pin immediately and toggle on Time-Out"]
    CLRTOG_ON_TO = 5,
    #[doc = "4: Set CCP output pin immediately and toggle on Time-Out"]
    SETTOG_ON_TO = 4,
    #[doc = "3: Set CCP output pin on Time-Out"]
    SET_ON_TO = 3,
    #[doc = "2: Clear CCP output pin on Time-Out"]
    CLR_ON_TO = 2,
    #[doc = "1: Toggle State on Time-Out"]
    TOG_ON_TO = 1,
    #[doc = "0: Disable compare operations"]
    DIS_CMP = 0,
}
impl From<TCACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCACT_A) -> Self {
        variant as _
    }
}
impl TCACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCACT_A {
        match self.bits {
            7 => TCACT_A::CLRSET_ON_TO,
            6 => TCACT_A::SETCLR_ON_TO,
            5 => TCACT_A::CLRTOG_ON_TO,
            4 => TCACT_A::SETTOG_ON_TO,
            3 => TCACT_A::SET_ON_TO,
            2 => TCACT_A::CLR_ON_TO,
            1 => TCACT_A::TOG_ON_TO,
            0 => TCACT_A::DIS_CMP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLRSET_ON_TO`"]
    #[inline(always)]
    pub fn is_clrset_on_to(&self) -> bool {
        *self == TCACT_A::CLRSET_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETCLR_ON_TO`"]
    #[inline(always)]
    pub fn is_setclr_on_to(&self) -> bool {
        *self == TCACT_A::SETCLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLRTOG_ON_TO`"]
    #[inline(always)]
    pub fn is_clrtog_on_to(&self) -> bool {
        *self == TCACT_A::CLRTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SETTOG_ON_TO`"]
    #[inline(always)]
    pub fn is_settog_on_to(&self) -> bool {
        *self == TCACT_A::SETTOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `SET_ON_TO`"]
    #[inline(always)]
    pub fn is_set_on_to(&self) -> bool {
        *self == TCACT_A::SET_ON_TO
    }
    #[doc = "Checks if the value of the field is `CLR_ON_TO`"]
    #[inline(always)]
    pub fn is_clr_on_to(&self) -> bool {
        *self == TCACT_A::CLR_ON_TO
    }
    #[doc = "Checks if the value of the field is `TOG_ON_TO`"]
    #[inline(always)]
    pub fn is_tog_on_to(&self) -> bool {
        *self == TCACT_A::TOG_ON_TO
    }
    #[doc = "Checks if the value of the field is `DIS_CMP`"]
    #[inline(always)]
    pub fn is_dis_cmp(&self) -> bool {
        *self == TCACT_A::DIS_CMP
    }
}
#[doc = "Field `TCACT` writer - 15:13\\]
Timer Compare Action Select"]
pub type TCACT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TBMR_SPEC, u8, TCACT_A, 3, O>;
impl<'a, const O: u8> TCACT_W<'a, O> {
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    #[inline(always)]
    pub fn clrset_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLRSET_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn setclr_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SETCLR_ON_TO)
    }
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn clrtog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLRTOG_ON_TO)
    }
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn settog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SETTOG_ON_TO)
    }
    #[doc = "Set CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn set_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::SET_ON_TO)
    }
    #[doc = "Clear CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn clr_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::CLR_ON_TO)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn tog_on_to(self) -> &'a mut W {
        self.variant(TCACT_A::TOG_ON_TO)
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn dis_cmp(self) -> &'a mut W {
        self.variant(TCACT_A::DIS_CMP)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBMR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcm(&self) -> TBCM_R {
        TBCM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline(always)]
    pub fn tbams(&self) -> TBAMS_R {
        TBAMS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&self) -> TBCDIR_R {
        TBCDIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer B Match Interrupt Enable."]
    #[inline(always)]
    pub fn tbmie(&self) -> TBMIE_R {
        TBMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer B Wait-On-Trigger"]
    #[inline(always)]
    pub fn tbwot(&self) -> TBWOT_R {
        TBWOT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TBSNAPS_R {
        TBSNAPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B PWM Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&self) -> TBILD_R {
        TBILD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TBPWMIE_R {
        TBPWMIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TBMRSU_R {
        TBMRSU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tbplo(&self) -> TBPLO_R {
        TBPLO_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
    #[inline(always)]
    pub fn tbcintd(&self) -> TBCINTD_R {
        TBCINTD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&self) -> TCACT_R {
        TCACT_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    #[must_use]
    pub fn tbmr(&mut self) -> TBMR_W<0> {
        TBMR_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer B Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbcm(&mut self) -> TBCM_W<2> {
        TBCM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn tbams(&mut self) -> TBAMS_W<3> {
        TBAMS_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer B Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn tbcdir(&mut self) -> TBCDIR_W<4> {
        TBCDIR_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer B Match Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tbmie(&mut self) -> TBMIE_W<5> {
        TBMIE_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer B Wait-On-Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tbwot(&mut self) -> TBWOT_W<6> {
        TBWOT_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer B Snap-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbsnaps(&mut self) -> TBSNAPS_W<7> {
        TBSNAPS_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B PWM Interval Load Write"]
    #[inline(always)]
    #[must_use]
    pub fn tbild(&mut self) -> TBILD_W<8> {
        TBILD_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tbpwmie(&mut self) -> TBPWMIE_W<9> {
        TBPWMIE_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tbmrsu(&mut self) -> TBMRSU_W<10> {
        TBMRSU_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tbplo(&mut self) -> TBPLO_W<11> {
        TBPLO_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbcintd(&mut self) -> TBCINTD_W<12> {
        TBCINTD_W::new(self)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcact(&mut self) -> TCACT_W<13> {
        TCACT_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmr](index.html) module"]
pub struct TBMR_SPEC;
impl crate::RegisterSpec for TBMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbmr::R](R) reader structure"]
impl crate::Readable for TBMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbmr::W](W) writer structure"]
impl crate::Writable for TBMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TBMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
