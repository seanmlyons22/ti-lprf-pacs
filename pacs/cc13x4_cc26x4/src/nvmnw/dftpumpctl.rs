#[doc = "Register `DFTPUMPCTL` reader"]
pub type R = crate::R<DftpumpctlSpec>;
#[doc = "Register `DFTPUMPCTL` writer"]
pub type W = crate::W<DftpumpctlSpec>;
#[doc = "6:0\\]
TCR test mode to be applied to the pump\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcr {
    #[doc = "127: Maximum value"]
    Maximum = 127,
    #[doc = "0: Minimum value"]
    Minimum = 0,
}
impl From<Tcr> for u8 {
    #[inline(always)]
    fn from(variant: Tcr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcr {
    type Ux = u8;
}
impl crate::IsEnum for Tcr {}
#[doc = "Field `TCR` reader - 6:0\\]
TCR test mode to be applied to the pump"]
pub type TcrR = crate::FieldReader<Tcr>;
impl TcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcr> {
        match self.bits {
            127 => Some(Tcr::Maximum),
            0 => Some(Tcr::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Tcr::Maximum
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Tcr::Minimum
    }
}
#[doc = "Field `TCR` writer - 6:0\\]
TCR test mode to be applied to the pump"]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 7, Tcr>;
impl<'a, REG> TcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::Maximum)
    }
    #[doc = "Minimum value"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::Minimum)
    }
}
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pumpclken {
    #[doc = "1: Force pump clock oscillator to be enabled."]
    Enable = 1,
    #[doc = "0: Allow pump clock oscillator to be controlled by hardware."]
    Hwctl = 0,
}
impl From<Pumpclken> for bool {
    #[inline(always)]
    fn from(variant: Pumpclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUMPCLKEN` reader - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
pub type PumpclkenR = crate::BitReader<Pumpclken>;
impl PumpclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pumpclken {
        match self.bits {
            true => Pumpclken::Enable,
            false => Pumpclken::Hwctl,
        }
    }
    #[doc = "Force pump clock oscillator to be enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Pumpclken::Enable
    }
    #[doc = "Allow pump clock oscillator to be controlled by hardware."]
    #[inline(always)]
    pub fn is_hwctl(&self) -> bool {
        *self == Pumpclken::Hwctl
    }
}
#[doc = "Field `PUMPCLKEN` writer - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
pub type PumpclkenW<'a, REG> = crate::BitWriter<'a, REG, Pumpclken>;
impl<'a, REG> PumpclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Force pump clock oscillator to be enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Pumpclken::Enable)
    }
    #[doc = "Allow pump clock oscillator to be controlled by hardware."]
    #[inline(always)]
    pub fn hwctl(self) -> &'a mut crate::W<REG> {
        self.variant(Pumpclken::Hwctl)
    }
}
#[doc = "9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssen {
    #[doc = "1: Enable"]
    Enable = 1,
    #[doc = "0: Disable"]
    Disable = 0,
}
impl From<Ssen> for bool {
    #[inline(always)]
    fn from(variant: Ssen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSEN` reader - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
pub type SsenR = crate::BitReader<Ssen>;
impl SsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssen {
        match self.bits {
            true => Ssen::Enable,
            false => Ssen::Disable,
        }
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ssen::Enable
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ssen::Disable
    }
}
#[doc = "Field `SSEN` writer - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
pub type SsenW<'a, REG> = crate::BitWriter<'a, REG, Ssen>;
impl<'a, REG> SsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssen::Enable)
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ssen::Disable)
    }
}
#[doc = "Field `RESERVED10` reader - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader;
#[doc = "Field `RESERVED10` writer - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CONFIGPMP` reader - 15:12\\]
Pump configuration control. LP, HP operation"]
pub type ConfigpmpR = crate::FieldReader;
#[doc = "Field `CONFIGPMP` writer - 15:12\\]
Pump configuration control. LP, HP operation"]
pub type ConfigpmpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IREFEVCTL` reader - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
pub type IrefevctlR = crate::FieldReader;
#[doc = "Field `IREFEVCTL` writer - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
pub type IrefevctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED19` writer - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the pump"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
    #[inline(always)]
    pub fn pumpclken(&self) -> PumpclkenR {
        PumpclkenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
    #[inline(always)]
    pub fn ssen(&self) -> SsenR {
        SsenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Pump configuration control. LP, HP operation"]
    #[inline(always)]
    pub fn configpmp(&self) -> ConfigpmpR {
        ConfigpmpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
    #[inline(always)]
    pub fn irefevctl(&self) -> IrefevctlR {
        IrefevctlR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
TCR test mode to be applied to the pump"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TcrW<DftpumpctlSpec> {
        TcrW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<DftpumpctlSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Allows direct control of the pump oscillator which is used to generate pumpclk. Normally, enable/disable of pumpclk is under NoWrapper state machine control. This bit allows system to enable the clock independently."]
    #[inline(always)]
    #[must_use]
    pub fn pumpclken(&mut self) -> PumpclkenW<DftpumpctlSpec> {
        PumpclkenW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Dither control for oscillator Enumeration: 0: Disable Dither 1: Enable Dither"]
    #[inline(always)]
    #[must_use]
    pub fn ssen(&mut self) -> SsenW<DftpumpctlSpec> {
        SsenW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<DftpumpctlSpec> {
        Reserved10W::new(self, 10)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Pump configuration control. LP, HP operation"]
    #[inline(always)]
    #[must_use]
    pub fn configpmp(&mut self) -> ConfigpmpW<DftpumpctlSpec> {
        ConfigpmpW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
IREFEV control IREFVRD, REFTC, IREFCONST, IREFCCOR blocks in IREFEV"]
    #[inline(always)]
    #[must_use]
    pub fn irefevctl(&mut self) -> IrefevctlW<DftpumpctlSpec> {
        IrefevctlW::new(self, 16)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> Reserved19W<DftpumpctlSpec> {
        Reserved19W::new(self, 19)
    }
}
#[doc = "DFT Pump Control Register This allows some configuration of pump parameters during test. This register is only writable when DFTEN.ENABLE is set. This register is blocked for writes after a 1 is written to the CMDEXEC register and prior to STATCMD.DONE being set by the NoWrapper hardware.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpumpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpumpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftpumpctlSpec;
impl crate::RegisterSpec for DftpumpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftpumpctl::R`](R) reader structure"]
impl crate::Readable for DftpumpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dftpumpctl::W`](W) writer structure"]
impl crate::Writable for DftpumpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTPUMPCTL to value 0x1000"]
impl crate::Resettable for DftpumpctlSpec {
    const RESET_VALUE: u32 = 0x1000;
}
