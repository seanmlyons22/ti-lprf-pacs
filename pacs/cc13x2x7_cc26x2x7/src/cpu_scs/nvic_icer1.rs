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
#[doc = "Field `CLRENA34` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type Clrena34R = crate::BitReader;
#[doc = "Field `CLRENA34` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type Clrena34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRENA35` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type Clrena35R = crate::BitReader;
#[doc = "Field `CLRENA35` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type Clrena35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRENA36` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type Clrena36R = crate::BitReader;
#[doc = "Field `CLRENA36` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type Clrena36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRENA37` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type Clrena37R = crate::BitReader;
#[doc = "Field `CLRENA37` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type Clrena37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
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
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena34(&self) -> Clrena34R {
        Clrena34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena35(&self) -> Clrena35R {
        Clrena35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena36(&self) -> Clrena36R {
        Clrena36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena37(&self) -> Clrena37R {
        Clrena37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
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
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena34(&mut self) -> Clrena34W<NvicIcer1Spec> {
        Clrena34W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena35(&mut self) -> Clrena35W<NvicIcer1Spec> {
        Clrena35W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena36(&mut self) -> Clrena36W<NvicIcer1Spec> {
        Clrena36W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena37(&mut self) -> Clrena37W<NvicIcer1Spec> {
        Clrena37W::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<NvicIcer1Spec> {
        Reserved6W::new(self, 6)
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
