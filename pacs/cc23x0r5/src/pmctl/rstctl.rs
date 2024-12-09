#[doc = "Register `RSTCTL` reader"]
pub type R = crate::R<RstctlSpec>;
#[doc = "Register `RSTCTL` writer"]
pub type W = crate::W<RstctlSpec>;
#[doc = "0:0\\]
Trigger system reset, which will reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to SYSRSTEV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sysrst {
    #[doc = "1: Trigger a system reset."]
    Set = 1,
    #[doc = "0: No effect"]
    Noeff = 0,
}
impl From<Sysrst> for bool {
    #[inline(always)]
    fn from(variant: Sysrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRST` reader - 0:0\\]
Trigger system reset, which will reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to SYSRSTEV."]
pub type SysrstR = crate::BitReader<Sysrst>;
impl SysrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sysrst {
        match self.bits {
            true => Sysrst::Set,
            false => Sysrst::Noeff,
        }
    }
    #[doc = "Trigger a system reset."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Sysrst::Set
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Sysrst::Noeff
    }
}
#[doc = "Field `SYSRST` writer - 0:0\\]
Trigger system reset, which will reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to SYSRSTEV."]
pub type SysrstW<'a, REG> = crate::BitWriter<'a, REG, Sysrst>;
impl<'a, REG> SysrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger a system reset."]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrst::Set)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrst::Noeff)
    }
}
#[doc = "1:1\\]
TSD (Thermal Shutdown) enable. TSD will trigger an immediate system reset, which reset the entire device and causes a reboot of the system. The device will be in reset until released by the TSD IP. The system reset event is captured as RSTSTA.TSDEV flag set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsden {
    #[doc = "1: Temperature shutdown comparator enable. Note: If TSD IP not present, see DESCEX.TSD, enable will have no effect."]
    En = 1,
    #[doc = "0: No effect"]
    Noeff = 0,
}
impl From<Tsden> for bool {
    #[inline(always)]
    fn from(variant: Tsden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSDEN` reader - 1:1\\]
TSD (Thermal Shutdown) enable. TSD will trigger an immediate system reset, which reset the entire device and causes a reboot of the system. The device will be in reset until released by the TSD IP. The system reset event is captured as RSTSTA.TSDEV flag set."]
pub type TsdenR = crate::BitReader<Tsden>;
impl TsdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsden {
        match self.bits {
            true => Tsden::En,
            false => Tsden::Noeff,
        }
    }
    #[doc = "Temperature shutdown comparator enable. Note: If TSD IP not present, see DESCEX.TSD, enable will have no effect."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Tsden::En
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_noeff(&self) -> bool {
        *self == Tsden::Noeff
    }
}
#[doc = "Field `TSDEN` writer - 1:1\\]
TSD (Thermal Shutdown) enable. TSD will trigger an immediate system reset, which reset the entire device and causes a reboot of the system. The device will be in reset until released by the TSD IP. The system reset event is captured as RSTSTA.TSDEV flag set."]
pub type TsdenW<'a, REG> = crate::BitWriter<'a, REG, Tsden>;
impl<'a, REG> TsdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Temperature shutdown comparator enable. Note: If TSD IP not present, see DESCEX.TSD, enable will have no effect."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Tsden::En)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Tsden::Noeff)
    }
}
#[doc = "2:2\\]
LF clock loss reset enable. Trigger system reset when LF clock loss is detected, which reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to LFLOSSEV.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfloss {
    #[doc = "1: LF clock loss detection will trigger a system reset."]
    Armed = 1,
    #[doc = "0: LF clock loss detection will not trigger a system reset."]
    Disarmed = 0,
}
impl From<Lfloss> for bool {
    #[inline(always)]
    fn from(variant: Lfloss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFLOSS` reader - 2:2\\]
LF clock loss reset enable. Trigger system reset when LF clock loss is detected, which reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to LFLOSSEV."]
pub type LflossR = crate::BitReader<Lfloss>;
impl LflossR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfloss {
        match self.bits {
            true => Lfloss::Armed,
            false => Lfloss::Disarmed,
        }
    }
    #[doc = "LF clock loss detection will trigger a system reset."]
    #[inline(always)]
    pub fn is_armed(&self) -> bool {
        *self == Lfloss::Armed
    }
    #[doc = "LF clock loss detection will not trigger a system reset."]
    #[inline(always)]
    pub fn is_disarmed(&self) -> bool {
        *self == Lfloss::Disarmed
    }
}
#[doc = "Field `LFLOSS` writer - 2:2\\]
LF clock loss reset enable. Trigger system reset when LF clock loss is detected, which reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to LFLOSSEV."]
pub type LflossW<'a, REG> = crate::BitWriter<'a, REG, Lfloss>;
impl<'a, REG> LflossW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LF clock loss detection will trigger a system reset."]
    #[inline(always)]
    pub fn armed(self) -> &'a mut crate::W<REG> {
        self.variant(Lfloss::Armed)
    }
    #[doc = "LF clock loss detection will not trigger a system reset."]
    #[inline(always)]
    pub fn disarmed(self) -> &'a mut crate::W<REG> {
        self.variant(Lfloss::Disarmed)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Trigger system reset, which will reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to SYSRSTEV."]
    #[inline(always)]
    pub fn sysrst(&self) -> SysrstR {
        SysrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TSD (Thermal Shutdown) enable. TSD will trigger an immediate system reset, which reset the entire device and causes a reboot of the system. The device will be in reset until released by the TSD IP. The system reset event is captured as RSTSTA.TSDEV flag set."]
    #[inline(always)]
    pub fn tsden(&self) -> TsdenR {
        TsdenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
LF clock loss reset enable. Trigger system reset when LF clock loss is detected, which reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to LFLOSSEV."]
    #[inline(always)]
    pub fn lfloss(&self) -> LflossR {
        LflossR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Trigger system reset, which will reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to SYSRSTEV."]
    #[inline(always)]
    #[must_use]
    pub fn sysrst(&mut self) -> SysrstW<RstctlSpec> {
        SysrstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TSD (Thermal Shutdown) enable. TSD will trigger an immediate system reset, which reset the entire device and causes a reboot of the system. The device will be in reset until released by the TSD IP. The system reset event is captured as RSTSTA.TSDEV flag set."]
    #[inline(always)]
    #[must_use]
    pub fn tsden(&mut self) -> TsdenW<RstctlSpec> {
        TsdenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
LF clock loss reset enable. Trigger system reset when LF clock loss is detected, which reset the entire device and causes a reboot of the system. The system reset event is captured as RSTSTA.RESETSRC set to SYSRESET and RSTSTA.SYSSRC set to LFLOSSEV."]
    #[inline(always)]
    #[must_use]
    pub fn lfloss(&mut self) -> LflossW<RstctlSpec> {
        LflossW::new(self, 2)
    }
}
#[doc = "Reset Control Register. This register configures and controls system reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstctlSpec;
impl crate::RegisterSpec for RstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstctl::R`](R) reader structure"]
impl crate::Readable for RstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`rstctl::W`](W) writer structure"]
impl crate::Writable for RstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSTCTL to value 0"]
impl crate::Resettable for RstctlSpec {
    const RESET_VALUE: u32 = 0;
}
