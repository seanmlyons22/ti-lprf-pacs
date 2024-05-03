#[doc = "Register `NVIC_IABR1` reader"]
pub type R = crate::R<NvicIabr1Spec>;
#[doc = "Register `NVIC_IABR1` writer"]
pub type W = crate::W<NvicIabr1Spec>;
#[doc = "Field `ACTIVE32` reader - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
pub type Active32R = crate::BitReader;
#[doc = "Field `ACTIVE32` writer - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
pub type Active32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE33` reader - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
pub type Active33R = crate::BitReader;
#[doc = "Field `ACTIVE33` writer - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
pub type Active33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn active32(&self) -> Active32R {
        Active32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn active33(&self) -> Active33R {
        Active33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active32(&mut self) -> Active32W<NvicIabr1Spec> {
        Active32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active33(&mut self) -> Active33W<NvicIabr1Spec> {
        Active33W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<NvicIabr1Spec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Irq 32 to 63 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIabr1Spec;
impl crate::RegisterSpec for NvicIabr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iabr1::R`](R) reader structure"]
impl crate::Readable for NvicIabr1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iabr1::W`](W) writer structure"]
impl crate::Writable for NvicIabr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IABR1 to value 0"]
impl crate::Resettable for NvicIabr1Spec {
    const RESET_VALUE: u32 = 0;
}
