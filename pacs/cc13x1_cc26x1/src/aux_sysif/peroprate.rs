#[doc = "Register `PEROPRATE` reader"]
pub type R = crate::R<PeroprateSpec>;
#[doc = "Register `PEROPRATE` writer"]
pub type W = crate::W<PeroprateSpec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
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
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
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
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peroprate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peroprate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
