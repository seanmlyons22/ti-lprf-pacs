#[doc = "Register `LFCLKSEL` reader"]
pub type R = crate::R<LfclkselSpec>;
#[doc = "Register `LFCLKSEL` writer"]
pub type W = crate::W<LfclkselSpec>;
#[doc = "1:0\\]
Select the main low frequency clock source. If running, this clock will be used to generate LFTICK and as CLKULL during STANDBY. If not running, LFTICK will be generated from HFOSC and STANDBY entry will be prevented.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Main {
    #[doc = "3: External LF clock through GPI."]
    Extlf = 3,
    #[doc = "2: Low frequency crystal oscillator"]
    Lfxt = 2,
    #[doc = "1: Low frequency on-chip oscillator"]
    Lfosc = 1,
    #[doc = "0: No LF clock selected. LFTICK will be generated from HFOSC, STANDBY entry will be prevented."]
    Fake = 0,
}
impl From<Main> for u8 {
    #[inline(always)]
    fn from(variant: Main) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Main {
    type Ux = u8;
}
impl crate::IsEnum for Main {}
#[doc = "Field `MAIN` reader - 1:0\\]
Select the main low frequency clock source. If running, this clock will be used to generate LFTICK and as CLKULL during STANDBY. If not running, LFTICK will be generated from HFOSC and STANDBY entry will be prevented."]
pub type MainR = crate::FieldReader<Main>;
impl MainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Main {
        match self.bits {
            3 => Main::Extlf,
            2 => Main::Lfxt,
            1 => Main::Lfosc,
            0 => Main::Fake,
            _ => unreachable!(),
        }
    }
    #[doc = "External LF clock through GPI."]
    #[inline(always)]
    pub fn is_extlf(&self) -> bool {
        *self == Main::Extlf
    }
    #[doc = "Low frequency crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == Main::Lfxt
    }
    #[doc = "Low frequency on-chip oscillator"]
    #[inline(always)]
    pub fn is_lfosc(&self) -> bool {
        *self == Main::Lfosc
    }
    #[doc = "No LF clock selected. LFTICK will be generated from HFOSC, STANDBY entry will be prevented."]
    #[inline(always)]
    pub fn is_fake(&self) -> bool {
        *self == Main::Fake
    }
}
#[doc = "Field `MAIN` writer - 1:0\\]
Select the main low frequency clock source. If running, this clock will be used to generate LFTICK and as CLKULL during STANDBY. If not running, LFTICK will be generated from HFOSC and STANDBY entry will be prevented."]
pub type MainW<'a, REG> = crate::FieldWriter<'a, REG, 2, Main, crate::Safe>;
impl<'a, REG> MainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External LF clock through GPI."]
    #[inline(always)]
    pub fn extlf(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Extlf)
    }
    #[doc = "Low frequency crystal oscillator"]
    #[inline(always)]
    pub fn lfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Lfxt)
    }
    #[doc = "Low frequency on-chip oscillator"]
    #[inline(always)]
    pub fn lfosc(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Lfosc)
    }
    #[doc = "No LF clock selected. LFTICK will be generated from HFOSC, STANDBY entry will be prevented."]
    #[inline(always)]
    pub fn fake(self) -> &'a mut crate::W<REG> {
        self.variant(Main::Fake)
    }
}
#[doc = "3:2\\]
Select low frequency clock source for the PRELFCLK interrupt. Can be used by Software to confirm that the clock is running and it's frequency is good, before selecting it in MAIN.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pre {
    #[doc = "3: External LF clock through GPI."]
    Extlf = 3,
    #[doc = "2: Low frequency crystal oscillator"]
    Lfxt = 2,
    #[doc = "1: Low frequency on-chip oscillator"]
    Lfosc = 1,
    #[doc = "0: No clock. Output will be tied low."]
    None = 0,
}
impl From<Pre> for u8 {
    #[inline(always)]
    fn from(variant: Pre) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pre {
    type Ux = u8;
}
impl crate::IsEnum for Pre {}
#[doc = "Field `PRE` reader - 3:2\\]
Select low frequency clock source for the PRELFCLK interrupt. Can be used by Software to confirm that the clock is running and it's frequency is good, before selecting it in MAIN."]
pub type PreR = crate::FieldReader<Pre>;
impl PreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pre {
        match self.bits {
            3 => Pre::Extlf,
            2 => Pre::Lfxt,
            1 => Pre::Lfosc,
            0 => Pre::None,
            _ => unreachable!(),
        }
    }
    #[doc = "External LF clock through GPI."]
    #[inline(always)]
    pub fn is_extlf(&self) -> bool {
        *self == Pre::Extlf
    }
    #[doc = "Low frequency crystal oscillator"]
    #[inline(always)]
    pub fn is_lfxt(&self) -> bool {
        *self == Pre::Lfxt
    }
    #[doc = "Low frequency on-chip oscillator"]
    #[inline(always)]
    pub fn is_lfosc(&self) -> bool {
        *self == Pre::Lfosc
    }
    #[doc = "No clock. Output will be tied low."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pre::None
    }
}
#[doc = "Field `PRE` writer - 3:2\\]
Select low frequency clock source for the PRELFCLK interrupt. Can be used by Software to confirm that the clock is running and it's frequency is good, before selecting it in MAIN."]
pub type PreW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pre, crate::Safe>;
impl<'a, REG> PreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External LF clock through GPI."]
    #[inline(always)]
    pub fn extlf(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::Extlf)
    }
    #[doc = "Low frequency crystal oscillator"]
    #[inline(always)]
    pub fn lfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::Lfxt)
    }
    #[doc = "Low frequency on-chip oscillator"]
    #[inline(always)]
    pub fn lfosc(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::Lfosc)
    }
    #[doc = "No clock. Output will be tied low."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pre::None)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select the main low frequency clock source. If running, this clock will be used to generate LFTICK and as CLKULL during STANDBY. If not running, LFTICK will be generated from HFOSC and STANDBY entry will be prevented."]
    #[inline(always)]
    pub fn main(&self) -> MainR {
        MainR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select low frequency clock source for the PRELFCLK interrupt. Can be used by Software to confirm that the clock is running and it's frequency is good, before selecting it in MAIN."]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select the main low frequency clock source. If running, this clock will be used to generate LFTICK and as CLKULL during STANDBY. If not running, LFTICK will be generated from HFOSC and STANDBY entry will be prevented."]
    #[inline(always)]
    #[must_use]
    pub fn main(&mut self) -> MainW<LfclkselSpec> {
        MainW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Select low frequency clock source for the PRELFCLK interrupt. Can be used by Software to confirm that the clock is running and it's frequency is good, before selecting it in MAIN."]
    #[inline(always)]
    #[must_use]
    pub fn pre(&mut self) -> PreW<LfclkselSpec> {
        PreW::new(self, 2)
    }
}
#[doc = "Low frequency clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfclkselSpec;
impl crate::RegisterSpec for LfclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfclksel::R`](R) reader structure"]
impl crate::Readable for LfclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`lfclksel::W`](W) writer structure"]
impl crate::Writable for LfclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFCLKSEL to value 0"]
impl crate::Resettable for LfclkselSpec {
    const RESET_VALUE: u32 = 0;
}
