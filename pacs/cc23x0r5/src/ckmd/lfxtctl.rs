#[doc = "Register `LFXTCTL` reader"]
pub type R = crate::R<LfxtctlSpec>;
#[doc = "Register `LFXTCTL` writer"]
pub type W = crate::W<LfxtctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
LFXT enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
LFXT enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMPREGEN` reader - 1:1\\]
Amplitude regulation loop enable"]
pub type AmpregenR = crate::BitReader;
#[doc = "Field `AMPREGEN` writer - 1:1\\]
Amplitude regulation loop enable"]
pub type AmpregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPBUFEN` reader - 2:2\\]
Control the buffer used. In normal operation, low-power buffer is used in all device modes. The high-performance buffer is only used for test purposes."]
pub type HpbufenR = crate::BitReader;
#[doc = "Field `HPBUFEN` writer - 2:2\\]
Control the buffer used. In normal operation, low-power buffer is used in all device modes. The high-performance buffer is only used for test purposes."]
pub type HpbufenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGBIAS` reader - 5:4\\]
Regulation loop bias resistor value This value depends on the crystal and needs to be configured by Firmware."]
pub type RegbiasR = crate::FieldReader;
#[doc = "Field `REGBIAS` writer - 5:4\\]
Regulation loop bias resistor value This value depends on the crystal and needs to be configured by Firmware."]
pub type RegbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIASBOOST` reader - 7:6\\]
Boost oscillator amplitude This value depends on the crystal and needs to be configured by Firmware."]
pub type BiasboostR = crate::FieldReader;
#[doc = "Field `BIASBOOST` writer - 7:6\\]
Boost oscillator amplitude This value depends on the crystal and needs to be configured by Firmware."]
pub type BiasboostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPBIAS` reader - 11:8\\]
Adjust current mirror ratio into oscillator core. This value is depending on crystal and is set by FW. This field uses a 2's complement encoding."]
pub type AmpbiasR = crate::FieldReader;
#[doc = "Field `AMPBIAS` writer - 11:8\\]
Adjust current mirror ratio into oscillator core. This value is depending on crystal and is set by FW. This field uses a 2's complement encoding."]
pub type AmpbiasW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "12:12\\]
Control the BIAS current of the input amp in LP buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bufbias {
    #[doc = "1: Maximum bias current: 50nA"]
    Max = 1,
    #[doc = "0: Minimum bias current: 25nA"]
    Min = 0,
}
impl From<Bufbias> for bool {
    #[inline(always)]
    fn from(variant: Bufbias) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUFBIAS` reader - 12:12\\]
Control the BIAS current of the input amp in LP buffer"]
pub type BufbiasR = crate::BitReader<Bufbias>;
impl BufbiasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bufbias {
        match self.bits {
            true => Bufbias::Max,
            false => Bufbias::Min,
        }
    }
    #[doc = "Maximum bias current: 50nA"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Bufbias::Max
    }
    #[doc = "Minimum bias current: 25nA"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Bufbias::Min
    }
}
#[doc = "Field `BUFBIAS` writer - 12:12\\]
Control the BIAS current of the input amp in LP buffer"]
pub type BufbiasW<'a, REG> = crate::BitWriter<'a, REG, Bufbias>;
impl<'a, REG> BufbiasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maximum bias current: 50nA"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Bufbias::Max)
    }
    #[doc = "Minimum bias current: 25nA"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Bufbias::Min)
    }
}
#[doc = "14:13\\]
Leakage compensation control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leakcomp {
    #[doc = "3: No leakage compensation"]
    Off = 3,
    #[doc = "1: Half leakage compensation"]
    Half = 1,
    #[doc = "0: Full leakage compensation"]
    Full = 0,
}
impl From<Leakcomp> for u8 {
    #[inline(always)]
    fn from(variant: Leakcomp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leakcomp {
    type Ux = u8;
}
impl crate::IsEnum for Leakcomp {}
#[doc = "Field `LEAKCOMP` reader - 14:13\\]
Leakage compensation control"]
pub type LeakcompR = crate::FieldReader<Leakcomp>;
impl LeakcompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Leakcomp> {
        match self.bits {
            3 => Some(Leakcomp::Off),
            1 => Some(Leakcomp::Half),
            0 => Some(Leakcomp::Full),
            _ => None,
        }
    }
    #[doc = "No leakage compensation"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Leakcomp::Off
    }
    #[doc = "Half leakage compensation"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Leakcomp::Half
    }
    #[doc = "Full leakage compensation"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Leakcomp::Full
    }
}
#[doc = "Field `LEAKCOMP` writer - 14:13\\]
Leakage compensation control"]
pub type LeakcompW<'a, REG> = crate::FieldWriter<'a, REG, 2, Leakcomp>;
impl<'a, REG> LeakcompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No leakage compensation"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Leakcomp::Off)
    }
    #[doc = "Half leakage compensation"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Leakcomp::Half)
    }
    #[doc = "Full leakage compensation"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Leakcomp::Full)
    }
}
#[doc = "Field `RESERVED15` reader - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved15R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
LFXT enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Amplitude regulation loop enable"]
    #[inline(always)]
    pub fn ampregen(&self) -> AmpregenR {
        AmpregenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Control the buffer used. In normal operation, low-power buffer is used in all device modes. The high-performance buffer is only used for test purposes."]
    #[inline(always)]
    pub fn hpbufen(&self) -> HpbufenR {
        HpbufenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Regulation loop bias resistor value This value depends on the crystal and needs to be configured by Firmware."]
    #[inline(always)]
    pub fn regbias(&self) -> RegbiasR {
        RegbiasR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Boost oscillator amplitude This value depends on the crystal and needs to be configured by Firmware."]
    #[inline(always)]
    pub fn biasboost(&self) -> BiasboostR {
        BiasboostR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Adjust current mirror ratio into oscillator core. This value is depending on crystal and is set by FW. This field uses a 2's complement encoding."]
    #[inline(always)]
    pub fn ampbias(&self) -> AmpbiasR {
        AmpbiasR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Control the BIAS current of the input amp in LP buffer"]
    #[inline(always)]
    pub fn bufbias(&self) -> BufbiasR {
        BufbiasR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Leakage compensation control"]
    #[inline(always)]
    pub fn leakcomp(&self) -> LeakcompR {
        LeakcompR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
LFXT enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<LfxtctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Amplitude regulation loop enable"]
    #[inline(always)]
    #[must_use]
    pub fn ampregen(&mut self) -> AmpregenW<LfxtctlSpec> {
        AmpregenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Control the buffer used. In normal operation, low-power buffer is used in all device modes. The high-performance buffer is only used for test purposes."]
    #[inline(always)]
    #[must_use]
    pub fn hpbufen(&mut self) -> HpbufenW<LfxtctlSpec> {
        HpbufenW::new(self, 2)
    }
    #[doc = "Bits 4:5 - 5:4\\]
Regulation loop bias resistor value This value depends on the crystal and needs to be configured by Firmware."]
    #[inline(always)]
    #[must_use]
    pub fn regbias(&mut self) -> RegbiasW<LfxtctlSpec> {
        RegbiasW::new(self, 4)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Boost oscillator amplitude This value depends on the crystal and needs to be configured by Firmware."]
    #[inline(always)]
    #[must_use]
    pub fn biasboost(&mut self) -> BiasboostW<LfxtctlSpec> {
        BiasboostW::new(self, 6)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Adjust current mirror ratio into oscillator core. This value is depending on crystal and is set by FW. This field uses a 2's complement encoding."]
    #[inline(always)]
    #[must_use]
    pub fn ampbias(&mut self) -> AmpbiasW<LfxtctlSpec> {
        AmpbiasW::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
Control the BIAS current of the input amp in LP buffer"]
    #[inline(always)]
    #[must_use]
    pub fn bufbias(&mut self) -> BufbiasW<LfxtctlSpec> {
        BufbiasW::new(self, 12)
    }
    #[doc = "Bits 13:14 - 14:13\\]
Leakage compensation control"]
    #[inline(always)]
    #[must_use]
    pub fn leakcomp(&mut self) -> LeakcompW<LfxtctlSpec> {
        LeakcompW::new(self, 13)
    }
}
#[doc = "Low frequency crystal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfxtctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfxtctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfxtctlSpec;
impl crate::RegisterSpec for LfxtctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfxtctl::R`](R) reader structure"]
impl crate::Readable for LfxtctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfxtctl::W`](W) writer structure"]
impl crate::Writable for LfxtctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFXTCTL to value 0"]
impl crate::Resettable for LfxtctlSpec {
    const RESET_VALUE: u32 = 0;
}
