#[doc = "Register `ADCCLKSEL` reader"]
pub type R = crate::R<AdcclkselSpec>;
#[doc = "Register `ADCCLKSEL` writer"]
pub type W = crate::W<AdcclkselSpec>;
#[doc = "1:0\\]
Select ADC clock source Change only while ADC is disabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "1: 48MHz HFXT"]
    Hfxt = 1,
    #[doc = "0: 48MHz CLKSVT"]
    Clksvt = 0,
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
#[doc = "Field `SRC` reader - 1:0\\]
Select ADC clock source Change only while ADC is disabled."]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Src> {
        match self.bits {
            1 => Some(Src::Hfxt),
            0 => Some(Src::Clksvt),
            _ => None,
        }
    }
    #[doc = "48MHz HFXT"]
    #[inline(always)]
    pub fn is_hfxt(&self) -> bool {
        *self == Src::Hfxt
    }
    #[doc = "48MHz CLKSVT"]
    #[inline(always)]
    pub fn is_clksvt(&self) -> bool {
        *self == Src::Clksvt
    }
}
#[doc = "Field `SRC` writer - 1:0\\]
Select ADC clock source Change only while ADC is disabled."]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48MHz HFXT"]
    #[inline(always)]
    pub fn hfxt(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Hfxt)
    }
    #[doc = "48MHz CLKSVT"]
    #[inline(always)]
    pub fn clksvt(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Clksvt)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Select ADC clock source Change only while ADC is disabled."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Select ADC clock source Change only while ADC is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<AdcclkselSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AdcclkselSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "ADC clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcclkselSpec;
impl crate::RegisterSpec for AdcclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcclksel::R`](R) reader structure"]
impl crate::Readable for AdcclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`adcclksel::W`](W) writer structure"]
impl crate::Writable for AdcclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADCCLKSEL to value 0"]
impl crate::Resettable for AdcclkselSpec {
    const RESET_VALUE: u32 = 0;
}
