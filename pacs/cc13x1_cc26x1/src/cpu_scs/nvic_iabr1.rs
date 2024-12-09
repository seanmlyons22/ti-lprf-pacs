#[doc = "Register `NVIC_IABR1` reader"]
pub type R = crate::R<NvicIabr1Spec>;
#[doc = "Register `NVIC_IABR1` writer"]
pub type W = crate::W<NvicIabr1Spec>;
#[doc = "Field `ACTIVE32` reader - 0:0\\]
Reading 0 from this bit implies that interrupt line 32 is not active. Reading 1 from this bit implies that the interrupt line 32 is active (See EVENT:CPUIRQSEL32.EV for details)."]
pub type Active32R = crate::BitReader;
#[doc = "Field `ACTIVE33` reader - 1:1\\]
Reading 0 from this bit implies that interrupt line 33 is not active. Reading 1 from this bit implies that the interrupt line 33 is active (See EVENT:CPUIRQSEL33.EV for details)."]
pub type Active33R = crate::BitReader;
#[doc = "Field `ACTIVE34` reader - 2:2\\]
Reading 0 from this bit implies that interrupt line 34 is not active. Reading 1 from this bit implies that the interrupt line 34 is active (See EVENT:CPUIRQSEL34.EV for details)."]
pub type Active34R = crate::BitReader;
#[doc = "Field `ACTIVE35` reader - 3:3\\]
Reading 0 from this bit implies that interrupt line 35 is not active. Reading 1 from this bit implies that the interrupt line 35 is active (See EVENT:CPUIRQSEL35.EV for details)."]
pub type Active35R = crate::BitReader;
#[doc = "Field `ACTIVE36` reader - 4:4\\]
Reading 0 from this bit implies that interrupt line 36 is not active. Reading 1 from this bit implies that the interrupt line 36 is active (See EVENT:CPUIRQSEL36.EV for details)."]
pub type Active36R = crate::BitReader;
#[doc = "Field `ACTIVE37` reader - 5:5\\]
Reading 0 from this bit implies that interrupt line 37 is not active. Reading 1 from this bit implies that the interrupt line 37 is active (See EVENT:CPUIRQSEL37.EV for details)."]
pub type Active37R = crate::BitReader;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
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
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 34 is not active. Reading 1 from this bit implies that the interrupt line 34 is active (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn active34(&self) -> Active34R {
        Active34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 35 is not active. Reading 1 from this bit implies that the interrupt line 35 is active (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn active35(&self) -> Active35R {
        Active35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 36 is not active. Reading 1 from this bit implies that the interrupt line 36 is active (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn active36(&self) -> Active36R {
        Active36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 37 is not active. Reading 1 from this bit implies that the interrupt line 37 is active (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn active37(&self) -> Active37R {
        Active37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
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
