#[doc = "Register `COMPTESTOUT` reader"]
pub type R = crate::R<ComptestoutSpec>;
#[doc = "Register `COMPTESTOUT` writer"]
pub type W = crate::W<ComptestoutSpec>;
#[doc = "0:0\\]
Comparator output in test mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    #[doc = "1: High"]
    High = 1,
    #[doc = "0: Low"]
    Low = 0,
}
impl From<Stat> for bool {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
Comparator output in test mode."]
pub type StatR = crate::BitReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stat {
        match self.bits {
            true => Stat::High,
            false => Stat::Low,
        }
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Stat::High
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Stat::Low
    }
}
#[doc = "Field `STAT` writer - 0:0\\]
Comparator output in test mode."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG, Stat>;
impl<'a, REG> StatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Stat::High)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Stat::Low)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Comparator output in test mode."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
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
Comparator output in test mode."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<ComptestoutSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ComptestoutSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "BATMON comparator test mode output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comptestout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comptestout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ComptestoutSpec;
impl crate::RegisterSpec for ComptestoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comptestout::R`](R) reader structure"]
impl crate::Readable for ComptestoutSpec {}
#[doc = "`write(|w| ..)` method takes [`comptestout::W`](W) writer structure"]
impl crate::Writable for ComptestoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COMPTESTOUT to value 0"]
impl crate::Resettable for ComptestoutSpec {
    const RESET_VALUE: u32 = 0;
}
