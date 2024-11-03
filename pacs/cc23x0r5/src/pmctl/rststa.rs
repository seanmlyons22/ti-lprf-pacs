#[doc = "Register `RSTSTA` reader"]
pub type R = crate::R<RststaSpec>;
#[doc = "Register `RSTSTA` writer"]
pub type W = crate::W<RststaSpec>;
#[doc = "2:0\\]
Shows the root cause of the last system reset. More than one reported reset source can have been active during the last system reset, but only the root cause is reported. If reset cause is SYSRESET or PINRESET, the other reset flags must be read to determine actual root cause.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resetsrc {
    #[doc = "6: Digital system reset. Actual root cause is given by SYSSRC."]
    Sysreset = 6,
    #[doc = "4: Brown out detect on VDDR"]
    Vddrloss = 4,
    #[doc = "2: Brown out detect on VDDS"]
    Vddsloss = 2,
    #[doc = "1: Reset pin. TSD will also trigger a pin reset, so actual root cause is given by TSDEV reset flag status."]
    Pinreset = 1,
    #[doc = "0: Power on reset"]
    Pwron = 0,
}
impl From<Resetsrc> for u8 {
    #[inline(always)]
    fn from(variant: Resetsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resetsrc {
    type Ux = u8;
}
impl crate::IsEnum for Resetsrc {}
#[doc = "Field `RESETSRC` reader - 2:0\\]
Shows the root cause of the last system reset. More than one reported reset source can have been active during the last system reset, but only the root cause is reported. If reset cause is SYSRESET or PINRESET, the other reset flags must be read to determine actual root cause."]
pub type ResetsrcR = crate::FieldReader<Resetsrc>;
impl ResetsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Resetsrc> {
        match self.bits {
            6 => Some(Resetsrc::Sysreset),
            4 => Some(Resetsrc::Vddrloss),
            2 => Some(Resetsrc::Vddsloss),
            1 => Some(Resetsrc::Pinreset),
            0 => Some(Resetsrc::Pwron),
            _ => None,
        }
    }
    #[doc = "Digital system reset. Actual root cause is given by SYSSRC."]
    #[inline(always)]
    pub fn is_sysreset(&self) -> bool {
        *self == Resetsrc::Sysreset
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn is_vddrloss(&self) -> bool {
        *self == Resetsrc::Vddrloss
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn is_vddsloss(&self) -> bool {
        *self == Resetsrc::Vddsloss
    }
    #[doc = "Reset pin. TSD will also trigger a pin reset, so actual root cause is given by TSDEV reset flag status."]
    #[inline(always)]
    pub fn is_pinreset(&self) -> bool {
        *self == Resetsrc::Pinreset
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn is_pwron(&self) -> bool {
        *self == Resetsrc::Pwron
    }
}
#[doc = "Field `RESETSRC` writer - 2:0\\]
Shows the root cause of the last system reset. More than one reported reset source can have been active during the last system reset, but only the root cause is reported. If reset cause is SYSRESET or PINRESET, the other reset flags must be read to determine actual root cause."]
pub type ResetsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Resetsrc>;
impl<'a, REG> ResetsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital system reset. Actual root cause is given by SYSSRC."]
    #[inline(always)]
    pub fn sysreset(self) -> &'a mut crate::W<REG> {
        self.variant(Resetsrc::Sysreset)
    }
    #[doc = "Brown out detect on VDDR"]
    #[inline(always)]
    pub fn vddrloss(self) -> &'a mut crate::W<REG> {
        self.variant(Resetsrc::Vddrloss)
    }
    #[doc = "Brown out detect on VDDS"]
    #[inline(always)]
    pub fn vddsloss(self) -> &'a mut crate::W<REG> {
        self.variant(Resetsrc::Vddsloss)
    }
    #[doc = "Reset pin. TSD will also trigger a pin reset, so actual root cause is given by TSDEV reset flag status."]
    #[inline(always)]
    pub fn pinreset(self) -> &'a mut crate::W<REG> {
        self.variant(Resetsrc::Pinreset)
    }
    #[doc = "Power on reset"]
    #[inline(always)]
    pub fn pwron(self) -> &'a mut crate::W<REG> {
        self.variant(Resetsrc::Pwron)
    }
}
#[doc = "3:3\\]
System reset triggered by TSD event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsdev {
    #[doc = "1: System reset triggered by TSD event"]
    Trig = 1,
    #[doc = "0: TSD event not triggered"]
    NoTrig = 0,
}
impl From<Tsdev> for bool {
    #[inline(always)]
    fn from(variant: Tsdev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSDEV` reader - 3:3\\]
System reset triggered by TSD event"]
pub type TsdevR = crate::BitReader<Tsdev>;
impl TsdevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsdev {
        match self.bits {
            true => Tsdev::Trig,
            false => Tsdev::NoTrig,
        }
    }
    #[doc = "System reset triggered by TSD event"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Tsdev::Trig
    }
    #[doc = "TSD event not triggered"]
    #[inline(always)]
    pub fn is_no_trig(&self) -> bool {
        *self == Tsdev::NoTrig
    }
}
#[doc = "Field `TSDEV` writer - 3:3\\]
System reset triggered by TSD event"]
pub type TsdevW<'a, REG> = crate::BitWriter<'a, REG, Tsdev>;
impl<'a, REG> TsdevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "System reset triggered by TSD event"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Tsdev::Trig)
    }
    #[doc = "TSD event not triggered"]
    #[inline(always)]
    pub fn no_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Tsdev::NoTrig)
    }
}
#[doc = "7:4\\]
Shows which reset event that triggered SYSRESET in RESETSRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syssrc {
    #[doc = "15: Digital Error reset event"]
    Derrev = 15,
    #[doc = "14: Analog Error reset event"]
    Aerrev = 14,
    #[doc = "6: Analog FSM timeout event"]
    Afsmev = 6,
    #[doc = "5: Serial Wire Debug reset event"]
    Swdrstev = 5,
    #[doc = "4: System reset event"]
    Sysrstev = 4,
    #[doc = "3: Watchdog timeout event"]
    Wdtev = 3,
    #[doc = "2: CPU LOCKUP event"]
    Lockupev = 2,
    #[doc = "1: CPU reset event"]
    Cpurstev = 1,
    #[doc = "0: LF clock loss event"]
    Lflossev = 0,
}
impl From<Syssrc> for u8 {
    #[inline(always)]
    fn from(variant: Syssrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syssrc {
    type Ux = u8;
}
impl crate::IsEnum for Syssrc {}
#[doc = "Field `SYSSRC` reader - 7:4\\]
Shows which reset event that triggered SYSRESET in RESETSRC"]
pub type SyssrcR = crate::FieldReader<Syssrc>;
impl SyssrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syssrc> {
        match self.bits {
            15 => Some(Syssrc::Derrev),
            14 => Some(Syssrc::Aerrev),
            6 => Some(Syssrc::Afsmev),
            5 => Some(Syssrc::Swdrstev),
            4 => Some(Syssrc::Sysrstev),
            3 => Some(Syssrc::Wdtev),
            2 => Some(Syssrc::Lockupev),
            1 => Some(Syssrc::Cpurstev),
            0 => Some(Syssrc::Lflossev),
            _ => None,
        }
    }
    #[doc = "Digital Error reset event"]
    #[inline(always)]
    pub fn is_derrev(&self) -> bool {
        *self == Syssrc::Derrev
    }
    #[doc = "Analog Error reset event"]
    #[inline(always)]
    pub fn is_aerrev(&self) -> bool {
        *self == Syssrc::Aerrev
    }
    #[doc = "Analog FSM timeout event"]
    #[inline(always)]
    pub fn is_afsmev(&self) -> bool {
        *self == Syssrc::Afsmev
    }
    #[doc = "Serial Wire Debug reset event"]
    #[inline(always)]
    pub fn is_swdrstev(&self) -> bool {
        *self == Syssrc::Swdrstev
    }
    #[doc = "System reset event"]
    #[inline(always)]
    pub fn is_sysrstev(&self) -> bool {
        *self == Syssrc::Sysrstev
    }
    #[doc = "Watchdog timeout event"]
    #[inline(always)]
    pub fn is_wdtev(&self) -> bool {
        *self == Syssrc::Wdtev
    }
    #[doc = "CPU LOCKUP event"]
    #[inline(always)]
    pub fn is_lockupev(&self) -> bool {
        *self == Syssrc::Lockupev
    }
    #[doc = "CPU reset event"]
    #[inline(always)]
    pub fn is_cpurstev(&self) -> bool {
        *self == Syssrc::Cpurstev
    }
    #[doc = "LF clock loss event"]
    #[inline(always)]
    pub fn is_lflossev(&self) -> bool {
        *self == Syssrc::Lflossev
    }
}
#[doc = "Field `SYSSRC` writer - 7:4\\]
Shows which reset event that triggered SYSRESET in RESETSRC"]
pub type SyssrcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Syssrc>;
impl<'a, REG> SyssrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Digital Error reset event"]
    #[inline(always)]
    pub fn derrev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Derrev)
    }
    #[doc = "Analog Error reset event"]
    #[inline(always)]
    pub fn aerrev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Aerrev)
    }
    #[doc = "Analog FSM timeout event"]
    #[inline(always)]
    pub fn afsmev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Afsmev)
    }
    #[doc = "Serial Wire Debug reset event"]
    #[inline(always)]
    pub fn swdrstev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Swdrstev)
    }
    #[doc = "System reset event"]
    #[inline(always)]
    pub fn sysrstev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Sysrstev)
    }
    #[doc = "Watchdog timeout event"]
    #[inline(always)]
    pub fn wdtev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Wdtev)
    }
    #[doc = "CPU LOCKUP event"]
    #[inline(always)]
    pub fn lockupev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Lockupev)
    }
    #[doc = "CPU reset event"]
    #[inline(always)]
    pub fn cpurstev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Cpurstev)
    }
    #[doc = "LF clock loss event"]
    #[inline(always)]
    pub fn lflossev(self) -> &'a mut crate::W<REG> {
        self.variant(Syssrc::Lflossev)
    }
}
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "16:16\\]
Wakeup from SHUTDOWN on an I/O event flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iowusd {
    #[doc = "1: Wakeup from SHUTDOWN triggered by an I/O event."]
    Trig = 1,
    #[doc = "0: Wakeup from SHUTDOWN not triggered by an I/O event."]
    NoTrig = 0,
}
impl From<Iowusd> for bool {
    #[inline(always)]
    fn from(variant: Iowusd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOWUSD` reader - 16:16\\]
Wakeup from SHUTDOWN on an I/O event flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
pub type IowusdR = crate::BitReader<Iowusd>;
impl IowusdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iowusd {
        match self.bits {
            true => Iowusd::Trig,
            false => Iowusd::NoTrig,
        }
    }
    #[doc = "Wakeup from SHUTDOWN triggered by an I/O event."]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Iowusd::Trig
    }
    #[doc = "Wakeup from SHUTDOWN not triggered by an I/O event."]
    #[inline(always)]
    pub fn is_no_trig(&self) -> bool {
        *self == Iowusd::NoTrig
    }
}
#[doc = "Field `IOWUSD` writer - 16:16\\]
Wakeup from SHUTDOWN on an I/O event flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
pub type IowusdW<'a, REG> = crate::BitWriter<'a, REG, Iowusd>;
impl<'a, REG> IowusdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup from SHUTDOWN triggered by an I/O event."]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Iowusd::Trig)
    }
    #[doc = "Wakeup from SHUTDOWN not triggered by an I/O event."]
    #[inline(always)]
    pub fn no_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Iowusd::NoTrig)
    }
}
#[doc = "17:17\\]
Wakeup from SHUTDOWN flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sddet {
    #[doc = "1: Wakeup from SHUTDOWN mode"]
    Trig = 1,
    #[doc = "0: Wakeup from SHUTDOWN mode not triggered"]
    NoTrig = 0,
}
impl From<Sddet> for bool {
    #[inline(always)]
    fn from(variant: Sddet) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SDDET` reader - 17:17\\]
Wakeup from SHUTDOWN flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
pub type SddetR = crate::BitReader<Sddet>;
impl SddetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sddet {
        match self.bits {
            true => Sddet::Trig,
            false => Sddet::NoTrig,
        }
    }
    #[doc = "Wakeup from SHUTDOWN mode"]
    #[inline(always)]
    pub fn is_trig(&self) -> bool {
        *self == Sddet::Trig
    }
    #[doc = "Wakeup from SHUTDOWN mode not triggered"]
    #[inline(always)]
    pub fn is_no_trig(&self) -> bool {
        *self == Sddet::NoTrig
    }
}
#[doc = "Field `SDDET` writer - 17:17\\]
Wakeup from SHUTDOWN flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
pub type SddetW<'a, REG> = crate::BitWriter<'a, REG, Sddet>;
impl<'a, REG> SddetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup from SHUTDOWN mode"]
    #[inline(always)]
    pub fn trig(self) -> &'a mut crate::W<REG> {
        self.variant(Sddet::Trig)
    }
    #[doc = "Wakeup from SHUTDOWN mode not triggered"]
    #[inline(always)]
    pub fn no_trig(self) -> &'a mut crate::W<REG> {
        self.variant(Sddet::NoTrig)
    }
}
#[doc = "Field `RESERVED18` reader - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Shows the root cause of the last system reset. More than one reported reset source can have been active during the last system reset, but only the root cause is reported. If reset cause is SYSRESET or PINRESET, the other reset flags must be read to determine actual root cause."]
    #[inline(always)]
    pub fn resetsrc(&self) -> ResetsrcR {
        ResetsrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
System reset triggered by TSD event"]
    #[inline(always)]
    pub fn tsdev(&self) -> TsdevR {
        TsdevR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Shows which reset event that triggered SYSRESET in RESETSRC"]
    #[inline(always)]
    pub fn syssrc(&self) -> SyssrcR {
        SyssrcR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Wakeup from SHUTDOWN on an I/O event flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
    #[inline(always)]
    pub fn iowusd(&self) -> IowusdR {
        IowusdR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Wakeup from SHUTDOWN flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
    #[inline(always)]
    pub fn sddet(&self) -> SddetR {
        SddetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Shows the root cause of the last system reset. More than one reported reset source can have been active during the last system reset, but only the root cause is reported. If reset cause is SYSRESET or PINRESET, the other reset flags must be read to determine actual root cause."]
    #[inline(always)]
    #[must_use]
    pub fn resetsrc(&mut self) -> ResetsrcW<RststaSpec> {
        ResetsrcW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
System reset triggered by TSD event"]
    #[inline(always)]
    #[must_use]
    pub fn tsdev(&mut self) -> TsdevW<RststaSpec> {
        TsdevW::new(self, 3)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Shows which reset event that triggered SYSRESET in RESETSRC"]
    #[inline(always)]
    #[must_use]
    pub fn syssrc(&mut self) -> SyssrcW<RststaSpec> {
        SyssrcW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<RststaSpec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Wakeup from SHUTDOWN on an I/O event flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn iowusd(&mut self) -> IowusdW<RststaSpec> {
        IowusdW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Wakeup from SHUTDOWN flag. Note: This flag will be cleared when SLPCTL.SLPN is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn sddet(&mut self) -> SddetW<RststaSpec> {
        SddetW::new(self, 17)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<RststaSpec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "Reset Status. This register contains the reset source and SHUTDOWN wakeup source for the system. Check WUSTA.SRC first to ensure that wakeup from STANDBY is not set. The capture feature is not rearmed until all of the possible reset sources have been released and the result has been copied to this register. During the copy and rearm process it is one 24MHz period in which an eventual new system reset will be reported as Power on reset regardless of the root cause.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rststa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RststaSpec;
impl crate::RegisterSpec for RststaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststa::R`](R) reader structure"]
impl crate::Readable for RststaSpec {}
#[doc = "`write(|w| ..)` method takes [`rststa::W`](W) writer structure"]
impl crate::Writable for RststaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTSTA to value 0"]
impl crate::Resettable for RststaSpec {
    const RESET_VALUE: u32 = 0;
}
