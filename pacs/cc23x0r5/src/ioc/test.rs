#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "0:0\\]
This is used to drive SWDIO (Serial Wire DIO) output data and output enable from debug sub-system onto DIO12 (Test Data Output) pad.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "1: Output data and output enable driven based on debug sub-system inputs"]
    High = 1,
    #[doc = "0: Output data and output enable managed by IOC"]
    Low = 0,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 0:0\\]
This is used to drive SWDIO (Serial Wire DIO) output data and output enable from debug sub-system onto DIO12 (Test Data Output) pad."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            true => Sel::High,
            false => Sel::Low,
        }
    }
    #[doc = "Output data and output enable driven based on debug sub-system inputs"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Sel::High
    }
    #[doc = "Output data and output enable managed by IOC"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Sel::Low
    }
}
#[doc = "Field `SEL` writer - 0:0\\]
This is used to drive SWDIO (Serial Wire DIO) output data and output enable from debug sub-system onto DIO12 (Test Data Output) pad."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output data and output enable driven based on debug sub-system inputs"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::High)
    }
    #[doc = "Output data and output enable managed by IOC"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Low)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is used to drive SWDIO (Serial Wire DIO) output data and output enable from debug sub-system onto DIO12 (Test Data Output) pad."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This is used to drive SWDIO (Serial Wire DIO) output data and output enable from debug sub-system onto DIO12 (Test Data Output) pad."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<TestSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "Test register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
