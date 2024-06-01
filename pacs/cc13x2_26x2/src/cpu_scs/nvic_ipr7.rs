#[doc = "Register `NVIC_IPR7` reader"]
pub type R = crate::R<NvicIpr7Spec>;
#[doc = "Register `NVIC_IPR7` writer"]
pub type W = crate::W<NvicIpr7Spec>;
#[doc = "Field `PRI_28` reader - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
pub type Pri28R = crate::FieldReader;
#[doc = "Field `PRI_28` writer - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
pub type Pri28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_29` reader - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
pub type Pri29R = crate::FieldReader;
#[doc = "Field `PRI_29` writer - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
pub type Pri29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_30` reader - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
pub type Pri30R = crate::FieldReader;
#[doc = "Field `PRI_30` writer - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
pub type Pri30W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_31` reader - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
pub type Pri31R = crate::FieldReader;
#[doc = "Field `PRI_31` writer - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
pub type Pri31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn pri_28(&self) -> Pri28R {
        Pri28R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn pri_29(&self) -> Pri29R {
        Pri29R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn pri_30(&self) -> Pri30R {
        Pri30R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn pri_31(&self) -> Pri31R {
        Pri31R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 28 (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_28(&mut self) -> Pri28W<NvicIpr7Spec> {
        Pri28W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 29 (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_29(&mut self) -> Pri29W<NvicIpr7Spec> {
        Pri29W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 30 (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_30(&mut self) -> Pri30W<NvicIpr7Spec> {
        Pri30W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 31 (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_31(&mut self) -> Pri31W<NvicIpr7Spec> {
        Pri31W::new(self, 24)
    }
}
#[doc = "Irq 28 to 31 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr7Spec;
impl crate::RegisterSpec for NvicIpr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr7::R`](R) reader structure"]
impl crate::Readable for NvicIpr7Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr7::W`](W) writer structure"]
impl crate::Writable for NvicIpr7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR7 to value 0"]
impl crate::Resettable for NvicIpr7Spec {
    const RESET_VALUE: u32 = 0;
}
