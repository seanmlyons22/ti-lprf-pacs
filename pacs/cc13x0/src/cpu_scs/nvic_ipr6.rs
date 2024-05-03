#[doc = "Register `NVIC_IPR6` reader"]
pub type R = crate::R<NvicIpr6Spec>;
#[doc = "Register `NVIC_IPR6` writer"]
pub type W = crate::W<NvicIpr6Spec>;
#[doc = "Field `PRI_24` reader - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
pub type Pri24R = crate::FieldReader;
#[doc = "Field `PRI_24` writer - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
pub type Pri24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_25` reader - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
pub type Pri25R = crate::FieldReader;
#[doc = "Field `PRI_25` writer - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
pub type Pri25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_26` reader - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
pub type Pri26R = crate::FieldReader;
#[doc = "Field `PRI_26` writer - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
pub type Pri26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_27` reader - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
pub type Pri27R = crate::FieldReader;
#[doc = "Field `PRI_27` writer - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
pub type Pri27W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn pri_24(&self) -> Pri24R {
        Pri24R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn pri_25(&self) -> Pri25R {
        Pri25R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn pri_26(&self) -> Pri26R {
        Pri26R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn pri_27(&self) -> Pri27R {
        Pri27R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 24 (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_24(&mut self) -> Pri24W<NvicIpr6Spec> {
        Pri24W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 25 (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_25(&mut self) -> Pri25W<NvicIpr6Spec> {
        Pri25W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 26 (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_26(&mut self) -> Pri26W<NvicIpr6Spec> {
        Pri26W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 27 (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_27(&mut self) -> Pri27W<NvicIpr6Spec> {
        Pri27W::new(self, 24)
    }
}
#[doc = "Irq 24 to 27 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr6Spec;
impl crate::RegisterSpec for NvicIpr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr6::R`](R) reader structure"]
impl crate::Readable for NvicIpr6Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr6::W`](W) writer structure"]
impl crate::Writable for NvicIpr6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR6 to value 0"]
impl crate::Resettable for NvicIpr6Spec {
    const RESET_VALUE: u32 = 0;
}
