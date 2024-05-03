#[doc = "Register `TAMR` reader"]
pub type R = crate::R<TamrSpec>;
#[doc = "Register `TAMR` writer"]
pub type W = crate::W<TamrSpec>;
#[doc = "1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tamr {
    #[doc = "3: Capture mode"]
    Capture = 3,
    #[doc = "2: Periodic Timer mode"]
    Periodic = 2,
    #[doc = "1: One-Shot Timer mode"]
    OneShot = 1,
}
impl From<Tamr> for u8 {
    #[inline(always)]
    fn from(variant: Tamr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tamr {
    type Ux = u8;
}
impl crate::IsEnum for Tamr {}
#[doc = "Field `TAMR` reader - 1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TamrR = crate::FieldReader<Tamr>;
impl TamrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tamr> {
        match self.bits {
            3 => Some(Tamr::Capture),
            2 => Some(Tamr::Periodic),
            1 => Some(Tamr::OneShot),
            _ => None,
        }
    }
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        *self == Tamr::Capture
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == Tamr::Periodic
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == Tamr::OneShot
    }
}
#[doc = "Field `TAMR` writer - 1:0\\]
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
pub type TamrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tamr>;
impl<'a, REG> TamrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture mode"]
    #[inline(always)]
    pub fn capture(self) -> &'a mut crate::W<REG> {
        self.variant(Tamr::Capture)
    }
    #[doc = "Periodic Timer mode"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut crate::W<REG> {
        self.variant(Tamr::Periodic)
    }
    #[doc = "One-Shot Timer mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut crate::W<REG> {
        self.variant(Tamr::OneShot)
    }
}
#[doc = "2:2\\]
GPT Timer A Capture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tacm {
    #[doc = "1: Edge-Time mode"]
    Edgtime = 1,
    #[doc = "0: Edge-Count mode"]
    Edgcnt = 0,
}
impl From<Tacm> for bool {
    #[inline(always)]
    fn from(variant: Tacm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACM` reader - 2:2\\]
GPT Timer A Capture Mode"]
pub type TacmR = crate::BitReader<Tacm>;
impl TacmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tacm {
        match self.bits {
            true => Tacm::Edgtime,
            false => Tacm::Edgcnt,
        }
    }
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn is_edgtime(&self) -> bool {
        *self == Tacm::Edgtime
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn is_edgcnt(&self) -> bool {
        *self == Tacm::Edgcnt
    }
}
#[doc = "Field `TACM` writer - 2:2\\]
GPT Timer A Capture Mode"]
pub type TacmW<'a, REG> = crate::BitWriter<'a, REG, Tacm>;
impl<'a, REG> TacmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge-Time mode"]
    #[inline(always)]
    pub fn edgtime(self) -> &'a mut crate::W<REG> {
        self.variant(Tacm::Edgtime)
    }
    #[doc = "Edge-Count mode"]
    #[inline(always)]
    pub fn edgcnt(self) -> &'a mut crate::W<REG> {
        self.variant(Tacm::Edgcnt)
    }
}
#[doc = "3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taams {
    #[doc = "1: PWM mode is enabled"]
    Pwm = 1,
    #[doc = "0: Capture/Compare mode is enabled."]
    CapComp = 0,
}
impl From<Taams> for bool {
    #[inline(always)]
    fn from(variant: Taams) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAAMS` reader - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
pub type TaamsR = crate::BitReader<Taams>;
impl TaamsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taams {
        match self.bits {
            true => Taams::Pwm,
            false => Taams::CapComp,
        }
    }
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn is_pwm(&self) -> bool {
        *self == Taams::Pwm
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn is_cap_comp(&self) -> bool {
        *self == Taams::CapComp
    }
}
#[doc = "Field `TAAMS` writer - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
pub type TaamsW<'a, REG> = crate::BitWriter<'a, REG, Taams>;
impl<'a, REG> TaamsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM mode is enabled"]
    #[inline(always)]
    pub fn pwm(self) -> &'a mut crate::W<REG> {
        self.variant(Taams::Pwm)
    }
    #[doc = "Capture/Compare mode is enabled."]
    #[inline(always)]
    pub fn cap_comp(self) -> &'a mut crate::W<REG> {
        self.variant(Taams::CapComp)
    }
}
#[doc = "4:4\\]
GPT Timer A Count Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tacdir {
    #[doc = "1: The timer counts up. When counting up, the timer starts from a value of 0x0."]
    Up = 1,
    #[doc = "0: The timer counts down."]
    Down = 0,
}
impl From<Tacdir> for bool {
    #[inline(always)]
    fn from(variant: Tacdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACDIR` reader - 4:4\\]
GPT Timer A Count Direction"]
pub type TacdirR = crate::BitReader<Tacdir>;
impl TacdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tacdir {
        match self.bits {
            true => Tacdir::Up,
            false => Tacdir::Down,
        }
    }
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Tacdir::Up
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Tacdir::Down
    }
}
#[doc = "Field `TACDIR` writer - 4:4\\]
GPT Timer A Count Direction"]
pub type TacdirW<'a, REG> = crate::BitWriter<'a, REG, Tacdir>;
impl<'a, REG> TacdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The timer counts up. When counting up, the timer starts from a value of 0x0."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Tacdir::Up)
    }
    #[doc = "The timer counts down."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Tacdir::Down)
    }
}
#[doc = "5:5\\]
GPT Timer A Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamie {
    #[doc = "1: An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    En = 1,
    #[doc = "0: The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    Dis = 0,
}
impl From<Tamie> for bool {
    #[inline(always)]
    fn from(variant: Tamie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMIE` reader - 5:5\\]
GPT Timer A Match Interrupt Enable"]
pub type TamieR = crate::BitReader<Tamie>;
impl TamieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamie {
        match self.bits {
            true => Tamie::En,
            false => Tamie::Dis,
        }
    }
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tamie::En
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tamie::Dis
    }
}
#[doc = "Field `TAMIE` writer - 5:5\\]
GPT Timer A Match Interrupt Enable"]
pub type TamieW<'a, REG> = crate::BitWriter<'a, REG, Tamie>;
impl<'a, REG> TamieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An interrupt is generated when the match value in TAMATCHR is reached in the one-shot and periodic modes."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tamie::En)
    }
    #[doc = "The match interrupt is disabled for match events. Additionally, output triggers on match events are prevented."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tamie::Dis)
    }
}
#[doc = "6:6\\]
GPT Timer A Wait-On-Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tawot {
    #[doc = "1: If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    Wait = 1,
    #[doc = "0: Timer A begins counting as soon as it is enabled."]
    Nowait = 0,
}
impl From<Tawot> for bool {
    #[inline(always)]
    fn from(variant: Tawot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAWOT` reader - 6:6\\]
GPT Timer A Wait-On-Trigger"]
pub type TawotR = crate::BitReader<Tawot>;
impl TawotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tawot {
        match self.bits {
            true => Tawot::Wait,
            false => Tawot::Nowait,
        }
    }
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        *self == Tawot::Wait
    }
    #[doc = "Timer A begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn is_nowait(&self) -> bool {
        *self == Tawot::Nowait
    }
}
#[doc = "Field `TAWOT` writer - 6:6\\]
GPT Timer A Wait-On-Trigger"]
pub type TawotW<'a, REG> = crate::BitWriter<'a, REG, Tawot>;
impl<'a, REG> TawotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If Timer A is enabled (CTL.TAEN = 1), Timer A does not begin counting until it receives a trigger from the timer in the previous position in the daisy chain. This bit must be clear for GPT Module 0, Timer A. This function is valid for one-shot, periodic, and PWM modes"]
    #[inline(always)]
    pub fn wait(self) -> &'a mut crate::W<REG> {
        self.variant(Tawot::Wait)
    }
    #[doc = "Timer A begins counting as soon as it is enabled."]
    #[inline(always)]
    pub fn nowait(self) -> &'a mut crate::W<REG> {
        self.variant(Tawot::Nowait)
    }
}
#[doc = "7:7\\]
GPT Timer A Snap-Shot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tasnaps {
    #[doc = "1: If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    En = 1,
    #[doc = "0: Snap-shot mode is disabled."]
    Dis = 0,
}
impl From<Tasnaps> for bool {
    #[inline(always)]
    fn from(variant: Tasnaps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASNAPS` reader - 7:7\\]
GPT Timer A Snap-Shot Mode"]
pub type TasnapsR = crate::BitReader<Tasnaps>;
impl TasnapsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tasnaps {
        match self.bits {
            true => Tasnaps::En,
            false => Tasnaps::Dis,
        }
    }
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tasnaps::En
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tasnaps::Dis
    }
}
#[doc = "Field `TASNAPS` writer - 7:7\\]
GPT Timer A Snap-Shot Mode"]
pub type TasnapsW<'a, REG> = crate::BitWriter<'a, REG, Tasnaps>;
impl<'a, REG> TasnapsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If Timer A is configured in the periodic mode, the actual free-running value of Timer A is loaded at the time-out event into the GPT Timer A (TAR) register."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tasnaps::En)
    }
    #[doc = "Snap-shot mode is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tasnaps::Dis)
    }
}
#[doc = "8:8\\]
GPT Timer A PWM Interval Load Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taild {
    #[doc = "1: Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    Toupdate = 1,
    #[doc = "0: Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    Cycleupdate = 0,
}
impl From<Taild> for bool {
    #[inline(always)]
    fn from(variant: Taild) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAILD` reader - 8:8\\]
GPT Timer A PWM Interval Load Write"]
pub type TaildR = crate::BitReader<Taild>;
impl TaildR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taild {
        match self.bits {
            true => Taild::Toupdate,
            false => Taild::Cycleupdate,
        }
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == Taild::Toupdate
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == Taild::Cycleupdate
    }
}
#[doc = "Field `TAILD` writer - 8:8\\]
GPT Timer A PWM Interval Load Write"]
pub type TaildW<'a, REG> = crate::BitWriter<'a, REG, Taild>;
impl<'a, REG> TaildW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update the TAR register with the value in the TAILR register on the next timeout. If the prescaler is used, update the TAPS register with the value in the TAPR register on the next timeout."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Taild::Toupdate)
    }
    #[doc = "Update the TAR register with the value in the TAILR register on the next clock cycle. If the pre-scaler is used, update the TAPS register with the value in the TAPR register on the next clock cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Taild::Cycleupdate)
    }
}
#[doc = "9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tapwmie {
    #[doc = "1: Interrupt is enabled. This bit is only valid in PWM mode."]
    En = 1,
    #[doc = "0: Interrupt is disabled."]
    Dis = 0,
}
impl From<Tapwmie> for bool {
    #[inline(always)]
    fn from(variant: Tapwmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPWMIE` reader - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TapwmieR = crate::BitReader<Tapwmie>;
impl TapwmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tapwmie {
        match self.bits {
            true => Tapwmie::En,
            false => Tapwmie::Dis,
        }
    }
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tapwmie::En
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Tapwmie::Dis
    }
}
#[doc = "Field `TAPWMIE` writer - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
pub type TapwmieW<'a, REG> = crate::BitWriter<'a, REG, Tapwmie>;
impl<'a, REG> TapwmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tapwmie::En)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Tapwmie::Dis)
    }
}
#[doc = "10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tamrsu {
    #[doc = "1: Update TAMATCHR and TAPR, if used, on the next time-out."]
    Toupdate = 1,
    #[doc = "0: Update TAMATCHR and TAPR, if used, on the next cycle."]
    Cycleupdate = 0,
}
impl From<Tamrsu> for bool {
    #[inline(always)]
    fn from(variant: Tamrsu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMRSU` reader - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
pub type TamrsuR = crate::BitReader<Tamrsu>;
impl TamrsuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tamrsu {
        match self.bits {
            true => Tamrsu::Toupdate,
            false => Tamrsu::Cycleupdate,
        }
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn is_toupdate(&self) -> bool {
        *self == Tamrsu::Toupdate
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn is_cycleupdate(&self) -> bool {
        *self == Tamrsu::Cycleupdate
    }
}
#[doc = "Field `TAMRSU` writer - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
pub type TamrsuW<'a, REG> = crate::BitWriter<'a, REG, Tamrsu>;
impl<'a, REG> TamrsuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update TAMATCHR and TAPR, if used, on the next time-out."]
    #[inline(always)]
    pub fn toupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tamrsu::Toupdate)
    }
    #[doc = "Update TAMATCHR and TAPR, if used, on the next cycle."]
    #[inline(always)]
    pub fn cycleupdate(self) -> &'a mut crate::W<REG> {
        self.variant(Tamrsu::Cycleupdate)
    }
}
#[doc = "11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taplo {
    #[doc = "1: CCP output pin is set to 1 on time-out"]
    CcpOnTo = 1,
    #[doc = "0: Legacy operation"]
    Legacy = 0,
}
impl From<Taplo> for bool {
    #[inline(always)]
    fn from(variant: Taplo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAPLO` reader - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TaploR = crate::BitReader<Taplo>;
impl TaploR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taplo {
        match self.bits {
            true => Taplo::CcpOnTo,
            false => Taplo::Legacy,
        }
    }
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn is_ccp_on_to(&self) -> bool {
        *self == Taplo::CcpOnTo
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == Taplo::Legacy
    }
}
#[doc = "Field `TAPLO` writer - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
pub type TaploW<'a, REG> = crate::BitWriter<'a, REG, Taplo>;
impl<'a, REG> TaploW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCP output pin is set to 1 on time-out"]
    #[inline(always)]
    pub fn ccp_on_to(self) -> &'a mut crate::W<REG> {
        self.variant(Taplo::CcpOnTo)
    }
    #[doc = "Legacy operation"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(Taplo::Legacy)
    }
}
#[doc = "12:12\\]
One-Shot/Periodic Interrupt Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tacintd {
    #[doc = "1: Time-out interrupt are disabled"]
    DisToIntr = 1,
    #[doc = "0: Time-out interrupt function as normal"]
    EnToIntr = 0,
}
impl From<Tacintd> for bool {
    #[inline(always)]
    fn from(variant: Tacintd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TACINTD` reader - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
pub type TacintdR = crate::BitReader<Tacintd>;
impl TacintdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tacintd {
        match self.bits {
            true => Tacintd::DisToIntr,
            false => Tacintd::EnToIntr,
        }
    }
    #[doc = "Time-out interrupt are disabled"]
    #[inline(always)]
    pub fn is_dis_to_intr(&self) -> bool {
        *self == Tacintd::DisToIntr
    }
    #[doc = "Time-out interrupt function as normal"]
    #[inline(always)]
    pub fn is_en_to_intr(&self) -> bool {
        *self == Tacintd::EnToIntr
    }
}
#[doc = "Field `TACINTD` writer - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
pub type TacintdW<'a, REG> = crate::BitWriter<'a, REG, Tacintd>;
impl<'a, REG> TacintdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time-out interrupt are disabled"]
    #[inline(always)]
    pub fn dis_to_intr(self) -> &'a mut crate::W<REG> {
        self.variant(Tacintd::DisToIntr)
    }
    #[doc = "Time-out interrupt function as normal"]
    #[inline(always)]
    pub fn en_to_intr(self) -> &'a mut crate::W<REG> {
        self.variant(Tacintd::EnToIntr)
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
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    pub fn tamr(&self) -> TamrR {
        TamrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode"]
    #[inline(always)]
    pub fn tacm(&self) -> TacmR {
        TacmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline(always)]
    pub fn taams(&self) -> TaamsR {
        TaamsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Count Direction"]
    #[inline(always)]
    pub fn tacdir(&self) -> TacdirR {
        TacdirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A Match Interrupt Enable"]
    #[inline(always)]
    pub fn tamie(&self) -> TamieR {
        TamieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A Wait-On-Trigger"]
    #[inline(always)]
    pub fn tawot(&self) -> TawotR {
        TawotR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer A Snap-Shot Mode"]
    #[inline(always)]
    pub fn tasnaps(&self) -> TasnapsR {
        TasnapsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer A PWM Interval Load Write"]
    #[inline(always)]
    pub fn taild(&self) -> TaildR {
        TaildR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn tapwmie(&self) -> TapwmieR {
        TapwmieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    pub fn tamrsu(&self) -> TamrsuR {
        TamrsuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    pub fn taplo(&self) -> TaploR {
        TaploR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    pub fn tacintd(&self) -> TacintdR {
        TacintdR::new(((self.bits >> 12) & 1) != 0)
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
GPT Timer A Mode 0x0 Reserved 0x1 One-Shot Timer mode 0x2 Periodic Timer mode 0x3 Capture mode The Timer mode is based on the timer configuration defined by bits 2:0 in the CFG register"]
    #[inline(always)]
    #[must_use]
    pub fn tamr(&mut self) -> TamrW<TamrSpec> {
        TamrW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
GPT Timer A Capture Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tacm(&mut self) -> TacmW<TamrSpec> {
        TacmW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
GPT Timer A Alternate Mode Note: To enable PWM mode, you must also clear TACM and then configure TAMR field to 0x2."]
    #[inline(always)]
    #[must_use]
    pub fn taams(&mut self) -> TaamsW<TamrSpec> {
        TaamsW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
GPT Timer A Count Direction"]
    #[inline(always)]
    #[must_use]
    pub fn tacdir(&mut self) -> TacdirW<TamrSpec> {
        TacdirW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
GPT Timer A Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamie(&mut self) -> TamieW<TamrSpec> {
        TamieW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
GPT Timer A Wait-On-Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn tawot(&mut self) -> TawotW<TamrSpec> {
        TawotW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
GPT Timer A Snap-Shot Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tasnaps(&mut self) -> TasnapsW<TamrSpec> {
        TasnapsW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
GPT Timer A PWM Interval Load Write"]
    #[inline(always)]
    #[must_use]
    pub fn taild(&mut self) -> TaildW<TamrSpec> {
        TaildW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
GPTM Timer A PWM Interrupt Enable This bit enables interrupts in PWM mode on rising, falling, or both edges of the CCP output, as defined by the CTL.TAEVENT In addition, when this bit is set and a capture event occurs, Timer A automatically generates triggers to the DMA if the trigger capability is enabled by setting the CTL.TAOTE bit and the DMAEV.CAEDMAEN bit respectively. 0 Capture event interrupt is disabled. 1 Capture event interrupt is enabled. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn tapwmie(&mut self) -> TapwmieW<TamrSpec> {
        TapwmieW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Timer A Match Register Update mode This bit defines when the TAMATCHR and TAPR registers are updated. If the timer is disabled (CTL.TAEN = 0) when this bit is set, TAMATCHR and TAPR are updated when the timer is enabled. If the timer is stalled (CTL.TASTALL = 1) when this bit is set, TAMATCHR and TAPR are updated according to the configuration of this bit."]
    #[inline(always)]
    #[must_use]
    pub fn tamrsu(&mut self) -> TamrsuW<TamrSpec> {
        TamrsuW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
GPTM Timer A PWM Legacy Operation 0 Legacy operation with CCP pin driven Low when the TAILR register is reloaded after the timer reaches 0. 1 CCP is driven High when the TAILR register is reloaded after the timer reaches 0. This bit is only valid in PWM mode."]
    #[inline(always)]
    #[must_use]
    pub fn taplo(&mut self) -> TaploW<TamrSpec> {
        TaploW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
One-Shot/Periodic Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tacintd(&mut self) -> TacintdW<TamrSpec> {
        TacintdW::new(self, 12)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Timer Compare Action Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcact(&mut self) -> TcactW<TamrSpec> {
        TcactW::new(self, 13)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TamrSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Timer A Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TamrSpec;
impl crate::RegisterSpec for TamrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tamr::R`](R) reader structure"]
impl crate::Readable for TamrSpec {}
#[doc = "`write(|w| ..)` method takes [`tamr::W`](W) writer structure"]
impl crate::Writable for TamrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAMR to value 0"]
impl crate::Resettable for TamrSpec {
    const RESET_VALUE: u32 = 0;
}
