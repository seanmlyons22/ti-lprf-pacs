#[doc = "Register `CPUCLKDIV` reader"]
pub type R = crate::R<CpuclkdivSpec>;
#[doc = "Register `CPUCLKDIV` writer"]
pub type W = crate::W<CpuclkdivSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ratio {
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Div1 = 0,
}
impl From<Ratio> for bool {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RATIO` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioR = crate::BitReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ratio> {
        match self.bits {
            false => Some(Ratio::Div1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ratio::Div1
    }
}
#[doc = "Field `RATIO` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioW<'a, REG> = crate::BitWriter<'a, REG, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div1)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<CpuclkdivSpec> {
        RatioW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CpuclkdivSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuclkdivSpec;
impl crate::RegisterSpec for CpuclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuclkdiv::R`](R) reader structure"]
impl crate::Readable for CpuclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuclkdiv::W`](W) writer structure"]
impl crate::Writable for CpuclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCLKDIV to value 0"]
impl crate::Resettable for CpuclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
