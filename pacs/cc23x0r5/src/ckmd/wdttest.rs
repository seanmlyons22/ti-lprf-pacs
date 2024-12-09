#[doc = "Register `WDTTEST` reader"]
pub type R = crate::R<WdttestSpec>;
#[doc = "Register `WDTTEST` writer"]
pub type W = crate::W<WdttestSpec>;
#[doc = "0:0\\]
WDT stall enable This field is only writable if not locked. See WDTLOCK register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stallen {
    #[doc = "1: ENABLE WDT stops counting while the CPU is stopped by a debugger."]
    En = 1,
    #[doc = "0: DISABLE WDT continues counting while the CPU is stopped by a debugger."]
    Dis = 0,
}
impl From<Stallen> for bool {
    #[inline(always)]
    fn from(variant: Stallen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALLEN` reader - 0:0\\]
WDT stall enable This field is only writable if not locked. See WDTLOCK register."]
pub type StallenR = crate::BitReader<Stallen>;
impl StallenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stallen {
        match self.bits {
            true => Stallen::En,
            false => Stallen::Dis,
        }
    }
    #[doc = "ENABLE WDT stops counting while the CPU is stopped by a debugger."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Stallen::En
    }
    #[doc = "DISABLE WDT continues counting while the CPU is stopped by a debugger."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Stallen::Dis
    }
}
#[doc = "Field `STALLEN` writer - 0:0\\]
WDT stall enable This field is only writable if not locked. See WDTLOCK register."]
pub type StallenW<'a, REG> = crate::BitWriter<'a, REG, Stallen>;
impl<'a, REG> StallenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ENABLE WDT stops counting while the CPU is stopped by a debugger."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::En)
    }
    #[doc = "DISABLE WDT continues counting while the CPU is stopped by a debugger."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
WDT stall enable This field is only writable if not locked. See WDTLOCK register."]
    #[inline(always)]
    pub fn stallen(&self) -> StallenR {
        StallenR::new((self.bits & 1) != 0)
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
WDT stall enable This field is only writable if not locked. See WDTLOCK register."]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> StallenW<WdttestSpec> {
        StallenW::new(self, 0)
    }
}
#[doc = "WDT test mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdttest::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdttest::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdttestSpec;
impl crate::RegisterSpec for WdttestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdttest::R`](R) reader structure"]
impl crate::Readable for WdttestSpec {}
#[doc = "`write(|w| ..)` method takes [`wdttest::W`](W) writer structure"]
impl crate::Writable for WdttestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDTTEST to value 0"]
impl crate::Resettable for WdttestSpec {
    const RESET_VALUE: u32 = 0;
}
