#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "2:0\\]
Timer mode control The CNTR restarts from 0 when MODE is written to UP_ONCE, UP_PER, UPDWN_PER, QDEC, SYNC_UP_ONCE, SYNC_UP_PER or SYNC_UPDWN_PER. When writing MODE all internally queued updates to the channels and TGT is cleared. When configuring the timer, MODE should be the last thing to configure. If changing timer configuration after MODE has been set is necessary, instructions, if any, given in the configuration registers should be followed. See for example C0CFG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "7: Start counting up and down periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UPDWN_PER automatically. It then operates as a normal timer in CTL.MODE = UPDWN_PER, counting from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    SyncUpdwnPer = 7,
    #[doc = "6: Start counting up periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_PER automatically. It then operates as a normal timer in CTL.MODE = UP_PER, incrementing from 0 to target value, repeatedly. Period = (target value * 2) * timer clock period"]
    SyncUpPer = 6,
    #[doc = "5: Start counting up once synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_ONCE automatically. It then functions as a normal timer in CTL.MODE = UP_ONCE, incrementing from 0 to target value, then stops and sets MODE to DIS."]
    SyncUpOnce = 5,
    #[doc = "4: The timer functions as a quadrature decoder. IOC input 0, IOC input 1 and IOC input 2 are used respectivly as PHA, PHB and IDX inputs. IDX can be turned off by setting C2CFG.EDGE = NONE. The timer clock frequency sets the sample rate of the QDEC logic. This frequency can be configured in PRECFG."]
    Qdec = 4,
    #[doc = "3: Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    UpdwnPer = 3,
    #[doc = "2: Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    UpPer = 2,
    #[doc = "1: Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    UpOnce = 1,
    #[doc = "0: Disable timer. Updates to counter, channels, and events stop."]
    Dis = 0,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - 2:0\\]
Timer mode control The CNTR restarts from 0 when MODE is written to UP_ONCE, UP_PER, UPDWN_PER, QDEC, SYNC_UP_ONCE, SYNC_UP_PER or SYNC_UPDWN_PER. When writing MODE all internally queued updates to the channels and TGT is cleared. When configuring the timer, MODE should be the last thing to configure. If changing timer configuration after MODE has been set is necessary, instructions, if any, given in the configuration registers should be followed. See for example C0CFG."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            7 => Mode::SyncUpdwnPer,
            6 => Mode::SyncUpPer,
            5 => Mode::SyncUpOnce,
            4 => Mode::Qdec,
            3 => Mode::UpdwnPer,
            2 => Mode::UpPer,
            1 => Mode::UpOnce,
            0 => Mode::Dis,
            _ => unreachable!(),
        }
    }
    #[doc = "Start counting up and down periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UPDWN_PER automatically. It then operates as a normal timer in CTL.MODE = UPDWN_PER, counting from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn is_sync_updwn_per(&self) -> bool {
        *self == Mode::SyncUpdwnPer
    }
    #[doc = "Start counting up periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_PER automatically. It then operates as a normal timer in CTL.MODE = UP_PER, incrementing from 0 to target value, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn is_sync_up_per(&self) -> bool {
        *self == Mode::SyncUpPer
    }
    #[doc = "Start counting up once synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_ONCE automatically. It then functions as a normal timer in CTL.MODE = UP_ONCE, incrementing from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn is_sync_up_once(&self) -> bool {
        *self == Mode::SyncUpOnce
    }
    #[doc = "The timer functions as a quadrature decoder. IOC input 0, IOC input 1 and IOC input 2 are used respectivly as PHA, PHB and IDX inputs. IDX can be turned off by setting C2CFG.EDGE = NONE. The timer clock frequency sets the sample rate of the QDEC logic. This frequency can be configured in PRECFG."]
    #[inline(always)]
    pub fn is_qdec(&self) -> bool {
        *self == Mode::Qdec
    }
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn is_updwn_per(&self) -> bool {
        *self == Mode::UpdwnPer
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn is_up_per(&self) -> bool {
        *self == Mode::UpPer
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn is_up_once(&self) -> bool {
        *self == Mode::UpOnce
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Mode::Dis
    }
}
#[doc = "Field `MODE` writer - 2:0\\]
Timer mode control The CNTR restarts from 0 when MODE is written to UP_ONCE, UP_PER, UPDWN_PER, QDEC, SYNC_UP_ONCE, SYNC_UP_PER or SYNC_UPDWN_PER. When writing MODE all internally queued updates to the channels and TGT is cleared. When configuring the timer, MODE should be the last thing to configure. If changing timer configuration after MODE has been set is necessary, instructions, if any, given in the configuration registers should be followed. See for example C0CFG."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start counting up and down periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UPDWN_PER automatically. It then operates as a normal timer in CTL.MODE = UPDWN_PER, counting from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn sync_updwn_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SyncUpdwnPer)
    }
    #[doc = "Start counting up periodically synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_PER automatically. It then operates as a normal timer in CTL.MODE = UP_PER, incrementing from 0 to target value, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn sync_up_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SyncUpPer)
    }
    #[doc = "Start counting up once synchronous to another LGPT, selected within STARTCFG. The timer is started by setting CTL.MODE = UP_ONCE automatically. It then functions as a normal timer in CTL.MODE = UP_ONCE, incrementing from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn sync_up_once(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::SyncUpOnce)
    }
    #[doc = "The timer functions as a quadrature decoder. IOC input 0, IOC input 1 and IOC input 2 are used respectivly as PHA, PHB and IDX inputs. IDX can be turned off by setting C2CFG.EDGE = NONE. The timer clock frequency sets the sample rate of the QDEC logic. This frequency can be configured in PRECFG."]
    #[inline(always)]
    pub fn qdec(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Qdec)
    }
    #[doc = "Count up and down periodically. The timer counts from 0 to target value and back to 0, repeatedly. Period = (target value * 2) * timer clock period"]
    #[inline(always)]
    pub fn updwn_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpdwnPer)
    }
    #[doc = "Count up periodically. The timer increments from 0 to target value, repeatedly. Period = (target value + 1) * timer clock period"]
    #[inline(always)]
    pub fn up_per(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpPer)
    }
    #[doc = "Count up once. The timer increments from 0 to target value, then stops and sets MODE to DIS."]
    #[inline(always)]
    pub fn up_once(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::UpOnce)
    }
    #[doc = "Disable timer. Updates to counter, channels, and events stop."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Dis)
    }
}
#[doc = "4:3\\]
Compare direction. This bit field controls the direction the counter must have in order to set the RIS compare interrupts.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmpdir {
    #[doc = "2: Compare RIS fields are only set on down count."]
    Down = 2,
    #[doc = "1: Compare RIS fields are only set on up count."]
    Up = 1,
    #[doc = "0: Compare RIS fields are set on up count and down count."]
    Both = 0,
}
impl From<Cmpdir> for u8 {
    #[inline(always)]
    fn from(variant: Cmpdir) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmpdir {
    type Ux = u8;
}
impl crate::IsEnum for Cmpdir {}
#[doc = "Field `CMPDIR` reader - 4:3\\]
Compare direction. This bit field controls the direction the counter must have in order to set the RIS compare interrupts."]
pub type CmpdirR = crate::FieldReader<Cmpdir>;
impl CmpdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmpdir {
        match self.bits {
            2 => Cmpdir::Down,
            1 => Cmpdir::Up,
            0 => Cmpdir::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "Compare RIS fields are only set on down count."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cmpdir::Down
    }
    #[doc = "Compare RIS fields are only set on up count."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cmpdir::Up
    }
    #[doc = "Compare RIS fields are set on up count and down count."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cmpdir::Both
    }
}
#[doc = "Field `CMPDIR` writer - 4:3\\]
Compare direction. This bit field controls the direction the counter must have in order to set the RIS compare interrupts."]
pub type CmpdirW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmpdir>;
impl<'a, REG> CmpdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Compare RIS fields are only set on down count."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpdir::Down)
    }
    #[doc = "Compare RIS fields are only set on up count."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpdir::Up)
    }
    #[doc = "Compare RIS fields are set on up count and down count."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cmpdir::Both)
    }
}
#[doc = "5:5\\]
Interrupt Phase. This bit field controls when the RIS.TGT and RIS.ZERO interrupts are set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intp {
    #[doc = "1: RIS.TGT and RIS.ZERO are set one timer clock cycle after CNTR = TARGET/ZERO."]
    Late = 1,
    #[doc = "0: RIS.TGT and RIS.ZERO are set one system clock cycle after CNTR = TARGET/ZERO."]
    Early = 0,
}
impl From<Intp> for bool {
    #[inline(always)]
    fn from(variant: Intp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTP` reader - 5:5\\]
Interrupt Phase. This bit field controls when the RIS.TGT and RIS.ZERO interrupts are set."]
pub type IntpR = crate::BitReader<Intp>;
impl IntpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intp {
        match self.bits {
            true => Intp::Late,
            false => Intp::Early,
        }
    }
    #[doc = "RIS.TGT and RIS.ZERO are set one timer clock cycle after CNTR = TARGET/ZERO."]
    #[inline(always)]
    pub fn is_late(&self) -> bool {
        *self == Intp::Late
    }
    #[doc = "RIS.TGT and RIS.ZERO are set one system clock cycle after CNTR = TARGET/ZERO."]
    #[inline(always)]
    pub fn is_early(&self) -> bool {
        *self == Intp::Early
    }
}
#[doc = "Field `INTP` writer - 5:5\\]
Interrupt Phase. This bit field controls when the RIS.TGT and RIS.ZERO interrupts are set."]
pub type IntpW<'a, REG> = crate::BitWriter<'a, REG, Intp>;
impl<'a, REG> IntpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RIS.TGT and RIS.ZERO are set one timer clock cycle after CNTR = TARGET/ZERO."]
    #[inline(always)]
    pub fn late(self) -> &'a mut crate::W<REG> {
        self.variant(Intp::Late)
    }
    #[doc = "RIS.TGT and RIS.ZERO are set one system clock cycle after CNTR = TARGET/ZERO."]
    #[inline(always)]
    pub fn early(self) -> &'a mut crate::W<REG> {
        self.variant(Intp::Early)
    }
}
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "8:8\\]
Channel 0 reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C0rst {
    #[doc = "1: Reset C0CC, PC0CC, and C0CFG."]
    Rst = 1,
    #[doc = "0: No effect."]
    Noeff = 0,
}
impl From<C0rst> for bool {
    #[inline(always)]
    fn from(variant: C0rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C0RST` writer - 8:8\\]
Channel 0 reset."]
pub type C0rstW<'a, REG> = crate::BitWriter<'a, REG, C0rst>;
impl<'a, REG> C0rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset C0CC, PC0CC, and C0CFG."]
    #[inline(always)]
    pub fn rst(self) -> &'a mut crate::W<REG> {
        self.variant(C0rst::Rst)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(C0rst::Noeff)
    }
}
#[doc = "9:9\\]
Channel 1 reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C1rst {
    #[doc = "1: Reset C1CC, PC1CC, and C1CFG."]
    Rst = 1,
    #[doc = "0: No effect."]
    Noeff = 0,
}
impl From<C1rst> for bool {
    #[inline(always)]
    fn from(variant: C1rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C1RST` writer - 9:9\\]
Channel 1 reset."]
pub type C1rstW<'a, REG> = crate::BitWriter<'a, REG, C1rst>;
impl<'a, REG> C1rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset C1CC, PC1CC, and C1CFG."]
    #[inline(always)]
    pub fn rst(self) -> &'a mut crate::W<REG> {
        self.variant(C1rst::Rst)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(C1rst::Noeff)
    }
}
#[doc = "10:10\\]
Channel 2 reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum C2rst {
    #[doc = "1: Reset C2CC, PC2CC, and C2CFG."]
    Rst = 1,
    #[doc = "0: No effect."]
    Noeff = 0,
}
impl From<C2rst> for bool {
    #[inline(always)]
    fn from(variant: C2rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `C2RST` writer - 10:10\\]
Channel 2 reset."]
pub type C2rstW<'a, REG> = crate::BitWriter<'a, REG, C2rst>;
impl<'a, REG> C2rstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset C2CC, PC2CC, and C2CFG."]
    #[inline(always)]
    pub fn rst(self) -> &'a mut crate::W<REG> {
        self.variant(C2rst::Rst)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(C2rst::Noeff)
    }
}
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved11R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Timer mode control The CNTR restarts from 0 when MODE is written to UP_ONCE, UP_PER, UPDWN_PER, QDEC, SYNC_UP_ONCE, SYNC_UP_PER or SYNC_UPDWN_PER. When writing MODE all internally queued updates to the channels and TGT is cleared. When configuring the timer, MODE should be the last thing to configure. If changing timer configuration after MODE has been set is necessary, instructions, if any, given in the configuration registers should be followed. See for example C0CFG."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Compare direction. This bit field controls the direction the counter must have in order to set the RIS compare interrupts."]
    #[inline(always)]
    pub fn cmpdir(&self) -> CmpdirR {
        CmpdirR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Phase. This bit field controls when the RIS.TGT and RIS.ZERO interrupts are set."]
    #[inline(always)]
    pub fn intp(&self) -> IntpR {
        IntpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Timer mode control The CNTR restarts from 0 when MODE is written to UP_ONCE, UP_PER, UPDWN_PER, QDEC, SYNC_UP_ONCE, SYNC_UP_PER or SYNC_UPDWN_PER. When writing MODE all internally queued updates to the channels and TGT is cleared. When configuring the timer, MODE should be the last thing to configure. If changing timer configuration after MODE has been set is necessary, instructions, if any, given in the configuration registers should be followed. See for example C0CFG."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtlSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Compare direction. This bit field controls the direction the counter must have in order to set the RIS compare interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn cmpdir(&mut self) -> CmpdirW<CtlSpec> {
        CmpdirW::new(self, 3)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt Phase. This bit field controls when the RIS.TGT and RIS.ZERO interrupts are set."]
    #[inline(always)]
    #[must_use]
    pub fn intp(&mut self) -> IntpW<CtlSpec> {
        IntpW::new(self, 5)
    }
    #[doc = "Bit 8 - 8:8\\]
Channel 0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn c0rst(&mut self) -> C0rstW<CtlSpec> {
        C0rstW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Channel 1 reset."]
    #[inline(always)]
    #[must_use]
    pub fn c1rst(&mut self) -> C1rstW<CtlSpec> {
        C1rstW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Channel 2 reset."]
    #[inline(always)]
    #[must_use]
    pub fn c2rst(&mut self) -> C2rstW<CtlSpec> {
        C2rstW::new(self, 10)
    }
}
#[doc = "Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
