#[doc = "Register `SWEVSET` reader"]
pub type R = crate::R<SwevsetSpec>;
#[doc = "Register `SWEVSET` writer"]
pub type W = crate::W<SwevsetSpec>;
#[doc = "Field `SWEV0` reader - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
pub type Swev0R = crate::BitReader;
#[doc = "Field `SWEV0` writer - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
pub type Swev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEV1` reader - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
pub type Swev1R = crate::BitReader;
#[doc = "Field `SWEV1` writer - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
pub type Swev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEV2` reader - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
pub type Swev2R = crate::BitReader;
#[doc = "Field `SWEV2` writer - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
pub type Swev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[inline(always)]
    pub fn swev0(&self) -> Swev0R {
        Swev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[inline(always)]
    pub fn swev1(&self) -> Swev1R {
        Swev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[inline(always)]
    pub fn swev2(&self) -> Swev2R {
        Swev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software event flag 0. 0: No effect. 1: Set software event flag 0."]
    #[inline(always)]
    #[must_use]
    pub fn swev0(&mut self) -> Swev0W<SwevsetSpec> {
        Swev0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software event flag 1. 0: No effect. 1: Set software event flag 1."]
    #[inline(always)]
    #[must_use]
    pub fn swev1(&mut self) -> Swev1W<SwevsetSpec> {
        Swev1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Software event flag 2. 0: No effect. 1: Set software event flag 2."]
    #[inline(always)]
    #[must_use]
    pub fn swev2(&mut self) -> Swev2W<SwevsetSpec> {
        Swev2W::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SwevsetSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevsetSpec;
impl crate::RegisterSpec for SwevsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swevset::R`](R) reader structure"]
impl crate::Readable for SwevsetSpec {}
#[doc = "`write(|w| ..)` method takes [`swevset::W`](W) writer structure"]
impl crate::Writable for SwevsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWEVSET to value 0"]
impl crate::Resettable for SwevsetSpec {
    const RESET_VALUE: u32 = 0;
}
