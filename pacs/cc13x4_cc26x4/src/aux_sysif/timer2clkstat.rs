#[doc = "Register `TIMER2CLKSTAT` reader"]
pub type R = crate::R<Timer2clkstatSpec>;
#[doc = "Register `TIMER2CLKSTAT` writer"]
pub type W = crate::W<Timer2clkstatSpec>;
#[doc = "2:0\\]
AUX_TIMER2 clock source status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stat {
    #[doc = "4: SCLK_HF / 2"]
    SclkHfdiv2 = 4,
    #[doc = "2: SCLK_MF"]
    SclkMf = 2,
    #[doc = "1: SCLK_LF"]
    SclkLf = 1,
    #[doc = "0: No clock"]
    None = 0,
}
impl From<Stat> for u8 {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stat {
    type Ux = u8;
}
impl crate::IsEnum for Stat {}
#[doc = "Field `STAT` reader - 2:0\\]
AUX_TIMER2 clock source status."]
pub type StatR = crate::FieldReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stat> {
        match self.bits {
            4 => Some(Stat::SclkHfdiv2),
            2 => Some(Stat::SclkMf),
            1 => Some(Stat::SclkLf),
            0 => Some(Stat::None),
            _ => None,
        }
    }
    #[doc = "SCLK_HF / 2"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == Stat::SclkHfdiv2
    }
    #[doc = "SCLK_MF"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == Stat::SclkMf
    }
    #[doc = "SCLK_LF"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == Stat::SclkLf
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Stat::None
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
AUX_TIMER2 clock source status."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "AUX_TIMER2 Clock Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2clkstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2clkstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2clkstatSpec;
impl crate::RegisterSpec for Timer2clkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2clkstat::R`](R) reader structure"]
impl crate::Readable for Timer2clkstatSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2clkstat::W`](W) writer structure"]
impl crate::Writable for Timer2clkstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2CLKSTAT to value 0"]
impl crate::Resettable for Timer2clkstatSpec {
    const RESET_VALUE: u32 = 0;
}
