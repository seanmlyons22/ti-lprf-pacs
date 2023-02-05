#[doc = "Register `HFTRACKCTL` reader"]
pub type R = crate::R<HftrackctlSpec>;
#[doc = "Register `HFTRACKCTL` writer"]
pub type W = crate::W<HftrackctlSpec>;
#[doc = "25:0\\]
Reference clock ratio. RATIO = 24MHz / (2*reference-frequency) * 2^24 Commonly used reference clock frequencies are provided as enumerations.\n\nValue on reset: 4194304"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ratio {
    #[doc = "50331648: Use for 4MHz reference clock"]
    Ref4m = 50331648,
    #[doc = "25165824: Use for 8MHz reference clock"]
    Ref8m = 25165824,
    #[doc = "4194304: Use for 48MHz reference clock"]
    Ref48m = 4194304,
}
impl From<Ratio> for u32 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u32;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - 25:0\\]
Reference clock ratio. RATIO = 24MHz / (2*reference-frequency) * 2^24 Commonly used reference clock frequencies are provided as enumerations."]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ratio> {
        match self.bits {
            50331648 => Some(Ratio::Ref4m),
            25165824 => Some(Ratio::Ref8m),
            4194304 => Some(Ratio::Ref48m),
            _ => None,
        }
    }
    #[doc = "Use for 4MHz reference clock"]
    #[inline(always)]
    pub fn is_ref4m(&self) -> bool {
        *self == Ratio::Ref4m
    }
    #[doc = "Use for 8MHz reference clock"]
    #[inline(always)]
    pub fn is_ref8m(&self) -> bool {
        *self == Ratio::Ref8m
    }
    #[doc = "Use for 48MHz reference clock"]
    #[inline(always)]
    pub fn is_ref48m(&self) -> bool {
        *self == Ratio::Ref48m
    }
}
#[doc = "Field `RATIO` writer - 25:0\\]
Reference clock ratio. RATIO = 24MHz / (2*reference-frequency) * 2^24 Commonly used reference clock frequencies are provided as enumerations."]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 26, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Use for 4MHz reference clock"]
    #[inline(always)]
    pub fn ref4m(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ref4m)
    }
    #[doc = "Use for 8MHz reference clock"]
    #[inline(always)]
    pub fn ref8m(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ref8m)
    }
    #[doc = "Use for 48MHz reference clock"]
    #[inline(always)]
    pub fn ref48m(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Ref48m)
    }
}
#[doc = "27:26\\]
Select the reference clock for the tracking loop. Change only while the tracking loop is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refclk {
    #[doc = "2: Select GPI as reference clock."]
    Gpi = 2,
    #[doc = "1: Select LRF reference clock."]
    Lrf = 1,
    #[doc = "0: Select HFXT as reference clock."]
    Hfxt = 0,
}
impl From<Refclk> for u8 {
    #[inline(always)]
    fn from(variant: Refclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refclk {
    type Ux = u8;
}
impl crate::IsEnum for Refclk {}
#[doc = "Field `REFCLK` reader - 27:26\\]
Select the reference clock for the tracking loop. Change only while the tracking loop is disabled."]
pub type RefclkR = crate::FieldReader<Refclk>;
impl RefclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refclk> {
        match self.bits {
            2 => Some(Refclk::Gpi),
            1 => Some(Refclk::Lrf),
            0 => Some(Refclk::Hfxt),
            _ => None,
        }
    }
    #[doc = "Select GPI as reference clock."]
    #[inline(always)]
    pub fn is_gpi(&self) -> bool {
        *self == Refclk::Gpi
    }
    #[doc = "Select LRF reference clock."]
    #[inline(always)]
    pub fn is_lrf(&self) -> bool {
        *self == Refclk::Lrf
    }
    #[doc = "Select HFXT as reference clock."]
    #[inline(always)]
    pub fn is_hfxt(&self) -> bool {
        *self == Refclk::Hfxt
    }
}
#[doc = "Field `REFCLK` writer - 27:26\\]
Select the reference clock for the tracking loop. Change only while the tracking loop is disabled."]
pub type RefclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refclk>;
impl<'a, REG> RefclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select GPI as reference clock."]
    #[inline(always)]
    pub fn gpi(self) -> &'a mut crate::W<REG> {
        self.variant(Refclk::Gpi)
    }
    #[doc = "Select LRF reference clock."]
    #[inline(always)]
    pub fn lrf(self) -> &'a mut crate::W<REG> {
        self.variant(Refclk::Lrf)
    }
    #[doc = "Select HFXT as reference clock."]
    #[inline(always)]
    pub fn hfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Refclk::Hfxt)
    }
}
#[doc = "Field `RESERVED28` reader - 29:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `DSMBYP` reader - 30:30\\]
Bypass Delta-Sigma-Modulation of fine trim."]
pub type DsmbypR = crate::BitReader;
#[doc = "Field `DSMBYP` writer - 30:30\\]
Bypass Delta-Sigma-Modulation of fine trim."]
pub type DsmbypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - 31:31\\]
Enable tracking loop."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 31:31\\]
Enable tracking loop."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Reference clock ratio. RATIO = 24MHz / (2*reference-frequency) * 2^24 Commonly used reference clock frequencies are provided as enumerations."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Select the reference clock for the tracking loop. Change only while the tracking loop is disabled."]
    #[inline(always)]
    pub fn refclk(&self) -> RefclkR {
        RefclkR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Bypass Delta-Sigma-Modulation of fine trim."]
    #[inline(always)]
    pub fn dsmbyp(&self) -> DsmbypR {
        DsmbypR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable tracking loop."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:25 - 25:0\\]
Reference clock ratio. RATIO = 24MHz / (2*reference-frequency) * 2^24 Commonly used reference clock frequencies are provided as enumerations."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<HftrackctlSpec> {
        RatioW::new(self, 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Select the reference clock for the tracking loop. Change only while the tracking loop is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> RefclkW<HftrackctlSpec> {
        RefclkW::new(self, 26)
    }
    #[doc = "Bit 30 - 30:30\\]
Bypass Delta-Sigma-Modulation of fine trim."]
    #[inline(always)]
    #[must_use]
    pub fn dsmbyp(&mut self) -> DsmbypW<HftrackctlSpec> {
        DsmbypW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Enable tracking loop."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<HftrackctlSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "High frequency tracking loop control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hftrackctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hftrackctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HftrackctlSpec;
impl crate::RegisterSpec for HftrackctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hftrackctl::R`](R) reader structure"]
impl crate::Readable for HftrackctlSpec {}
#[doc = "`write(|w| ..)` method takes [`hftrackctl::W`](W) writer structure"]
impl crate::Writable for HftrackctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFTRACKCTL to value 0x0040_0000"]
impl crate::Resettable for HftrackctlSpec {
    const RESET_VALUE: u32 = 0x0040_0000;
}
