#[doc = "Register `NVIC_ICPR1` reader"]
pub type R = crate::R<NvicIcpr1Spec>;
#[doc = "Register `NVIC_ICPR1` writer"]
pub type W = crate::W<NvicIcpr1Spec>;
#[doc = "Field `CLRPEND32` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub type Clrpend32R = crate::BitReader;
#[doc = "Field `CLRPEND32` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
pub type Clrpend32W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRPEND33` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub type Clrpend33R = crate::BitReader;
#[doc = "Field `CLRPEND33` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
pub type Clrpend33W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend32(&self) -> Clrpend32R {
        Clrpend32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend33(&self) -> Clrpend33R {
        Clrpend33R::new(((self.bits >> 1) & 1) != 0)
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
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 32 (See EVENT:CPUIRQSEL32.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend32(&mut self) -> Clrpend32W<NvicIcpr1Spec> {
        Clrpend32W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 33 (See EVENT:CPUIRQSEL33.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend33(&mut self) -> Clrpend33W<NvicIcpr1Spec> {
        Clrpend33W::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<NvicIcpr1Spec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Irq 32 to 63 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_icpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_icpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIcpr1Spec;
impl crate::RegisterSpec for NvicIcpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_icpr1::R`](R) reader structure"]
impl crate::Readable for NvicIcpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_icpr1::W`](W) writer structure"]
impl crate::Writable for NvicIcpr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ICPR1 to value 0"]
impl crate::Resettable for NvicIcpr1Spec {
    const RESET_VALUE: u32 = 0;
}
