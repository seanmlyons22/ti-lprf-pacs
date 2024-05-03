#[doc = "Register `NVIC_IPR3` reader"]
pub type R = crate::R<NvicIpr3Spec>;
#[doc = "Register `NVIC_IPR3` writer"]
pub type W = crate::W<NvicIpr3Spec>;
#[doc = "Field `PRI_12` reader - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
pub type Pri12R = crate::FieldReader;
#[doc = "Field `PRI_12` writer - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
pub type Pri12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_13` reader - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
pub type Pri13R = crate::FieldReader;
#[doc = "Field `PRI_13` writer - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
pub type Pri13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_14` reader - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
pub type Pri14R = crate::FieldReader;
#[doc = "Field `PRI_14` writer - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
pub type Pri14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_15` reader - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
pub type Pri15R = crate::FieldReader;
#[doc = "Field `PRI_15` writer - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
pub type Pri15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn pri_12(&self) -> Pri12R {
        Pri12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn pri_13(&self) -> Pri13R {
        Pri13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn pri_14(&self) -> Pri14R {
        Pri14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn pri_15(&self) -> Pri15R {
        Pri15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 12 (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_12(&mut self) -> Pri12W<NvicIpr3Spec> {
        Pri12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 13 (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_13(&mut self) -> Pri13W<NvicIpr3Spec> {
        Pri13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 14 (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_14(&mut self) -> Pri14W<NvicIpr3Spec> {
        Pri14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 15 (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_15(&mut self) -> Pri15W<NvicIpr3Spec> {
        Pri15W::new(self, 24)
    }
}
#[doc = "Irq 12 to 15 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr3Spec;
impl crate::RegisterSpec for NvicIpr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr3::R`](R) reader structure"]
impl crate::Readable for NvicIpr3Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr3::W`](W) writer structure"]
impl crate::Writable for NvicIpr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR3 to value 0"]
impl crate::Resettable for NvicIpr3Spec {
    const RESET_VALUE: u32 = 0;
}
