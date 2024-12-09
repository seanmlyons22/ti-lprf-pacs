#[doc = "Register `T1CTL` reader"]
pub type R = crate::R<T1ctlSpec>;
#[doc = "Register `T1CTL` writer"]
pub type W = crate::W<T1ctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
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
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<T1ctlSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "Timer 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t1ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t1ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T1ctlSpec;
impl crate::RegisterSpec for T1ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t1ctl::R`](R) reader structure"]
impl crate::Readable for T1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`t1ctl::W`](W) writer structure"]
impl crate::Writable for T1ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T1CTL to value 0"]
impl crate::Resettable for T1ctlSpec {
    const RESET_VALUE: u32 = 0;
}
