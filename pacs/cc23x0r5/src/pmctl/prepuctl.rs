#[doc = "Register `PREPUCTL` reader"]
pub type R = crate::R<PrepuctlSpec>;
#[doc = "Register `PREPUCTL` writer"]
pub type W = crate::W<PrepuctlSpec>;
#[doc = "7:0\\]
Nominal pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the nominal time in advance of a RTC event. Nominal value is used if no temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nom {
    #[doc = "254: Highest possible value"]
    Max = 254,
    #[doc = "0: Smallest value"]
    Min = 0,
}
impl From<Nom> for u8 {
    #[inline(always)]
    fn from(variant: Nom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nom {
    type Ux = u8;
}
impl crate::IsEnum for Nom {}
#[doc = "Field `NOM` reader - 7:0\\]
Nominal pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the nominal time in advance of a RTC event. Nominal value is used if no temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
pub type NomR = crate::FieldReader<Nom>;
impl NomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nom> {
        match self.bits {
            254 => Some(Nom::Max),
            0 => Some(Nom::Min),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Nom::Max
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Nom::Min
    }
}
#[doc = "Field `NOM` writer - 7:0\\]
Nominal pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the nominal time in advance of a RTC event. Nominal value is used if no temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
pub type NomW<'a, REG> = crate::FieldWriter<'a, REG, 8, Nom>;
impl<'a, REG> NomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Nom::Max)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Nom::Min)
    }
}
#[doc = "15:8\\]
Conservative pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the conservative time in advance of a RTC event. Conservative value is used if a temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cons {
    #[doc = "254: Highest possible value"]
    Max = 254,
    #[doc = "0: Smallest value"]
    Min = 0,
}
impl From<Cons> for u8 {
    #[inline(always)]
    fn from(variant: Cons) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cons {
    type Ux = u8;
}
impl crate::IsEnum for Cons {}
#[doc = "Field `CONS` reader - 15:8\\]
Conservative pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the conservative time in advance of a RTC event. Conservative value is used if a temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
pub type ConsR = crate::FieldReader<Cons>;
impl ConsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cons> {
        match self.bits {
            254 => Some(Cons::Max),
            0 => Some(Cons::Min),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Cons::Max
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Cons::Min
    }
}
#[doc = "Field `CONS` writer - 15:8\\]
Conservative pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the conservative time in advance of a RTC event. Conservative value is used if a temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
pub type ConsW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cons>;
impl<'a, REG> ConsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn max(self) -> &'a mut crate::W<REG> {
        self.variant(Cons::Max)
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn min(self) -> &'a mut crate::W<REG> {
        self.variant(Cons::Min)
    }
}
#[doc = "Field `RESERVED16` reader - 29:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 29:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "30:30\\]
Wakeup time measurement enable. When set will enable WUTIME.DIGWU, WUTIME.HFXTWU and DELTA.TIME time measurements.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wutimen {
    #[doc = "1: Enable wakeup time measurement"]
    En = 1,
    #[doc = "0: Disable wakeup time measurement"]
    Dis = 0,
}
impl From<Wutimen> for bool {
    #[inline(always)]
    fn from(variant: Wutimen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUTIMEN` reader - 30:30\\]
Wakeup time measurement enable. When set will enable WUTIME.DIGWU, WUTIME.HFXTWU and DELTA.TIME time measurements."]
pub type WutimenR = crate::BitReader<Wutimen>;
impl WutimenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wutimen {
        match self.bits {
            true => Wutimen::En,
            false => Wutimen::Dis,
        }
    }
    #[doc = "Enable wakeup time measurement"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Wutimen::En
    }
    #[doc = "Disable wakeup time measurement"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Wutimen::Dis
    }
}
#[doc = "Field `WUTIMEN` writer - 30:30\\]
Wakeup time measurement enable. When set will enable WUTIME.DIGWU, WUTIME.HFXTWU and DELTA.TIME time measurements."]
pub type WutimenW<'a, REG> = crate::BitWriter<'a, REG, Wutimen>;
impl<'a, REG> WutimenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable wakeup time measurement"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Wutimen::En)
    }
    #[doc = "Disable wakeup time measurement"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Wutimen::Dis)
    }
}
#[doc = "31:31\\]
Pre PowerUp Enable. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This to start HFXT settling earlier, so that HFXT can be ready when SW is ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prepuen {
    #[doc = "1: Enable pre-powerup"]
    En = 1,
    #[doc = "0: Disable pre-powerup"]
    Dis = 0,
}
impl From<Prepuen> for bool {
    #[inline(always)]
    fn from(variant: Prepuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREPUEN` reader - 31:31\\]
Pre PowerUp Enable. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This to start HFXT settling earlier, so that HFXT can be ready when SW is ready."]
pub type PrepuenR = crate::BitReader<Prepuen>;
impl PrepuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prepuen {
        match self.bits {
            true => Prepuen::En,
            false => Prepuen::Dis,
        }
    }
    #[doc = "Enable pre-powerup"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Prepuen::En
    }
    #[doc = "Disable pre-powerup"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Prepuen::Dis
    }
}
#[doc = "Field `PREPUEN` writer - 31:31\\]
Pre PowerUp Enable. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This to start HFXT settling earlier, so that HFXT can be ready when SW is ready."]
pub type PrepuenW<'a, REG> = crate::BitWriter<'a, REG, Prepuen>;
impl<'a, REG> PrepuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable pre-powerup"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Prepuen::En)
    }
    #[doc = "Disable pre-powerup"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Prepuen::Dis)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Nominal pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the nominal time in advance of a RTC event. Nominal value is used if no temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
    #[inline(always)]
    pub fn nom(&self) -> NomR {
        NomR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Conservative pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the conservative time in advance of a RTC event. Conservative value is used if a temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
    #[inline(always)]
    pub fn cons(&self) -> ConsR {
        ConsR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 30 - 30:30\\]
Wakeup time measurement enable. When set will enable WUTIME.DIGWU, WUTIME.HFXTWU and DELTA.TIME time measurements."]
    #[inline(always)]
    pub fn wutimen(&self) -> WutimenR {
        WutimenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Pre PowerUp Enable. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This to start HFXT settling earlier, so that HFXT can be ready when SW is ready."]
    #[inline(always)]
    pub fn prepuen(&self) -> PrepuenR {
        PrepuenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Nominal pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the nominal time in advance of a RTC event. Nominal value is used if no temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
    #[inline(always)]
    #[must_use]
    pub fn nom(&mut self) -> NomW<PrepuctlSpec> {
        NomW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Conservative pre-wakeup time. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This field will give the conservative time in advance of a RTC event. Conservative value is used if a temperature change has been detected since STANDBY mode was entered. The time unit for the value is 8us."]
    #[inline(always)]
    #[must_use]
    pub fn cons(&mut self) -> ConsW<PrepuctlSpec> {
        ConsW::new(self, 8)
    }
    #[doc = "Bits 16:29 - 29:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<PrepuctlSpec> {
        Reserved16W::new(self, 16)
    }
    #[doc = "Bit 30 - 30:30\\]
Wakeup time measurement enable. When set will enable WUTIME.DIGWU, WUTIME.HFXTWU and DELTA.TIME time measurements."]
    #[inline(always)]
    #[must_use]
    pub fn wutimen(&mut self) -> WutimenW<PrepuctlSpec> {
        WutimenW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Pre PowerUp Enable. When PREPUEN is set the device will start the wakeup process in advance of the RTC wakeup event. This to start HFXT settling earlier, so that HFXT can be ready when SW is ready."]
    #[inline(always)]
    #[must_use]
    pub fn prepuen(&mut self) -> PrepuenW<PrepuctlSpec> {
        PrepuenW::new(self, 31)
    }
}
#[doc = "Pre Power-Up Control Register. This register contains settings and control for pre-powerup, STANDBY and wakeup measurements.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prepuctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prepuctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrepuctlSpec;
impl crate::RegisterSpec for PrepuctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prepuctl::R`](R) reader structure"]
impl crate::Readable for PrepuctlSpec {}
#[doc = "`write(|w| ..)` method takes [`prepuctl::W`](W) writer structure"]
impl crate::Writable for PrepuctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREPUCTL to value 0"]
impl crate::Resettable for PrepuctlSpec {
    const RESET_VALUE: u32 = 0;
}
