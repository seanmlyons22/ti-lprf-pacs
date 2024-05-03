#[doc = "Register `NVIC_ISPR1` reader"]
pub type R = crate::R<NvicIspr1Spec>;
#[doc = "Register `NVIC_ISPR1` writer"]
pub type W = crate::W<NvicIspr1Spec>;
#[doc = "Field `SETPEND32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub type Setpend32R = crate::BitReader;
#[doc = "Field `SETPEND32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub type Setpend32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub type Setpend33R = crate::BitReader;
#[doc = "Field `SETPEND33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub type Setpend33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND34` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
pub type Setpend34R = crate::BitReader;
#[doc = "Field `SETPEND34` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
pub type Setpend34W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND35` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
pub type Setpend35R = crate::BitReader;
#[doc = "Field `SETPEND35` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
pub type Setpend35W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND36` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
pub type Setpend36R = crate::BitReader;
#[doc = "Field `SETPEND36` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
pub type Setpend36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND37` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
pub type Setpend37R = crate::BitReader;
#[doc = "Field `SETPEND37` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
pub type Setpend37W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend32(&self) -> Setpend32R {
        Setpend32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend33(&self) -> Setpend33R {
        Setpend33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend34(&self) -> Setpend34R {
        Setpend34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend35(&self) -> Setpend35R {
        Setpend35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend36(&self) -> Setpend36R {
        Setpend36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend37(&self) -> Setpend37R {
        Setpend37R::new(((self.bits >> 5) & 1) != 0)
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
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend32(&mut self) -> Setpend32W<NvicIspr1Spec> {
        Setpend32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend33(&mut self) -> Setpend33W<NvicIspr1Spec> {
        Setpend33W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 34 (See EVENT:CPUIRQSEL34.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend34(&mut self) -> Setpend34W<NvicIspr1Spec> {
        Setpend34W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 35 (See EVENT:CPUIRQSEL35.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend35(&mut self) -> Setpend35W<NvicIspr1Spec> {
        Setpend35W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 36 (See EVENT:CPUIRQSEL36.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend36(&mut self) -> Setpend36W<NvicIspr1Spec> {
        Setpend36W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 37 (See EVENT:CPUIRQSEL37.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend37(&mut self) -> Setpend37W<NvicIspr1Spec> {
        Setpend37W::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<NvicIspr1Spec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Irq 32 to 63 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIspr1Spec;
impl crate::RegisterSpec for NvicIspr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr1::R`](R) reader structure"]
impl crate::Readable for NvicIspr1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr1::W`](W) writer structure"]
impl crate::Writable for NvicIspr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR1 to value 0"]
impl crate::Resettable for NvicIspr1Spec {
    const RESET_VALUE: u32 = 0;
}
