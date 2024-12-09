#[doc = "Register `PEROPRATE` reader"]
pub type R = crate::R<PeroprateSpec>;
#[doc = "Register `PEROPRATE` writer"]
pub type W = crate::W<PeroprateSpec>;
#[doc = "0:0\\]
Select operational rate for AUX_MAC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MacOpRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<MacOpRate> for bool {
    #[inline(always)]
    fn from(variant: MacOpRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAC_OP_RATE` reader - 0:0\\]
Select operational rate for AUX_MAC."]
pub type MacOpRateR = crate::BitReader<MacOpRate>;
impl MacOpRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MacOpRate {
        match self.bits {
            true => MacOpRate::BusRate,
            false => MacOpRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == MacOpRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == MacOpRate::SceRate
    }
}
#[doc = "Field `MAC_OP_RATE` writer - 0:0\\]
Select operational rate for AUX_MAC."]
pub type MacOpRateW<'a, REG> = crate::BitWriter<'a, REG, MacOpRate>;
impl<'a, REG> MacOpRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(MacOpRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(MacOpRate::SceRate)
    }
}
#[doc = "1:1\\]
Select operational rate for AUX_SPIM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpimOpRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<SpimOpRate> for bool {
    #[inline(always)]
    fn from(variant: SpimOpRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIM_OP_RATE` reader - 1:1\\]
Select operational rate for AUX_SPIM."]
pub type SpimOpRateR = crate::BitReader<SpimOpRate>;
impl SpimOpRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpimOpRate {
        match self.bits {
            true => SpimOpRate::BusRate,
            false => SpimOpRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == SpimOpRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == SpimOpRate::SceRate
    }
}
#[doc = "Field `SPIM_OP_RATE` writer - 1:1\\]
Select operational rate for AUX_SPIM."]
pub type SpimOpRateW<'a, REG> = crate::BitWriter<'a, REG, SpimOpRate>;
impl<'a, REG> SpimOpRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(SpimOpRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(SpimOpRate::SceRate)
    }
}
#[doc = "2:2\\]
Select operational rate for AUX_TIMER01.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timer01OpRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<Timer01OpRate> for bool {
    #[inline(always)]
    fn from(variant: Timer01OpRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER01_OP_RATE` reader - 2:2\\]
Select operational rate for AUX_TIMER01."]
pub type Timer01OpRateR = crate::BitReader<Timer01OpRate>;
impl Timer01OpRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timer01OpRate {
        match self.bits {
            true => Timer01OpRate::BusRate,
            false => Timer01OpRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == Timer01OpRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == Timer01OpRate::SceRate
    }
}
#[doc = "Field `TIMER01_OP_RATE` writer - 2:2\\]
Select operational rate for AUX_TIMER01."]
pub type Timer01OpRateW<'a, REG> = crate::BitWriter<'a, REG, Timer01OpRate>;
impl<'a, REG> Timer01OpRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(Timer01OpRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(Timer01OpRate::SceRate)
    }
}
#[doc = "3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnaifDacOpRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<AnaifDacOpRate> for bool {
    #[inline(always)]
    fn from(variant: AnaifDacOpRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANAIF_DAC_OP_RATE` reader - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
pub type AnaifDacOpRateR = crate::BitReader<AnaifDacOpRate>;
impl AnaifDacOpRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AnaifDacOpRate {
        match self.bits {
            true => AnaifDacOpRate::BusRate,
            false => AnaifDacOpRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AnaifDacOpRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AnaifDacOpRate::SceRate
    }
}
#[doc = "Field `ANAIF_DAC_OP_RATE` writer - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
pub type AnaifDacOpRateW<'a, REG> = crate::BitWriter<'a, REG, AnaifDacOpRate>;
impl<'a, REG> AnaifDacOpRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AnaifDacOpRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AnaifDacOpRate::SceRate)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    pub fn mac_op_rate(&self) -> MacOpRateR {
        MacOpRateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    pub fn spim_op_rate(&self) -> SpimOpRateR {
        SpimOpRateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    pub fn timer01_op_rate(&self) -> Timer01OpRateR {
        Timer01OpRateR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    pub fn anaif_dac_op_rate(&self) -> AnaifDacOpRateR {
        AnaifDacOpRateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select operational rate for AUX_MAC."]
    #[inline(always)]
    #[must_use]
    pub fn mac_op_rate(&mut self) -> MacOpRateW<PeroprateSpec> {
        MacOpRateW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select operational rate for AUX_SPIM."]
    #[inline(always)]
    #[must_use]
    pub fn spim_op_rate(&mut self) -> SpimOpRateW<PeroprateSpec> {
        SpimOpRateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select operational rate for AUX_TIMER01."]
    #[inline(always)]
    #[must_use]
    pub fn timer01_op_rate(&mut self) -> Timer01OpRateW<PeroprateSpec> {
        Timer01OpRateW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Select operational rate for AUX_ANAIF DAC sample clock state machine."]
    #[inline(always)]
    #[must_use]
    pub fn anaif_dac_op_rate(&mut self) -> AnaifDacOpRateW<PeroprateSpec> {
        AnaifDacOpRateW::new(self, 3)
    }
}
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. You must select SCE rate when AUX_SCE uses such peripheral or an event produced by it. You must select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peroprate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peroprate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeroprateSpec;
impl crate::RegisterSpec for PeroprateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peroprate::R`](R) reader structure"]
impl crate::Readable for PeroprateSpec {}
#[doc = "`write(|w| ..)` method takes [`peroprate::W`](W) writer structure"]
impl crate::Writable for PeroprateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEROPRATE to value 0"]
impl crate::Resettable for PeroprateSpec {
    const RESET_VALUE: u32 = 0;
}
