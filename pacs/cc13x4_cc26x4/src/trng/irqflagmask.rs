#[doc = "Register `IRQFLAGMASK` reader"]
pub type R = crate::R<IrqflagmaskSpec>;
#[doc = "Register `IRQFLAGMASK` writer"]
pub type W = crate::W<IrqflagmaskSpec>;
#[doc = "Field `RDY` reader - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHUTDOWN_OVF` reader - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
pub type ShutdownOvfR = crate::BitReader;
#[doc = "Field `SHUTDOWN_OVF` writer - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
pub type ShutdownOvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
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
1: Allow IRQFLAGSTAT.RDY to activate the interrupt from this module."]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RdyW<IrqflagmaskSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Allow IRQFLAGSTAT.SHUTDOWN_OVF to activate the interrupt from this module."]
    #[inline(always)]
    #[must_use]
    pub fn shutdown_ovf(&mut self) -> ShutdownOvfW<IrqflagmaskSpec> {
        ShutdownOvfW::new(self, 1)
    }
}
#[doc = "Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflagmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflagmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqflagmaskSpec;
impl crate::RegisterSpec for IrqflagmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqflagmask::R`](R) reader structure"]
impl crate::Readable for IrqflagmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`irqflagmask::W`](W) writer structure"]
impl crate::Writable for IrqflagmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQFLAGMASK to value 0"]
impl crate::Resettable for IrqflagmaskSpec {
    const RESET_VALUE: u32 = 0;
}
