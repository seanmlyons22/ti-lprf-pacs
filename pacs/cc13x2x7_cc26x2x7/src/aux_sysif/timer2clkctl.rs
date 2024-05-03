#[doc = "Register `TIMER2CLKCTL` reader"]
pub type R = crate::R<Timer2clkctlSpec>;
#[doc = "Register `TIMER2CLKCTL` writer"]
pub type W = crate::W<Timer2clkctlSpec>;
#[doc = "2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "4: SCLK_HF / 2"]
    SclkHfdiv2 = 4,
    #[doc = "2: SCLK_MF"]
    SclkMf = 2,
    #[doc = "1: SCLK_LF"]
    SclkLf = 1,
    #[doc = "0: no clock"]
    None = 0,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            4 => Some(Src::SclkHfdiv2),
            2 => Some(Src::SclkMf),
            1 => Some(Src::SclkLf),
            0 => Some(Src::None),
            _ => None,
        }
    }
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == Src::SclkHfdiv2
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == Src::SclkMf
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == Src::SclkLf
    }
    #[doc = "no clock"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Src::None
    }
}
#[doc = "Field `SRC` writer - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkHfdiv2)
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkMf)
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkLf)
    }
    #[doc = "no clock"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Src::None)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Select clock source for AUX_TIMER2. Update is only accepted if SRC equals TIMER2CLKSTAT.STAT or TIMER2CLKSWITCH.RDY is 1. It is recommended to select NONE only when TIMER2BRIDGE.BUSY is 0. A non-enumerated value is ignored."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<Timer2clkctlSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Timer2clkctlSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "AUX_TIMER2 Clock Control Access to AUX_TIMER2 is only possible when TIMER2CLKSTAT.STAT is different from NONE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2clkctlSpec;
impl crate::RegisterSpec for Timer2clkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2clkctl::R`](R) reader structure"]
impl crate::Readable for Timer2clkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2clkctl::W`](W) writer structure"]
impl crate::Writable for Timer2clkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2CLKCTL to value 0"]
impl crate::Resettable for Timer2clkctlSpec {
    const RESET_VALUE: u32 = 0;
}
