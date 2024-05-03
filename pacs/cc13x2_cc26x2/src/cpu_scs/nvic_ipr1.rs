#[doc = "Register `NVIC_IPR1` reader"]
pub type R = crate::R<NvicIpr1Spec>;
#[doc = "Register `NVIC_IPR1` writer"]
pub type W = crate::W<NvicIpr1Spec>;
#[doc = "Field `PRI_4` reader - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
pub type Pri4R = crate::FieldReader;
#[doc = "Field `PRI_4` writer - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
pub type Pri4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_5` reader - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
pub type Pri5R = crate::FieldReader;
#[doc = "Field `PRI_5` writer - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
pub type Pri5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_6` reader - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
pub type Pri6R = crate::FieldReader;
#[doc = "Field `PRI_6` writer - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
pub type Pri6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_7` reader - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
pub type Pri7R = crate::FieldReader;
#[doc = "Field `PRI_7` writer - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
pub type Pri7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn pri_4(&self) -> Pri4R {
        Pri4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn pri_5(&self) -> Pri5R {
        Pri5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn pri_6(&self) -> Pri6R {
        Pri6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn pri_7(&self) -> Pri7R {
        Pri7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 4 (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_4(&mut self) -> Pri4W<NvicIpr1Spec> {
        Pri4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 5 (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_5(&mut self) -> Pri5W<NvicIpr1Spec> {
        Pri5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 6 (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_6(&mut self) -> Pri6W<NvicIpr1Spec> {
        Pri6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 7 (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_7(&mut self) -> Pri7W<NvicIpr1Spec> {
        Pri7W::new(self, 24)
    }
}
#[doc = "Irq 4 to 7 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr1Spec;
impl crate::RegisterSpec for NvicIpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr1::R`](R) reader structure"]
impl crate::Readable for NvicIpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr1::W`](W) writer structure"]
impl crate::Writable for NvicIpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR1 to value 0"]
impl crate::Resettable for NvicIpr1Spec {
    const RESET_VALUE: u32 = 0;
}
