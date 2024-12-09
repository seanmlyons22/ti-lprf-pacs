#[doc = "Register `AUXSCECLK` reader"]
pub type R = crate::R<AuxsceclkSpec>;
#[doc = "Register `AUXSCECLK` writer"]
pub type W = crate::W<AuxsceclkSpec>;
#[doc = "0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Src {
    #[doc = "1: MF Clock (SCLK_MF)"]
    SclkMf = 1,
    #[doc = "0: HF Clock divided by 2 (SCLK_HFDIV2)"]
    SclkHfdiv2 = 0,
}
impl From<Src> for bool {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type SrcR = crate::BitReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            true => Src::SclkMf,
            false => Src::SclkHfdiv2,
        }
    }
    #[doc = "MF Clock (SCLK_MF)"]
    #[inline(always)]
    pub fn is_sclk_mf(&self) -> bool {
        *self == Src::SclkMf
    }
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    #[inline(always)]
    pub fn is_sclk_hfdiv2(&self) -> bool {
        *self == Src::SclkHfdiv2
    }
}
#[doc = "Field `SRC` writer - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MF Clock (SCLK_MF)"]
    #[inline(always)]
    pub fn sclk_mf(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkMf)
    }
    #[doc = "HF Clock divided by 2 (SCLK_HFDIV2)"]
    #[inline(always)]
    pub fn sclk_hfdiv2(self) -> &'a mut crate::W<REG> {
        self.variant(Src::SclkHfdiv2)
    }
}
#[doc = "Field `RESERVED3` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdSrc {
    #[doc = "1: LF clock (SCLK_LF )"]
    SclkLf = 1,
    #[doc = "0: No clock"]
    NoClock = 0,
}
impl From<PdSrc> for bool {
    #[inline(always)]
    fn from(variant: PdSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PD_SRC` reader - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type PdSrcR = crate::BitReader<PdSrc>;
impl PdSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdSrc {
        match self.bits {
            true => PdSrc::SclkLf,
            false => PdSrc::NoClock,
        }
    }
    #[doc = "LF clock (SCLK_LF )"]
    #[inline(always)]
    pub fn is_sclk_lf(&self) -> bool {
        *self == PdSrc::SclkLf
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == PdSrc::NoClock
    }
}
#[doc = "Field `PD_SRC` writer - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
pub type PdSrcW<'a, REG> = crate::BitWriter<'a, REG, PdSrc>;
impl<'a, REG> PdSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LF clock (SCLK_LF )"]
    #[inline(always)]
    pub fn sclk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(PdSrc::SclkLf)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut crate::W<REG> {
        self.variant(PdSrc::NoClock)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    pub fn pd_src(&self) -> PdSrcR {
        PdSrcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Selects the clock source for the AUX domain when AUX is in active mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<AuxsceclkSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Selects the clock source for the AUX domain when AUX is in powerdown mode. Note: Switching the clock source is guaranteed to be glitch-free"]
    #[inline(always)]
    #[must_use]
    pub fn pd_src(&mut self) -> PdSrcW<AuxsceclkSpec> {
        PdSrcW::new(self, 8)
    }
}
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxsceclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxsceclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuxsceclkSpec;
impl crate::RegisterSpec for AuxsceclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxsceclk::R`](R) reader structure"]
impl crate::Readable for AuxsceclkSpec {}
#[doc = "`write(|w| ..)` method takes [`auxsceclk::W`](W) writer structure"]
impl crate::Writable for AuxsceclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXSCECLK to value 0"]
impl crate::Resettable for AuxsceclkSpec {
    const RESET_VALUE: u32 = 0;
}
