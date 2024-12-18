#[doc = "Register `SYSBUSCLKDIV` reader"]
pub type R = crate::R<SysbusclkdivSpec>;
#[doc = "Register `SYSBUSCLKDIV` writer"]
pub type W = crate::W<SysbusclkdivSpec>;
#[doc = "2:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Div2 = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Div1 = 0,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ratio> {
        match self.bits {
            1 => Some(Ratio::Div2),
            0 => Some(Ratio::Div1),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ratio::Div2
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ratio::Div1
    }
}
#[doc = "Field `RATIO` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ratio>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div2)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div1)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<SysbusclkdivSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysbusclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysbusclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysbusclkdivSpec;
impl crate::RegisterSpec for SysbusclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysbusclkdiv::R`](R) reader structure"]
impl crate::Readable for SysbusclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`sysbusclkdiv::W`](W) writer structure"]
impl crate::Writable for SysbusclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSBUSCLKDIV to value 0"]
impl crate::Resettable for SysbusclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
