#[doc = "Register `NVIC_IPR8` reader"]
pub type R = crate::R<NvicIpr8Spec>;
#[doc = "Register `NVIC_IPR8` writer"]
pub type W = crate::W<NvicIpr8Spec>;
#[doc = "Field `PRI_32` reader - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub type Pri32R = crate::FieldReader;
#[doc = "Field `PRI_32` writer - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
pub type Pri32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_33` reader - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub type Pri33R = crate::FieldReader;
#[doc = "Field `PRI_33` writer - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
pub type Pri33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_34` reader - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
pub type Pri34R = crate::FieldReader;
#[doc = "Field `PRI_34` writer - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
pub type Pri34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_35` reader - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
pub type Pri35R = crate::FieldReader;
#[doc = "Field `PRI_35` writer - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
pub type Pri35W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    pub fn pri_32(&self) -> Pri32R {
        Pri32R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    pub fn pri_33(&self) -> Pri33R {
        Pri33R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    pub fn pri_34(&self) -> Pri34R {
        Pri34R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    pub fn pri_35(&self) -> Pri35R {
        Pri35R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 32 (See EVENT:CPUIRQSEL32.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_32(&mut self) -> Pri32W<NvicIpr8Spec> {
        Pri32W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 33 (See EVENT:CPUIRQSEL33.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_33(&mut self) -> Pri33W<NvicIpr8Spec> {
        Pri33W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 34 (See EVENT:CPUIRQSEL34.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_34(&mut self) -> Pri34W<NvicIpr8Spec> {
        Pri34W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 35 (See EVENT:CPUIRQSEL35.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_35(&mut self) -> Pri35W<NvicIpr8Spec> {
        Pri35W::new(self, 24)
    }
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr8Spec;
impl crate::RegisterSpec for NvicIpr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr8::R`](R) reader structure"]
impl crate::Readable for NvicIpr8Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr8::W`](W) writer structure"]
impl crate::Writable for NvicIpr8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR8 to value 0"]
impl crate::Resettable for NvicIpr8Spec {
    const RESET_VALUE: u32 = 0;
}
