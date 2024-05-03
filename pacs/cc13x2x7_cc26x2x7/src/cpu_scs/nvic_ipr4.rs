#[doc = "Register `NVIC_IPR4` reader"]
pub type R = crate::R<NvicIpr4Spec>;
#[doc = "Register `NVIC_IPR4` writer"]
pub type W = crate::W<NvicIpr4Spec>;
#[doc = "Field `PRI_16` reader - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub type Pri16R = crate::FieldReader;
#[doc = "Field `PRI_16` writer - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
pub type Pri16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_17` reader - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub type Pri17R = crate::FieldReader;
#[doc = "Field `PRI_17` writer - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
pub type Pri17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_18` reader - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub type Pri18R = crate::FieldReader;
#[doc = "Field `PRI_18` writer - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
pub type Pri18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_19` reader - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub type Pri19R = crate::FieldReader;
#[doc = "Field `PRI_19` writer - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
pub type Pri19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn pri_16(&self) -> Pri16R {
        Pri16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn pri_17(&self) -> Pri17R {
        Pri17R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn pri_18(&self) -> Pri18R {
        Pri18R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn pri_19(&self) -> Pri19R {
        Pri19R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 16 (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_16(&mut self) -> Pri16W<NvicIpr4Spec> {
        Pri16W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 17 (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_17(&mut self) -> Pri17W<NvicIpr4Spec> {
        Pri17W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 18 (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_18(&mut self) -> Pri18W<NvicIpr4Spec> {
        Pri18W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 19 (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_19(&mut self) -> Pri19W<NvicIpr4Spec> {
        Pri19W::new(self, 24)
    }
}
#[doc = "Irq 16 to 19 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr4Spec;
impl crate::RegisterSpec for NvicIpr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr4::R`](R) reader structure"]
impl crate::Readable for NvicIpr4Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr4::W`](W) writer structure"]
impl crate::Writable for NvicIpr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR4 to value 0"]
impl crate::Resettable for NvicIpr4Spec {
    const RESET_VALUE: u32 = 0;
}
