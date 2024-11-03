#[doc = "Register `CLKCFG0` reader"]
pub type R = crate::R<Clkcfg0Spec>;
#[doc = "Register `CLKCFG0` writer"]
pub type W = crate::W<Clkcfg0Spec>;
#[doc = "2:0\\]
Prescaler configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Presc {
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
impl From<Presc> for u8 {
    #[inline(always)]
    fn from(variant: Presc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Presc {
    type Ux = u8;
}
impl crate::IsEnum for Presc {}
#[doc = "Field `PRESC` reader - 2:0\\]
Prescaler configuration"]
pub type PrescR = crate::FieldReader<Presc>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Presc {
        match self.bits {
            7 => Presc::DivBy8,
            6 => Presc::DivBy7,
            5 => Presc::DivBy6,
            4 => Presc::DivBy5,
            3 => Presc::DivBy4,
            2 => Presc::DivBy3,
            1 => Presc::DivBy2,
            0 => Presc::DivBy1,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn is_div_by_8(&self) -> bool {
        *self == Presc::DivBy8
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn is_div_by_7(&self) -> bool {
        *self == Presc::DivBy7
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn is_div_by_6(&self) -> bool {
        *self == Presc::DivBy6
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn is_div_by_5(&self) -> bool {
        *self == Presc::DivBy5
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn is_div_by_4(&self) -> bool {
        *self == Presc::DivBy4
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn is_div_by_3(&self) -> bool {
        *self == Presc::DivBy3
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn is_div_by_2(&self) -> bool {
        *self == Presc::DivBy2
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn is_div_by_1(&self) -> bool {
        *self == Presc::DivBy1
    }
}
#[doc = "Field `PRESC` writer - 2:0\\]
Prescaler configuration"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Presc, crate::Safe>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide clock source by 8"]
    #[inline(always)]
    pub fn div_by_8(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy8)
    }
    #[doc = "Divide clock source by 7"]
    #[inline(always)]
    pub fn div_by_7(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy7)
    }
    #[doc = "Divide clock source by 6"]
    #[inline(always)]
    pub fn div_by_6(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy6)
    }
    #[doc = "Divide clock source by 5"]
    #[inline(always)]
    pub fn div_by_5(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy5)
    }
    #[doc = "Divide clock source by 4"]
    #[inline(always)]
    pub fn div_by_4(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy4)
    }
    #[doc = "Divide clock source by 3"]
    #[inline(always)]
    pub fn div_by_3(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy3)
    }
    #[doc = "Divide clock source by 2"]
    #[inline(always)]
    pub fn div_by_2(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy2)
    }
    #[doc = "Do not divide clock source"]
    #[inline(always)]
    pub fn div_by_1(self) -> &'a mut crate::W<REG> {
        self.variant(Presc::DivBy1)
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
Prescaler configuration"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new((self.bits & 7) as u8)
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
Prescaler configuration"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<Clkcfg0Spec> {
        PrescW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<Clkcfg0Spec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Clock configuration register 0. This register is used to configure the clock prescaler.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkcfg0Spec;
impl crate::RegisterSpec for Clkcfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg0::R`](R) reader structure"]
impl crate::Readable for Clkcfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg0::W`](W) writer structure"]
impl crate::Writable for Clkcfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG0 to value 0"]
impl crate::Resettable for Clkcfg0Spec {
    const RESET_VALUE: u32 = 0;
}
