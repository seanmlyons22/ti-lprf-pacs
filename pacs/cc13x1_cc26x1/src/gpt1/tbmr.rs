#[doc = "Register `TBMR` reader"]
pub type R = crate::R<TbmrSpec>;
#[doc = "Register `TBMR` writer"]
pub type W = crate::W<TbmrSpec>;
#[doc = "1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tbmr {
    #[doc = "3: Capture mode"]
    Capture = 3,
    #[doc = "2: Periodic Timer mode"]
    Periodic = 2,
    #[doc = "1: One-Shot Timer mode"]
    OneShot = 1,
}
impl From<Tbmr> for u8 {
    #[inline(always)]
    fn from(variant: Tbmr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tbmr {
    type Ux = u8;
}
impl crate::IsEnum for Tbmr {}
#[doc = "Field `TBMR` reader - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TbmrR = crate::FieldReader<Tbmr>;
impl TbmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tbmr> {
        match self.bits {
            3 => Some(Tbmr::Capture),
            2 => Some(Tbmr::Periodic),
            1 => Some(Tbmr::OneShot),
            _ => None,
        }
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Tbmr::Capture
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == Tbmr::Periodic
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == Tbmr::OneShot
    }
}
#[doc = "Field `TBMR` writer - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TbmrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tbmr>;
impl<'a, REG> TbmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmr::Capture)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmr::Periodic)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmr::OneShot)
    }
}
#[doc = "2:2\\]
GPT Timer B Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbcm {
    #[doc = "1: Edge-Time mode"]
    Edgtime = 1,
    #[doc = "0: Edge-Count mode"]
    Edgcnt = 0,
}
impl From<Tbcm> for bool {
    #[inline(always)]
    fn from(variant: Tbcm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBCM` reader - 2:2\\]
GPT Timer B Capture Mode"]
pub type TbcmR = crate::BitReader<Tbcm>;
impl TbcmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbcm {
        match self.bits {
            true => Tbcm::Edgtime,
            false => Tbcm::Edgcnt,
        }
    }
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn is_edgtime(&self) -> bool {
        *self == Tbcm::Edgtime
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn is_edgcnt(&self) -> bool {
        *self == Tbcm::Edgcnt
    }
}
#[doc = "Field `TBCM` writer - 2:2\\]
GPT Timer B Capture Mode"]
pub type TbcmW<'a, REG> = crate::BitWriter<'a, REG, Tbcm>;
impl<'a, REG> TbcmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn edgtime(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcm::Edgtime)
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn edgcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcm::Edgcnt)
    }
}
#[doc = "3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbams {
    #[doc = "1: PWM mode is enabled"]
    Pwm = 1,
    #[doc = "0: Capture/Compare mode is enabled."]
    CapComp = 0,
}
impl From<Tbams> for bool {
    #[inline(always)]
    fn from(variant: Tbams) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBAMS` reader - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
pub type TbamsR = crate::BitReader<Tbams>;
impl TbamsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbams {
        match self.bits {
            true => Tbams::Pwm,
            false => Tbams::CapComp,
        }
    }
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Tbams::Pwm
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn is_cap_comp(&self) -> bool {
        *self == Tbams::CapComp
    }
}
#[doc = "Field `TBAMS` writer - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
pub type TbamsW<'a, REG> = crate::BitWriter<'a, REG, Tbams>;
impl<'a, REG> TbamsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Tbams::Pwm)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn cap_comp(self) -> &'a mut crate::W<REG> {
        self.variant(Tbams::CapComp)
    }
}
#[doc = "4:4\\]
GPT Timer B Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbcdir {
    #[doc = "1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    Up = 1,
    #[doc = "0: The timer counts down."]
    Down = 0,
}
impl From<Tbcdir> for bool {
    #[inline(always)]
    fn from(variant: Tbcdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBCDIR` reader - 4:4\\]
GPT Timer B Count Direction"]
pub type TbcdirR = crate::BitReader<Tbcdir>;
impl TbcdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbcdir {
        match self.bits {
            true => Tbcdir::Up,
            false => Tbcdir::Down,
        }
    }
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Tbcdir::Up
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Tbcdir::Down
    }
}
#[doc = "Field `TBCDIR` writer - 4:4\\]
GPT Timer B Count Direction"]
pub type TbcdirW<'a, REG> = crate::BitWriter<'a, REG, Tbcdir>;
impl<'a, REG> TbcdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcdir::Up)
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcdir::Down)
    }
}
#[doc = "5:5\\]
GPT Timer B Match Interrupt Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbmie {
    #[doc = "1: An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    En = 1,
    #[doc = "0: The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    Dis = 0,
}
impl From<Tbmie> for bool {
    #[inline(always)]
    fn from(variant: Tbmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBMIE` reader - 5:5\\]
GPT Timer B Match Interrupt Enable."]
pub type TbmieR = crate::BitReader<Tbmie>;
impl TbmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbmie {
        match self.bits {
            true => Tbmie::En,
            false => Tbmie::Dis,
        }
    }
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbmie::En
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbmie::Dis
    }
}
#[doc = "Field `TBMIE` writer - 5:5\\]
GPT Timer B Match Interrupt Enable."]
pub type TbmieW<'a, REG> = crate::BitWriter<'a, REG, Tbmie>;
impl<'a, REG> TbmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An interrupt is generated when the match value in the TBMATCHR register is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmie::En)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmie::Dis)
    }
}
#[doc = "6:6\\]
GPT Timer B Wait-On-Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbwot {
    #[doc = "1: If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    Wait = 1,
    #[doc = "0: Timer B begins counting as soon as it is enabled."]
    Nowait = 0,
}
impl From<Tbwot> for bool {
    #[inline(always)]
    fn from(variant: Tbwot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBWOT` reader - 6:6\\]
GPT Timer B Wait-On-Trigger"]
pub type TbwotR = crate::BitReader<Tbwot>;
impl TbwotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbwot {
        match self.bits {
            true => Tbwot::Wait,
            false => Tbwot::Nowait,
        }
    }
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Tbwot::Wait
    }
    #[doc = "Timer B begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == Tbwot::Nowait
    }
}
#[doc = "Field `TBWOT` writer - 6:6\\]
GPT Timer B Wait-On-Trigger"]
pub type TbwotW<'a, REG> = crate::BitWriter<'a, REG, Tbwot>;
impl<'a, REG> TbwotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If Timer B is enabled (CTL.TBEN is set), Timer B does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Tbwot::Wait)
    }
    #[doc = "Timer B begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut crate::W<REG> {
        self.variant(Tbwot::Nowait)
    }
}
#[doc = "7:7\\]
GPT Timer B Snap-Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbsnaps {
    #[doc = "1: If Timer B is configured in the periodic mode"]
    En = 1,
    #[doc = "0: Snap-shot mode is disabled."]
    Dis = 0,
}
impl From<Tbsnaps> for bool {
    #[inline(always)]
    fn from(variant: Tbsnaps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBSNAPS` reader - 7:7\\]
GPT Timer B Snap-Shot Mode"]
pub type TbsnapsR = crate::BitReader<Tbsnaps>;
impl TbsnapsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbsnaps {
        match self.bits {
            true => Tbsnaps::En,
            false => Tbsnaps::Dis,
        }
    }
    #[doc = "If Timer B is configured in the periodic mode"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbsnaps::En
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbsnaps::Dis
    }
}
#[doc = "Field `TBSNAPS` writer - 7:7\\]
GPT Timer B Snap-Shot Mode"]
pub type TbsnapsW<'a, REG> = crate::BitWriter<'a, REG, Tbsnaps>;
impl<'a, REG> TbsnapsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If Timer B is configured in the periodic mode"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbsnaps::En)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbsnaps::Dis)
    }
}
#[doc = "8:8\\]
GPT Timer B PWM Interval Load Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbild {
    #[doc = "1: Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    Toupdate = 1,
    #[doc = "0: Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    Cycleupdate = 0,
}
impl From<Tbild> for bool {
    #[inline(always)]
    fn from(variant: Tbild) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBILD` reader - 8:8\\]
GPT Timer B PWM Interval Load Write"]
pub type TbildR = crate::BitReader<Tbild>;
impl TbildR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbild {
        match self.bits {
            true => Tbild::Toupdate,
            false => Tbild::Cycleupdate,
        }
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == Tbild::Toupdate
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == Tbild::Cycleupdate
    }
}
#[doc = "Field `TBILD` writer - 8:8\\]
GPT Timer B PWM Interval Load Write"]
pub type TbildW<'a, REG> = crate::BitWriter<'a, REG, Tbild>;
impl<'a, REG> TbildW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update the TBR register with the value in the TBILR register on the next timeout. If the prescaler is used, update the TBPS register with the value in the TBPR register on the next timeout."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tbild::Toupdate)
    }
    #[doc = "Update the TBR register with the value in the TBILR register on the next clock cycle. If the pre-scaler is used, update the TBPS register with the value in the TBPR register on the next clock cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tbild::Cycleupdate)
    }
}
#[doc = "9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbpwmie {
    #[doc = "1: Interrupt is enabled. This bit is only valid in PWM mode."]
    En = 1,
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
}
impl From<Tbpwmie> for bool {
    #[inline(always)]
    fn from(variant: Tbpwmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBPWMIE` reader - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TbpwmieR = crate::BitReader<Tbpwmie>;
impl TbpwmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbpwmie {
        match self.bits {
            true => Tbpwmie::En,
            false => Tbpwmie::Dis,
        }
    }
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tbpwmie::En
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tbpwmie::Dis
    }
}
#[doc = "Field `TBPWMIE` writer - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TbpwmieW<'a, REG> = crate::BitWriter<'a, REG, Tbpwmie>;
impl<'a, REG> TbpwmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tbpwmie::En)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tbpwmie::Dis)
    }
}
#[doc = "10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbmrsu {
    #[doc = "1: Update TBMATCHR and TBPR, if used, on the next time-out."]
    Toupdate = 1,
    #[doc = "0: Update TBMATCHR and TBPR, if used, on the next cycle."]
    Cycleupdate = 0,
}
impl From<Tbmrsu> for bool {
    #[inline(always)]
    fn from(variant: Tbmrsu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBMRSU` reader - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
pub type TbmrsuR = crate::BitReader<Tbmrsu>;
impl TbmrsuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbmrsu {
        match self.bits {
            true => Tbmrsu::Toupdate,
            false => Tbmrsu::Cycleupdate,
        }
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == Tbmrsu::Toupdate
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == Tbmrsu::Cycleupdate
    }
}
#[doc = "Field `TBMRSU` writer - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
pub type TbmrsuW<'a, REG> = crate::BitWriter<'a, REG, Tbmrsu>;
impl<'a, REG> TbmrsuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update TBMATCHR and TBPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmrsu::Toupdate)
    }
    #[doc = "Update TBMATCHR and TBPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tbmrsu::Cycleupdate)
    }
}
#[doc = "11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbplo {
    #[doc = "1: CCP output pin is set to 1 on time-out"]
    CcpOnTo = 1,
    #[doc = "0: Legacy operation"]
    Legacy = 0,
}
impl From<Tbplo> for bool {
    #[inline(always)]
    fn from(variant: Tbplo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBPLO` reader - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TbploR = crate::BitReader<Tbplo>;
impl TbploR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbplo {
        match self.bits {
            true => Tbplo::CcpOnTo,
            false => Tbplo::Legacy,
        }
    }
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == Tbplo::CcpOnTo
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == Tbplo::Legacy
    }
}
#[doc = "Field `TBPLO` writer - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TbploW<'a, REG> = crate::BitWriter<'a, REG, Tbplo>;
impl<'a, REG> TbploW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn ccp_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tbplo::CcpOnTo)
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(Tbplo::Legacy)
    }
}
#[doc = "12:12\\]
One-Shot/Periodic Interrupt Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbcintd {
    #[doc = "1: Mask Time-Out Interrupt"]
    DisToIntr = 1,
    #[doc = "0: Normal Time-Out Interrupt"]
    EnToIntr = 0,
}
impl From<Tbcintd> for bool {
    #[inline(always)]
    fn from(variant: Tbcintd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBCINTD` reader - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
pub type TbcintdR = crate::BitReader<Tbcintd>;
impl TbcintdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbcintd {
        match self.bits {
            true => Tbcintd::DisToIntr,
            false => Tbcintd::EnToIntr,
        }
    }
    #[doc = "Mask Time-Out Interrupt"]
    #[inline(always)]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == Tbcintd::DisToIntr
    }
    #[doc = "Normal Time-Out Interrupt"]
    #[inline(always)]
    pub fn is_en_to_intr(&self) -> bool {
        *self == Tbcintd::EnToIntr
    }
}
#[doc = "Field `TBCINTD` writer - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
pub type TbcintdW<'a, REG> = crate::BitWriter<'a, REG, Tbcintd>;
impl<'a, REG> TbcintdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mask Time-Out Interrupt"]
    #[inline(always)]
    pub fn dis_to_intr(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcintd::DisToIntr)
    }
    #[doc = "Normal Time-Out Interrupt"]
    #[inline(always)]
    pub fn en_to_intr(self) -> &'a mut crate::W<REG> {
        self.variant(Tbcintd::EnToIntr)
    }
}
#[doc = "15:13\\]
Timer Compare Action Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcact {
    #[doc = "7: Clear CCP output pin immediately and set on Time-Out"]
    ClrsetOnTo = 7,
    #[doc = "6: Set CCP output pin immediately and clear on Time-Out"]
    SetclrOnTo = 6,
    #[doc = "5: Clear CCP output pin immediately and toggle on Time-Out"]
    ClrtogOnTo = 5,
    #[doc = "4: Set CCP output pin immediately and toggle on Time-Out"]
    SettogOnTo = 4,
    #[doc = "3: Set CCP output pin on Time-Out"]
    SetOnTo = 3,
    #[doc = "2: Clear CCP output pin on Time-Out"]
    ClrOnTo = 2,
    #[doc = "1: Toggle State on Time-Out"]
    TogOnTo = 1,
    #[doc = "0: Disable compare operations"]
    DisCmp = 0,
}
impl From<Tcact> for u8 {
    #[inline(always)]
    fn from(variant: Tcact) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcact {
    type Ux = u8;
}
impl crate::IsEnum for Tcact {}
#[doc = "Field `TCACT` reader - 15:13\\]
Timer Compare Action Select"]
pub type TcactR = crate::FieldReader<Tcact>;
impl TcactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcact {
        match self.bits {
            7 => Tcact::ClrsetOnTo,
            6 => Tcact::SetclrOnTo,
            5 => Tcact::ClrtogOnTo,
            4 => Tcact::SettogOnTo,
            3 => Tcact::SetOnTo,
            2 => Tcact::ClrOnTo,
            1 => Tcact::TogOnTo,
            0 => Tcact::DisCmp,
            _ => unreachable!(),
        }
    }
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    #[inline(always)]
    pub fn is_clrset_on_to(&self) -> bool {
        *self == Tcact::ClrsetOnTo
    }
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn is_setclr_on_to(&self) -> bool {
        *self == Tcact::SetclrOnTo
    }
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn is_clrtog_on_to(&self) -> bool {
        *self == Tcact::ClrtogOnTo
    }
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn is_settog_on_to(&self) -> bool {
        *self == Tcact::SettogOnTo
    }
    #[doc = "Set CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn is_set_on_to(&self) -> bool {
        *self == Tcact::SetOnTo
    }
    #[doc = "Clear CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn is_clr_on_to(&self) -> bool {
        *self == Tcact::ClrOnTo
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn is_tog_on_to(&self) -> bool {
        *self == Tcact::TogOnTo
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn is_dis_cmp(&self) -> bool {
        *self == Tcact::DisCmp
    }
}
#[doc = "Field `TCACT` writer - 15:13\\]
Timer Compare Action Select"]
pub type TcactW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tcact, crate::Safe>;
impl<'a, REG> TcactW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clear CCP output pin immediately and set on Time-Out"]
    #[inline(always)]
    pub fn clrset_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::ClrsetOnTo)
    }
    #[doc = "Set CCP output pin immediately and clear on Time-Out"]
    #[inline(always)]
    pub fn setclr_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::SetclrOnTo)
    }
    #[doc = "Clear CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn clrtog_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::ClrtogOnTo)
    }
    #[doc = "Set CCP output pin immediately and toggle on Time-Out"]
    #[inline(always)]
    pub fn settog_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::SettogOnTo)
    }
    #[doc = "Set CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn set_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::SetOnTo)
    }
    #[doc = "Clear CCP output pin on Time-Out"]
    #[inline(always)]
    pub fn clr_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::ClrOnTo)
    }
    #[doc = "Toggle State on Time-Out"]
    #[inline(always)]
    pub fn tog_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::TogOnTo)
    }
    #[doc = "Disable compare operations"]
    #[inline(always)]
    pub fn dis_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Tcact::DisCmp)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    pub fn tbmr(&self) -> TbmrR {
        TbmrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer B Capture Mode"]
    #[inline(always)]
    pub fn tbcm(&self) -> TbcmR {
        TbcmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline(always)]
    pub fn tbams(&self) -> TbamsR {
        TbamsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer B Count Direction"]
    #[inline(always)]
    pub fn tbcdir(&self) -> TbcdirR {
        TbcdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer B Match Interrupt Enable."]
    #[inline(always)]
    pub fn tbmie(&self) -> TbmieR {
        TbmieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer B Wait-On-Trigger"]
    #[inline(always)]
    pub fn tbwot(&self) -> TbwotR {
        TbwotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer B Snap-Shot Mode"]
    #[inline(always)]
    pub fn tbsnaps(&self) -> TbsnapsR {
        TbsnapsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B PWM Interval Load Write"]
    #[inline(always)]
    pub fn tbild(&self) -> TbildR {
        TbildR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tbpwmie(&self) -> TbpwmieR {
        TbpwmieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tbmrsu(&self) -> TbmrsuR {
        TbmrsuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tbplo(&self) -> TbploR {
        TbploR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
    #[inline(always)]
    pub fn tbcintd(&self) -> TbcintdR {
        TbcintdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    pub fn tcact(&self) -> TcactR {
        TcactR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
GPT Timer B Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    #[must_use]
    pub fn tbmr(&mut self) -> TbmrW<TbmrSpec> {
        TbmrW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer B Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbcm(&mut self) -> TbcmW<TbmrSpec> {
        TbcmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer B Alternate Mode Note: To enable PWM mode, you must also clear TBCM bit and configure TBMR field to 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn tbams(&mut self) -> TbamsW<TbmrSpec> {
        TbamsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer B Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn tbcdir(&mut self) -> TbcdirW<TbmrSpec> {
        TbcdirW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer B Match Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tbmie(&mut self) -> TbmieW<TbmrSpec> {
        TbmieW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer B Wait-On-Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tbwot(&mut self) -> TbwotW<TbmrSpec> {
        TbwotW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer B Snap-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbsnaps(&mut self) -> TbsnapsW<TbmrSpec> {
        TbsnapsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer B PWM Interval Load Write"]
    #[inline(always)]
    #[must_use]
    pub fn tbild(&mut self) -> TbildW<TbmrSpec> {
        TbildW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer B PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TBEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TBOTE bit and the DMAEV.CBEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tbpwmie(&mut self) -> TbpwmieW<TbmrSpec> {
        TbpwmieW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer B Match Register Update mode This bit defines when the TBMATCHR and TBPR registers are updated If the timer is disabled (CTL.TBEN is clear) when this bit is set, TBMATCHR and TBPR are updated when the timer is enabled. If the timer is stalled (CTL.TBSTALL is set) when this bit is set, TBMATCHR and TBPR are updated according to the configuration of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tbmrsu(&mut self) -> TbmrsuW<TbmrSpec> {
        TbmrsuW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer B PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TBILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TBILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tbplo(&mut self) -> TbploW<TbmrSpec> {
        TbploW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tbcintd(&mut self) -> TbcintdW<TbmrSpec> {
        TbcintdW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcact(&mut self) -> TcactW<TbmrSpec> {
        TcactW::new(self, 13)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TbmrSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Timer B Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbmrSpec;
impl crate::RegisterSpec for TbmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmr::R`](R) reader structure"]
impl crate::Readable for TbmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbmr::W`](W) writer structure"]
impl crate::Writable for TbmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBMR to value 0"]
impl crate::Resettable for TbmrSpec {
    const RESET_VALUE: u32 = 0;
}
