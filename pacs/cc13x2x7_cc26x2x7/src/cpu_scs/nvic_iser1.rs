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
#[doc = "Field `SETENA34` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type Setena34R = crate::BitReader;
#[doc = "Field `SETENA34` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
pub type Setena34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA35` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type Setena35R = crate::BitReader;
#[doc = "Field `SETENA35` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
pub type Setena35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA36` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type Setena36R = crate::BitReader;
#[doc = "Field `SETENA36` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
pub type Setena36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA37` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type Setena37R = crate::BitReader;
#[doc = "Field `SETENA37` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
pub type Setena37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
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
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena34(&self) -> Setena34R {
        Setena34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena35(&self) -> Setena35R {
        Setena35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena36(&self) -> Setena36R {
        Setena36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena37(&self) -> Setena37R {
        Setena37R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena34(&mut self) -> Setena34W<NvicIser1Spec> {
        Setena34W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena35(&mut self) -> Setena35W<NvicIser1Spec> {
        Setena35W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena36(&mut self) -> Setena36W<NvicIser1Spec> {
        Setena36W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena37(&mut self) -> Setena37W<NvicIser1Spec> {
        Setena37W::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<NvicIser1Spec> {
        Reserved6W::new(self, 6)
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
