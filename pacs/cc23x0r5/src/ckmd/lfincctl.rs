#[doc = "Register `LFINCCTL` reader"]
pub type R = crate::R<LfincctlSpec>;
#[doc = "Register `LFINCCTL` writer"]
pub type W = crate::W<LfincctlSpec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "2:2\\]
Use a higher gear after re-enabling / wakeup. The filter will require 16-24 LFCLK periods to settle (depending on STOPGEAR), but may respond faster to frequency changes during STANDBY.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softrstrt {
    #[doc = "1: Use soft gearing restarts"]
    On = 1,
    #[doc = "0: Don't use soft gearing restarts"]
    Off = 0,
}
impl From<Softrstrt> for bool {
    #[inline(always)]
    fn from(variant: Softrstrt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTRSTRT` reader - 2:2\\]
Use a higher gear after re-enabling / wakeup. The filter will require 16-24 LFCLK periods to settle (depending on STOPGEAR), but may respond faster to frequency changes during STANDBY."]
pub type SoftrstrtR = crate::BitReader<Softrstrt>;
impl SoftrstrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softrstrt {
        match self.bits {
            true => Softrstrt::On,
            false => Softrstrt::Off,
        }
    }
    #[doc = "Use soft gearing restarts"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Softrstrt::On
    }
    #[doc = "Don't use soft gearing restarts"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Softrstrt::Off
    }
}
#[doc = "Field `SOFTRSTRT` writer - 2:2\\]
Use a higher gear after re-enabling / wakeup. The filter will require 16-24 LFCLK periods to settle (depending on STOPGEAR), but may respond faster to frequency changes during STANDBY."]
pub type SoftrstrtW<'a, REG> = crate::BitWriter<'a, REG, Softrstrt>;
impl<'a, REG> SoftrstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use soft gearing restarts"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Softrstrt::On)
    }
    #[doc = "Don't use soft gearing restarts"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Softrstrt::Off)
    }
}
#[doc = "4:3\\]
Controls gearing restart of the LFINC filter.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gearrstrt {
    #[doc = "2: Restart gearing when the error accumulator crosses the threshold twice in a row."]
    Twothr = 2,
    #[doc = "1: Restart gearing when the error accumulator crosses the threshold once."]
    Onethr = 1,
    #[doc = "0: Never restart gearing. Very stable filter value, but very slow response on frequency changes."]
    Never = 0,
}
impl From<Gearrstrt> for u8 {
    #[inline(always)]
    fn from(variant: Gearrstrt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gearrstrt {
    type Ux = u8;
}
impl crate::IsEnum for Gearrstrt {}
#[doc = "Field `GEARRSTRT` reader - 4:3\\]
Controls gearing restart of the LFINC filter."]
pub type GearrstrtR = crate::FieldReader<Gearrstrt>;
impl GearrstrtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gearrstrt> {
        match self.bits {
            2 => Some(Gearrstrt::Twothr),
            1 => Some(Gearrstrt::Onethr),
            0 => Some(Gearrstrt::Never),
            _ => None,
        }
    }
    #[doc = "Restart gearing when the error accumulator crosses the threshold twice in a row."]
    #[inline(always)]
    pub fn is_twothr(&self) -> bool {
        *self == Gearrstrt::Twothr
    }
    #[doc = "Restart gearing when the error accumulator crosses the threshold once."]
    #[inline(always)]
    pub fn is_onethr(&self) -> bool {
        *self == Gearrstrt::Onethr
    }
    #[doc = "Never restart gearing. Very stable filter value, but very slow response on frequency changes."]
    #[inline(always)]
    pub fn is_never(&self) -> bool {
        *self == Gearrstrt::Never
    }
}
#[doc = "Field `GEARRSTRT` writer - 4:3\\]
Controls gearing restart of the LFINC filter."]
pub type GearrstrtW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gearrstrt>;
impl<'a, REG> GearrstrtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restart gearing when the error accumulator crosses the threshold twice in a row."]
    #[inline(always)]
    pub fn twothr(self) -> &'a mut crate::W<REG> {
        self.variant(Gearrstrt::Twothr)
    }
    #[doc = "Restart gearing when the error accumulator crosses the threshold once."]
    #[inline(always)]
    pub fn onethr(self) -> &'a mut crate::W<REG> {
        self.variant(Gearrstrt::Onethr)
    }
    #[doc = "Never restart gearing. Very stable filter value, but very slow response on frequency changes."]
    #[inline(always)]
    pub fn never(self) -> &'a mut crate::W<REG> {
        self.variant(Gearrstrt::Never)
    }
}
#[doc = "6:5\\]
Controls the threshold for gearing restart of the LFINC filter. Only effective if GEARRSTRT is not ONETHR or TWOTHR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Errthr {
    #[doc = "3: Restart gearing on small error. Potentially more false restarts, faster response on small frequency shifts."]
    Small = 3,
    #[doc = "2: Middle value towards SMALL."]
    Midsmall = 2,
    #[doc = "1: Middle value towards LARGE."]
    Midlarge = 1,
    #[doc = "0: Restart gearing on large error. Fewer false restarts, slower response on small frequency shifts."]
    Large = 0,
}
impl From<Errthr> for u8 {
    #[inline(always)]
    fn from(variant: Errthr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Errthr {
    type Ux = u8;
}
impl crate::IsEnum for Errthr {}
#[doc = "Field `ERRTHR` reader - 6:5\\]
Controls the threshold for gearing restart of the LFINC filter. Only effective if GEARRSTRT is not ONETHR or TWOTHR."]
pub type ErrthrR = crate::FieldReader<Errthr>;
impl ErrthrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errthr {
        match self.bits {
            3 => Errthr::Small,
            2 => Errthr::Midsmall,
            1 => Errthr::Midlarge,
            0 => Errthr::Large,
            _ => unreachable!(),
        }
    }
    #[doc = "Restart gearing on small error. Potentially more false restarts, faster response on small frequency shifts."]
    #[inline(always)]
    pub fn is_small(&self) -> bool {
        *self == Errthr::Small
    }
    #[doc = "Middle value towards SMALL."]
    #[inline(always)]
    pub fn is_midsmall(&self) -> bool {
        *self == Errthr::Midsmall
    }
    #[doc = "Middle value towards LARGE."]
    #[inline(always)]
    pub fn is_midlarge(&self) -> bool {
        *self == Errthr::Midlarge
    }
    #[doc = "Restart gearing on large error. Fewer false restarts, slower response on small frequency shifts."]
    #[inline(always)]
    pub fn is_large(&self) -> bool {
        *self == Errthr::Large
    }
}
#[doc = "Field `ERRTHR` writer - 6:5\\]
Controls the threshold for gearing restart of the LFINC filter. Only effective if GEARRSTRT is not ONETHR or TWOTHR."]
pub type ErrthrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Errthr, crate::Safe>;
impl<'a, REG> ErrthrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restart gearing on small error. Potentially more false restarts, faster response on small frequency shifts."]
    #[inline(always)]
    pub fn small(self) -> &'a mut crate::W<REG> {
        self.variant(Errthr::Small)
    }
    #[doc = "Middle value towards SMALL."]
    #[inline(always)]
    pub fn midsmall(self) -> &'a mut crate::W<REG> {
        self.variant(Errthr::Midsmall)
    }
    #[doc = "Middle value towards LARGE."]
    #[inline(always)]
    pub fn midlarge(self) -> &'a mut crate::W<REG> {
        self.variant(Errthr::Midlarge)
    }
    #[doc = "Restart gearing on large error. Fewer false restarts, slower response on small frequency shifts."]
    #[inline(always)]
    pub fn large(self) -> &'a mut crate::W<REG> {
        self.variant(Errthr::Large)
    }
}
#[doc = "7:7\\]
Controls the final gear of the LFINC filter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopgear {
    #[doc = "1: Highest final gear. Best dynamic frequency tracking, but higher variation in filter value."]
    High = 1,
    #[doc = "0: Lowest final gear. Best settling, but less dynamic frequency tracking."]
    Low = 0,
}
impl From<Stopgear> for bool {
    #[inline(always)]
    fn from(variant: Stopgear) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPGEAR` reader - 7:7\\]
Controls the final gear of the LFINC filter."]
pub type StopgearR = crate::BitReader<Stopgear>;
impl StopgearR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopgear {
        match self.bits {
            true => Stopgear::High,
            false => Stopgear::Low,
        }
    }
    #[doc = "Highest final gear. Best dynamic frequency tracking, but higher variation in filter value."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Stopgear::High
    }
    #[doc = "Lowest final gear. Best settling, but less dynamic frequency tracking."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Stopgear::Low
    }
}
#[doc = "Field `STOPGEAR` writer - 7:7\\]
Controls the final gear of the LFINC filter."]
pub type StopgearW<'a, REG> = crate::BitWriter<'a, REG, Stopgear>;
impl<'a, REG> StopgearW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Highest final gear. Best dynamic frequency tracking, but higher variation in filter value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Stopgear::High)
    }
    #[doc = "Lowest final gear. Best settling, but less dynamic frequency tracking."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Stopgear::Low)
    }
}
#[doc = "Field `INT` reader - 29:8\\]
Integral part of the LFINC filter. This value is updated by Hardware to reflect the current state of the filter. It can also be written to change the current state."]
pub type IntR = crate::FieldReader<u32>;
#[doc = "Field `INT` writer - 29:8\\]
Integral part of the LFINC filter. This value is updated by Hardware to reflect the current state of the filter. It can also be written to change the current state."]
pub type IntW<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `RESERVED30` reader - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30R = crate::BitReader;
#[doc = "Field `RESERVED30` writer - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "31:31\\]
Controls if the LFINC filter prevents STANBY entry until settled.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Preventstby {
    #[doc = "1: Enable. Prevent STANDBY entry."]
    On = 1,
    #[doc = "0: Disable. Do not prevent STANDBY entry."]
    Off = 0,
}
impl From<Preventstby> for bool {
    #[inline(always)]
    fn from(variant: Preventstby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREVENTSTBY` reader - 31:31\\]
Controls if the LFINC filter prevents STANBY entry until settled."]
pub type PreventstbyR = crate::BitReader<Preventstby>;
impl PreventstbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Preventstby {
        match self.bits {
            true => Preventstby::On,
            false => Preventstby::Off,
        }
    }
    #[doc = "Enable. Prevent STANDBY entry."]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Preventstby::On
    }
    #[doc = "Disable. Do not prevent STANDBY entry."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Preventstby::Off
    }
}
#[doc = "Field `PREVENTSTBY` writer - 31:31\\]
Controls if the LFINC filter prevents STANBY entry until settled."]
pub type PreventstbyW<'a, REG> = crate::BitWriter<'a, REG, Preventstby>;
impl<'a, REG> PreventstbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable. Prevent STANDBY entry."]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Preventstby::On)
    }
    #[doc = "Disable. Do not prevent STANDBY entry."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Preventstby::Off)
    }
}
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Use a higher gear after re-enabling / wakeup. The filter will require 16-24 LFCLK periods to settle (depending on STOPGEAR), but may respond faster to frequency changes during STANDBY."]
    #[inline(always)]
    pub fn softrstrt(&self) -> SoftrstrtR {
        SoftrstrtR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Controls gearing restart of the LFINC filter."]
    #[inline(always)]
    pub fn gearrstrt(&self) -> GearrstrtR {
        GearrstrtR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Controls the threshold for gearing restart of the LFINC filter. Only effective if GEARRSTRT is not ONETHR or TWOTHR."]
    #[inline(always)]
    pub fn errthr(&self) -> ErrthrR {
        ErrthrR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls the final gear of the LFINC filter."]
    #[inline(always)]
    pub fn stopgear(&self) -> StopgearR {
        StopgearR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:29 - 29:8\\]
Integral part of the LFINC filter. This value is updated by Hardware to reflect the current state of the filter. It can also be written to change the current state."]
    #[inline(always)]
    pub fn int(&self) -> IntR {
        IntR::new((self.bits >> 8) & 0x003f_ffff)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls if the LFINC filter prevents STANBY entry until settled."]
    #[inline(always)]
    pub fn preventstby(&self) -> PreventstbyR {
        PreventstbyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<LfincctlSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Use a higher gear after re-enabling / wakeup. The filter will require 16-24 LFCLK periods to settle (depending on STOPGEAR), but may respond faster to frequency changes during STANDBY."]
    #[inline(always)]
    #[must_use]
    pub fn softrstrt(&mut self) -> SoftrstrtW<LfincctlSpec> {
        SoftrstrtW::new(self, 2)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Controls gearing restart of the LFINC filter."]
    #[inline(always)]
    #[must_use]
    pub fn gearrstrt(&mut self) -> GearrstrtW<LfincctlSpec> {
        GearrstrtW::new(self, 3)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Controls the threshold for gearing restart of the LFINC filter. Only effective if GEARRSTRT is not ONETHR or TWOTHR."]
    #[inline(always)]
    #[must_use]
    pub fn errthr(&mut self) -> ErrthrW<LfincctlSpec> {
        ErrthrW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Controls the final gear of the LFINC filter."]
    #[inline(always)]
    #[must_use]
    pub fn stopgear(&mut self) -> StopgearW<LfincctlSpec> {
        StopgearW::new(self, 7)
    }
    #[doc = "Bits 8:29 - 29:8\\]
Integral part of the LFINC filter. This value is updated by Hardware to reflect the current state of the filter. It can also be written to change the current state."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> IntW<LfincctlSpec> {
        IntW::new(self, 8)
    }
    #[doc = "Bit 30 - 30:30\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved30(&mut self) -> Reserved30W<LfincctlSpec> {
        Reserved30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Controls if the LFINC filter prevents STANBY entry until settled."]
    #[inline(always)]
    #[must_use]
    pub fn preventstby(&mut self) -> PreventstbyW<LfincctlSpec> {
        PreventstbyW::new(self, 31)
    }
}
#[doc = "Low frequency time increment control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfincctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfincctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfincctlSpec;
impl crate::RegisterSpec for LfincctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfincctl::R`](R) reader structure"]
impl crate::Readable for LfincctlSpec {}
#[doc = "`write(|w| ..)` method takes [`lfincctl::W`](W) writer structure"]
impl crate::Writable for LfincctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LFINCCTL to value 0x9e84_8014"]
impl crate::Resettable for LfincctlSpec {
    const RESET_VALUE: u32 = 0x9e84_8014;
}
