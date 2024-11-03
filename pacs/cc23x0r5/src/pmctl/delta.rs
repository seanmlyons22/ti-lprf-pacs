#[doc = "Register `DELTA` reader"]
pub type R = crate::R<DeltaSpec>;
#[doc = "Register `DELTA` writer"]
pub type W = crate::W<DeltaSpec>;
#[doc = "Field `TIME` reader - 11:0\\]
Delta time. Measured time in us between SWSTMP.SWRDY and HFXT ready. This is a always a positive number, and SLWP is used to determine which event occurred first. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
pub type TimeR = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - 11:0\\]
Delta time. Measured time in us between SWSTMP.SWRDY and HFXT ready. This is a always a positive number, and SLWP is used to determine which event occurred first. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
pub type TimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED12` reader - 29:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 29:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "31:30\\]
Slow part. States which of HFXT ready or SW ready that completed first during wakeup from STANDBY mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Slwp {
    #[doc = "3: No valid measurement available"]
    Invalid3 = 3,
    #[doc = "2: HFXT ready set after SW ready (SWSTMP.SWRDY)"]
    Hfxt = 2,
    #[doc = "1: HFXT ready set before SW ready (SWSTMP.SWRDY)"]
    Svt = 1,
    #[doc = "0: No valid measurement available"]
    Invalid0 = 0,
}
impl From<Slwp> for u8 {
    #[inline(always)]
    fn from(variant: Slwp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Slwp {
    type Ux = u8;
}
impl crate::IsEnum for Slwp {}
#[doc = "Field `SLWP` reader - 31:30\\]
Slow part. States which of HFXT ready or SW ready that completed first during wakeup from STANDBY mode."]
pub type SlwpR = crate::FieldReader<Slwp>;
impl SlwpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slwp {
        match self.bits {
            3 => Slwp::Invalid3,
            2 => Slwp::Hfxt,
            1 => Slwp::Svt,
            0 => Slwp::Invalid0,
            _ => unreachable!(),
        }
    }
    #[doc = "No valid measurement available"]
    #[inline(always)]
    pub fn is_invalid3(&self) -> bool {
        *self == Slwp::Invalid3
    }
    #[doc = "HFXT ready set after SW ready (SWSTMP.SWRDY)"]
    #[inline(always)]
    pub fn is_hfxt(&self) -> bool {
        *self == Slwp::Hfxt
    }
    #[doc = "HFXT ready set before SW ready (SWSTMP.SWRDY)"]
    #[inline(always)]
    pub fn is_svt(&self) -> bool {
        *self == Slwp::Svt
    }
    #[doc = "No valid measurement available"]
    #[inline(always)]
    pub fn is_invalid0(&self) -> bool {
        *self == Slwp::Invalid0
    }
}
#[doc = "Field `SLWP` writer - 31:30\\]
Slow part. States which of HFXT ready or SW ready that completed first during wakeup from STANDBY mode."]
pub type SlwpW<'a, REG> = crate::FieldWriter<'a, REG, 2, Slwp, crate::Safe>;
impl<'a, REG> SlwpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No valid measurement available"]
    #[inline(always)]
    pub fn invalid3(self) -> &'a mut crate::W<REG> {
        self.variant(Slwp::Invalid3)
    }
    #[doc = "HFXT ready set after SW ready (SWSTMP.SWRDY)"]
    #[inline(always)]
    pub fn hfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Slwp::Hfxt)
    }
    #[doc = "HFXT ready set before SW ready (SWSTMP.SWRDY)"]
    #[inline(always)]
    pub fn svt(self) -> &'a mut crate::W<REG> {
        self.variant(Slwp::Svt)
    }
    #[doc = "No valid measurement available"]
    #[inline(always)]
    pub fn invalid0(self) -> &'a mut crate::W<REG> {
        self.variant(Slwp::Invalid0)
    }
}
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Delta time. Measured time in us between SWSTMP.SWRDY and HFXT ready. This is a always a positive number, and SLWP is used to determine which event occurred first. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
    #[inline(always)]
    pub fn time(&self) -> TimeR {
        TimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x0003_ffff)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Slow part. States which of HFXT ready or SW ready that completed first during wakeup from STANDBY mode."]
    #[inline(always)]
    pub fn slwp(&self) -> SlwpR {
        SlwpR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Delta time. Measured time in us between SWSTMP.SWRDY and HFXT ready. This is a always a positive number, and SLWP is used to determine which event occurred first. Measurement is enabled when PREPUCTL.WUTIMEN is set."]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TimeW<DeltaSpec> {
        TimeW::new(self, 0)
    }
    #[doc = "Bits 12:29 - 29:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<DeltaSpec> {
        Reserved12W::new(self, 12)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Slow part. States which of HFXT ready or SW ready that completed first during wakeup from STANDBY mode."]
    #[inline(always)]
    #[must_use]
    pub fn slwp(&mut self) -> SlwpW<DeltaSpec> {
        SlwpW::new(self, 30)
    }
}
#[doc = "Delta Time Register. This register contains the measured delta time during wakeup from STANDBY mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delta::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delta::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeltaSpec;
impl crate::RegisterSpec for DeltaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delta::R`](R) reader structure"]
impl crate::Readable for DeltaSpec {}
#[doc = "`write(|w| ..)` method takes [`delta::W`](W) writer structure"]
impl crate::Writable for DeltaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DELTA to value 0"]
impl crate::Resettable for DeltaSpec {
    const RESET_VALUE: u32 = 0;
}
