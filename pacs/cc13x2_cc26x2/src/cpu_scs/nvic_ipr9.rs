#[doc = "Register `NVIC_IPR9` reader"]
pub type R = crate::R<NvicIpr9Spec>;
#[doc = "Register `NVIC_IPR9` writer"]
pub type W = crate::W<NvicIpr9Spec>;
#[doc = "Field `PRI_36` reader - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
pub type Pri36R = crate::FieldReader;
#[doc = "Field `PRI_36` writer - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
pub type Pri36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRI_37` reader - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
pub type Pri37R = crate::FieldReader;
#[doc = "Field `PRI_37` writer - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
pub type Pri37W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    pub fn pri_36(&self) -> Pri36R {
        Pri36R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    pub fn pri_37(&self) -> Pri37R {
        Pri37R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Priority of interrupt 36 (See EVENT:CPUIRQSEL36.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_36(&mut self) -> Pri36W<NvicIpr9Spec> {
        Pri36W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Priority of interrupt 37 (See EVENT:CPUIRQSEL37.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn pri_37(&mut self) -> Pri37W<NvicIpr9Spec> {
        Pri37W::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<NvicIpr9Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Irq 32 to 35 Priority This register is used to assign a priority from 0 to 255 to each of the available interrupts. 0 is the highest priority, and 255 is the lowest. The interpretation of the Interrupt Priority Registers changes based on the setting in AIRCR.PRIGROUP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ipr9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ipr9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIpr9Spec;
impl crate::RegisterSpec for NvicIpr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ipr9::R`](R) reader structure"]
impl crate::Readable for NvicIpr9Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ipr9::W`](W) writer structure"]
impl crate::Writable for NvicIpr9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IPR9 to value 0"]
impl crate::Resettable for NvicIpr9Spec {
    const RESET_VALUE: u32 = 0;
}
