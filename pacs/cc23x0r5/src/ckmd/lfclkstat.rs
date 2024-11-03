#[doc = "Register `LFCLKSTAT` reader"]
pub type R = crate::R<LfclkstatSpec>;
#[doc = "Register `LFCLKSTAT` writer"]
pub type W = crate::W<LfclkstatSpec>;
#[doc = "Field `LFINC` reader - 21:0\\]
Measured value of LFINC. Given in microseconds with 16 fractional bits. This value is calculated by Hardware. It is the LFCLK period according to CLKULL cycles."]
pub type LfincR = crate::FieldReader<u32>;
#[doc = "Field `LFINC` writer - 21:0\\]
Measured value of LFINC. Given in microseconds with 16 fractional bits. This value is calculated by Hardware. It is the LFCLK period according to CLKULL cycles."]
pub type LfincW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "23:22\\]
Source of LFINC used by the RTC. This value depends on LFINCOVR.OVERRIDE, LF clock availability, HF tracking loop status and the device state (ACTIVE/STANDBY).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lfincsrc {
    #[doc = "3: Using FAKE LFTICKs with corresponding LFINC value."]
    Fake = 3,
    #[doc = "2: Using override value from LFINCOVR.LFINC"]
    Override = 2,
    #[doc = "1: Using filtered / average value. This value is updated by hardware and can be read and updated in LFINCCTL.INT."]
    Avg = 1,
    #[doc = "0: Using measured value. This value is updated by hardware and can be read from LFINC."]
    Meas = 0,
}
impl From<Lfincsrc> for u8 {
    #[inline(always)]
    fn from(variant: Lfincsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lfincsrc {
    type Ux = u8;
}
impl crate::IsEnum for Lfincsrc {}
#[doc = "Field `LFINCSRC` reader - 23:22\\]
Source of LFINC used by the RTC. This value depends on LFINCOVR.OVERRIDE, LF clock availability, HF tracking loop status and the device state (ACTIVE/STANDBY)."]
pub type LfincsrcR = crate::FieldReader<Lfincsrc>;
impl LfincsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfincsrc {
        match self.bits {
            3 => Lfincsrc::Fake,
            2 => Lfincsrc::Override,
            1 => Lfincsrc::Avg,
            0 => Lfincsrc::Meas,
            _ => unreachable!(),
        }
    }
    #[doc = "Using FAKE LFTICKs with corresponding LFINC value."]
    #[inline(always)]
    pub fn is_fake(&self) -> bool {
        *self == Lfincsrc::Fake
    }
    #[doc = "Using override value from LFINCOVR.LFINC"]
    #[inline(always)]
    pub fn is_override(&self) -> bool {
        *self == Lfincsrc::Override
    }
    #[doc = "Using filtered / average value. This value is updated by hardware and can be read and updated in LFINCCTL.INT."]
    #[inline(always)]
    pub fn is_avg(&self) -> bool {
        *self == Lfincsrc::Avg
    }
    #[doc = "Using measured value. This value is updated by hardware and can be read from LFINC."]
    #[inline(always)]
    pub fn is_meas(&self) -> bool {
        *self == Lfincsrc::Meas
    }
}
#[doc = "Field `LFINCSRC` writer - 23:22\\]
Source of LFINC used by the RTC. This value depends on LFINCOVR.OVERRIDE, LF clock availability, HF tracking loop status and the device state (ACTIVE/STANDBY)."]
pub type LfincsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lfincsrc, crate::Safe>;
impl<'a, REG> LfincsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Using FAKE LFTICKs with corresponding LFINC value."]
    #[inline(always)]
    pub fn fake(self) -> &'a mut crate::W<REG> {
        self.variant(Lfincsrc::Fake)
    }
    #[doc = "Using override value from LFINCOVR.LFINC"]
    #[inline(always)]
    pub fn override_(self) -> &'a mut crate::W<REG> {
        self.variant(Lfincsrc::Override)
    }
    #[doc = "Using filtered / average value. This value is updated by hardware and can be read and updated in LFINCCTL.INT."]
    #[inline(always)]
    pub fn avg(self) -> &'a mut crate::W<REG> {
        self.variant(Lfincsrc::Avg)
    }
    #[doc = "Using measured value. This value is updated by hardware and can be read from LFINC."]
    #[inline(always)]
    pub fn meas(self) -> &'a mut crate::W<REG> {
        self.variant(Lfincsrc::Meas)
    }
}
#[doc = "24:24\\]
Source of LFTICK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lfticksrc {
    #[doc = "1: LFTICK generated from CLKULL (LFCLK not available)"]
    Fake = 1,
    #[doc = "0: LFTICK generated from the selected LFCLK"]
    Lfclk = 0,
}
impl From<Lfticksrc> for bool {
    #[inline(always)]
    fn from(variant: Lfticksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LFTICKSRC` reader - 24:24\\]
Source of LFTICK."]
pub type LfticksrcR = crate::BitReader<Lfticksrc>;
impl LfticksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lfticksrc {
        match self.bits {
            true => Lfticksrc::Fake,
            false => Lfticksrc::Lfclk,
        }
    }
    #[doc = "LFTICK generated from CLKULL (LFCLK not available)"]
    #[inline(always)]
    pub fn is_fake(&self) -> bool {
        *self == Lfticksrc::Fake
    }
    #[doc = "LFTICK generated from the selected LFCLK"]
    #[inline(always)]
    pub fn is_lfclk(&self) -> bool {
        *self == Lfticksrc::Lfclk
    }
}
#[doc = "Field `LFTICKSRC` writer - 24:24\\]
Source of LFTICK."]
pub type LfticksrcW<'a, REG> = crate::BitWriter<'a, REG, Lfticksrc>;
impl<'a, REG> LfticksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LFTICK generated from CLKULL (LFCLK not available)"]
    #[inline(always)]
    pub fn fake(self) -> &'a mut crate::W<REG> {
        self.variant(Lfticksrc::Fake)
    }
    #[doc = "LFTICK generated from the selected LFCLK"]
    #[inline(always)]
    pub fn lfclk(self) -> &'a mut crate::W<REG> {
        self.variant(Lfticksrc::Lfclk)
    }
}
#[doc = "Field `FLTSETTLED` reader - 25:25\\]
LFINC filter is running and settled."]
pub type FltsettledR = crate::BitReader;
#[doc = "Field `FLTSETTLED` writer - 25:25\\]
LFINC filter is running and settled."]
pub type FltsettledW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED26` reader - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26R = crate::FieldReader;
#[doc = "Field `RESERVED26` writer - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved26W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `GOOD` reader - 31:31\\]
Low frequency clock good Note: This is only a coarse frequency check based on LFQUALCTL. The clock may not be accurate enough for timing purposes."]
pub type GoodR = crate::BitReader;
#[doc = "Field `GOOD` writer - 31:31\\]
Low frequency clock good Note: This is only a coarse frequency check based on LFQUALCTL. The clock may not be accurate enough for timing purposes."]
pub type GoodW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - 21:0\\]
Measured value of LFINC. Given in microseconds with 16 fractional bits. This value is calculated by Hardware. It is the LFCLK period according to CLKULL cycles."]
    #[inline(always)]
    pub fn lfinc(&self) -> LfincR {
        LfincR::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Source of LFINC used by the RTC. This value depends on LFINCOVR.OVERRIDE, LF clock availability, HF tracking loop status and the device state (ACTIVE/STANDBY)."]
    #[inline(always)]
    pub fn lfincsrc(&self) -> LfincsrcR {
        LfincsrcR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Source of LFTICK."]
    #[inline(always)]
    pub fn lfticksrc(&self) -> LfticksrcR {
        LfticksrcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
LFINC filter is running and settled."]
    #[inline(always)]
    pub fn fltsettled(&self) -> FltsettledR {
        FltsettledR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved26(&self) -> Reserved26R {
        Reserved26R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Low frequency clock good Note: This is only a coarse frequency check based on LFQUALCTL. The clock may not be accurate enough for timing purposes."]
    #[inline(always)]
    pub fn good(&self) -> GoodR {
        GoodR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:21 - 21:0\\]
Measured value of LFINC. Given in microseconds with 16 fractional bits. This value is calculated by Hardware. It is the LFCLK period according to CLKULL cycles."]
    #[inline(always)]
    #[must_use]
    pub fn lfinc(&mut self) -> LfincW<LfclkstatSpec> {
        LfincW::new(self, 0)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Source of LFINC used by the RTC. This value depends on LFINCOVR.OVERRIDE, LF clock availability, HF tracking loop status and the device state (ACTIVE/STANDBY)."]
    #[inline(always)]
    #[must_use]
    pub fn lfincsrc(&mut self) -> LfincsrcW<LfclkstatSpec> {
        LfincsrcW::new(self, 22)
    }
    #[doc = "Bit 24 - 24:24\\]
Source of LFTICK."]
    #[inline(always)]
    #[must_use]
    pub fn lfticksrc(&mut self) -> LfticksrcW<LfclkstatSpec> {
        LfticksrcW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
LFINC filter is running and settled."]
    #[inline(always)]
    #[must_use]
    pub fn fltsettled(&mut self) -> FltsettledW<LfclkstatSpec> {
        FltsettledW::new(self, 25)
    }
    #[doc = "Bits 26:30 - 30:26\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved26(&mut self) -> Reserved26W<LfclkstatSpec> {
        Reserved26W::new(self, 26)
    }
    #[doc = "Bit 31 - 31:31\\]
Low frequency clock good Note: This is only a coarse frequency check based on LFQUALCTL. The clock may not be accurate enough for timing purposes."]
    #[inline(always)]
    #[must_use]
    pub fn good(&mut self) -> GoodW<LfclkstatSpec> {
        GoodW::new(self, 31)
    }
}
#[doc = "Low-frequency clock status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclkstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkstatSpec;
impl crate::RegisterSpec for LfclkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclkstat::R`](R) reader structure"]
impl crate::Readable for LfclkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclkstat::W`](W) writer structure"]
impl crate::Writable for LfclkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFCLKSTAT to value 0"]
impl crate::Resettable for LfclkstatSpec {
    const RESET_VALUE: u32 = 0;
}
