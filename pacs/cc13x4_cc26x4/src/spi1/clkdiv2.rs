#[doc = "Register `CLKDIV2` reader"]
pub type R = crate::R<Clkdiv2Spec>;
#[doc = "Register `CLKDIV2` writer"]
pub type W = crate::W<Clkdiv2Spec>;
#[doc = "2:0\\]
Selects divide ratio of module clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "7: Divide clock source by 8"]
    DivBy8 = 7,
    #[doc = "6: Divide clock source by 7"]
    DivBy7 = 6,
    #[doc = "5: Divide clock source by 6"]
    DivBy6 = 5,
    #[doc = "4: Divide clock source by 5"]
    DivBy5 = 4,
    #[doc = "3: Divide clock source by 4"]
    DivBy4 = 3,
    #[doc = "2: Divide clock source by 3"]
    DivBy3 = 2,
    #[doc = "1: Divide clock source by 2"]
    DivBy2 = 1,
    #[doc = "0: Do not divide clock source"]
    DivBy1 = 0,
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
Selects divide ratio of module clock"]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            7 => Ratio::DivBy8,
            6 => Ratio::DivBy7,
            5 => Ratio::DivBy6,
            4 => Ratio::DivBy5,
            3 => Ratio::DivBy4,
            2 => Ratio::DivBy3,
            1 => Ratio::DivBy2,
            0 => Ratio::DivBy1,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Ratio::DivBy8
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn is_div_by_7(&self) -> bool {
        *self == Ratio::DivBy7
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn is_div_by_6(&self) -> bool {
        *self == Ratio::DivBy6
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn is_div_by_5(&self) -> bool {
        *self == Ratio::DivBy5
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Ratio::DivBy4
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn is_div_by_3(&self) -> bool {
        *self == Ratio::DivBy3
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Ratio::DivBy2
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Ratio::DivBy1
    }
}
#[doc = "Field `RATIO` writer - 2:0\\]
Selects divide ratio of module clock"]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ratio, crate::Safe>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy8)
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy7)
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy6)
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy5)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy4)
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy3)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy2)
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::DivBy1)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Selects divide ratio of module clock"]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 7) as u8)
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
Selects divide ratio of module clock"]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<Clkdiv2Spec> {
        RatioW::new(self, 0)
    }
}
#[doc = "This register is used to specify a divide ratio of the SPI functional clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkdiv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkdiv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv2Spec;
impl crate::RegisterSpec for Clkdiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv2::R`](R) reader structure"]
impl crate::Readable for Clkdiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv2::W`](W) writer structure"]
impl crate::Writable for Clkdiv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for Clkdiv2Spec {
    const RESET_VALUE: u32 = 0;
}
