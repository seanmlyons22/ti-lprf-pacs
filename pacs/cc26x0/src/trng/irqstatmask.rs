#[doc = "Register `IRQSTATMASK` reader"]
pub type R = crate::R<IrqstatmaskSpec>;
#[doc = "Register `IRQSTATMASK` writer"]
pub type W = crate::W<IrqstatmaskSpec>;
#[doc = "Field `RDY` reader - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
pub type ShutdownOvfR = crate::BitReader;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
pub type ShutdownOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
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
New random value available (result of IRQFLAGSTAT.RDY AND'ed with IRQFLAGMASK.RDY)"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<IrqstatmaskSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Shutdown Overflow (result of IRQFLAGSTAT.SHUTDOWN_OVF AND'ed with IRQFLAGMASK.SHUTDOWN_OVF)"]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> ShutdownOvfW<IrqstatmaskSpec> {
        ShutdownOvfW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<IrqstatmaskSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Interrupt Status After Masking\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstatmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstatmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstatmaskSpec;
impl crate::RegisterSpec for IrqstatmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqstatmask::R`](R) reader structure"]
impl crate::Readable for IrqstatmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irqstatmask::W`](W) writer structure"]
impl crate::Writable for IrqstatmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSTATMASK to value 0"]
impl crate::Resettable for IrqstatmaskSpec {
    const RESET_VALUE: u32 = 0;
}
