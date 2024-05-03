#[doc = "Register `NVIC_IPR5` reader"]
pub type R = crate::R<NvicIpr5Spec>;
#[doc = "Register `NVIC_IPR5` writer"]
pub type W = crate::W<NvicIpr5Spec>;
#[doc = "Field `PRI_20` reader - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub type Pri20R = crate::FieldReader;
#[doc = "Field `PRI_20` writer - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
pub type Pri20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_21` reader - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub type Pri21R = crate::FieldReader;
#[doc = "Field `PRI_21` writer - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
pub type Pri21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_22` reader - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub type Pri22R = crate::FieldReader;
#[doc = "Field `PRI_22` writer - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
pub type Pri22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_23` reader - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub type Pri23R = crate::FieldReader;
#[doc = "Field `PRI_23` writer - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
pub type Pri23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn pri_20(&self) -> Pri20R {
        Pri20R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn pri_21(&self) -> Pri21R {
        Pri21R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn pri_22(&self) -> Pri22R {
        Pri22R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn pri_23(&self) -> Pri23R {
        Pri23R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 20 (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_20(&mut self) -> Pri20W<NvicIpr5Spec> {
        Pri20W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 21 (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_21(&mut self) -> Pri21W<NvicIpr5Spec> {
        Pri21W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 22 (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_22(&mut self) -> Pri22W<NvicIpr5Spec> {
        Pri22W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 23 (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_23(&mut self) -> Pri23W<NvicIpr5Spec> {
        Pri23W::new(self, 24)
    }
}
#[doc = "Irq 20 to 23 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr5Spec;
impl crate::RegisterSpec for NvicIpr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr5::R`](R) reader structure"]
impl crate::Readable for NvicIpr5Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr5::W`](W) writer structure"]
impl crate::Writable for NvicIpr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NvicIpr5Spec {
    const RESET_VALUE: u32 = 0;
}
