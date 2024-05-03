#[doc = "Register `IRQFLAGCLR` reader"]
pub type R = crate::R<IrqflagclrSpec>;
#[doc = "Register `IRQFLAGCLR` writer"]
pub type W = crate::W<IrqflagclrSpec>;
#[doc = "Field `RDY` reader - 0:0\\]
1: Clear IRQFLAGSTAT.RDY."]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - 0:0\\]
1: Clear IRQFLAGSTAT.RDY."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
pub type ShutdownOvfR = crate::BitReader;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
pub type ShutdownOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Clear IRQFLAGSTAT.RDY."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
    #[inline(always)]
    pub fn shutdown_ovf(&self) -> ShutdownOvfR {
        ShutdownOvfR::new(((self.bits >> 1) & 1) != 0)
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
1: Clear IRQFLAGSTAT.RDY."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<IrqflagclrSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Clear IRQFLAGSTAT.SHUTDOWN_OVF."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> ShutdownOvfW<IrqflagclrSpec> {
        ShutdownOvfW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqflagclrSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Interrupt Flag Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqflagclrSpec;
impl crate::RegisterSpec for IrqflagclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqflagclr::R`](R) reader structure"]
impl crate::Readable for IrqflagclrSpec {}
#[doc = "`write(|w| ..)` method takes [`irqflagclr::W`](W) writer structure"]
impl crate::Writable for IrqflagclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQFLAGCLR to value 0"]
impl crate::Resettable for IrqflagclrSpec {
    const RESET_VALUE: u32 = 0;
}
