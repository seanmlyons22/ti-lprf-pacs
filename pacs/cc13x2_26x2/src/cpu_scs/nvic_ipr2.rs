#[doc = "Register `NVIC_IPR2` reader"]
pub type R = crate::R<NvicIpr2Spec>;
#[doc = "Register `NVIC_IPR2` writer"]
pub type W = crate::W<NvicIpr2Spec>;
#[doc = "Field `PRI_8` reader - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
pub type Pri8R = crate::FieldReader;
#[doc = "Field `PRI_8` writer - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
pub type Pri8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_9` reader - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
pub type Pri9R = crate::FieldReader;
#[doc = "Field `PRI_9` writer - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
pub type Pri9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_10` reader - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
pub type Pri10R = crate::FieldReader;
#[doc = "Field `PRI_10` writer - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
pub type Pri10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_11` reader - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
pub type Pri11R = crate::FieldReader;
#[doc = "Field `PRI_11` writer - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
pub type Pri11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn pri_8(&self) -> Pri8R {
        Pri8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn pri_9(&self) -> Pri9R {
        Pri9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn pri_10(&self) -> Pri10R {
        Pri10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn pri_11(&self) -> Pri11R {
        Pri11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 8 (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_8(&mut self) -> Pri8W<NvicIpr2Spec> {
        Pri8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 9 (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_9(&mut self) -> Pri9W<NvicIpr2Spec> {
        Pri9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Priority of interrupt 10 (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_10(&mut self) -> Pri10W<NvicIpr2Spec> {
        Pri10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Priority of interrupt 11 (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_11(&mut self) -> Pri11W<NvicIpr2Spec> {
        Pri11W::new(self, 24)
    }
}
#[doc = "Irq 8 to 11 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr2Spec;
impl crate::RegisterSpec for NvicIpr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr2::R`](R) reader structure"]
impl crate::Readable for NvicIpr2Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr2::W`](W) writer structure"]
impl crate::Writable for NvicIpr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NvicIpr2Spec {
    const RESET_VALUE: u32 = 0;
}
