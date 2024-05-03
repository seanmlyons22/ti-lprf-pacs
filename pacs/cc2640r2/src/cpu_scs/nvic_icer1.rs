#[doc = "Register `NVIC_ICER1` reader"]
pub type R = crate::R<NvicIcer1Spec>;
#[doc = "Register `NVIC_ICER1` writer"]
pub type W = crate::W<NvicIcer1Spec>;
#[doc = "Field `CLRENA32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type Clrena32R = crate::BitReader;
#[doc = "Field `CLRENA32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
pub type Clrena32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRENA33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type Clrena33R = crate::BitReader;
#[doc = "Field `CLRENA33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
pub type Clrena33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena32(&self) -> Clrena32R {
        Clrena32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena33(&self) -> Clrena33R {
        Clrena33R::new(((self.bits >> 1) & 1) != 0)
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
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena32(&mut self) -> Clrena32W<NvicIcer1Spec> {
        Clrena32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena33(&mut self) -> Clrena33W<NvicIcer1Spec> {
        Clrena33W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<NvicIcer1Spec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Irq 32 to 63 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIcer1Spec;
impl crate::RegisterSpec for NvicIcer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icer1::R`](R) reader structure"]
impl crate::Readable for NvicIcer1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_icer1::W`](W) writer structure"]
impl crate::Writable for NvicIcer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ICER1 to value 0"]
impl crate::Resettable for NvicIcer1Spec {
    const RESET_VALUE: u32 = 0;
}
