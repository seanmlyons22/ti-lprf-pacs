#[doc = "Register `NVIC_ISER1` reader"]
pub type R = crate::R<NvicIser1Spec>;
#[doc = "Register `NVIC_ISER1` writer"]
pub type W = crate::W<NvicIser1Spec>;
#[doc = "Field `SETENA32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type Setena32R = crate::BitReader;
#[doc = "Field `SETENA32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type Setena32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type Setena33R = crate::BitReader;
#[doc = "Field `SETENA33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type Setena33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena32(&self) -> Setena32R {
        Setena32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena33(&self) -> Setena33R {
        Setena33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena32(&mut self) -> Setena32W<NvicIser1Spec> {
        Setena32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena33(&mut self) -> Setena33W<NvicIser1Spec> {
        Setena33W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<NvicIser1Spec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Irq 32 to 63 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIser1Spec;
impl crate::RegisterSpec for NvicIser1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser1::R`](R) reader structure"]
impl crate::Readable for NvicIser1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser1::W`](W) writer structure"]
impl crate::Writable for NvicIser1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISER1 to value 0"]
impl crate::Resettable for NvicIser1Spec {
    const RESET_VALUE: u32 = 0;
}
