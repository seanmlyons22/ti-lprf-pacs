#[doc = "Register `IRQSTAT` reader"]
pub type R = crate::R<IrqstatSpec>;
#[doc = "Register `IRQSTAT` writer"]
pub type W = crate::W<IrqstatSpec>;
#[doc = "Field `STAT` reader - 0:0\\]
TRNG Interrupt status. OR'ed version of IRQFLAGSTAT.SHUTDOWN_OVF and IRQFLAGSTAT.RDY"]
pub type StatR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TRNG Interrupt status. OR'ed version of IRQFLAGSTAT.SHUTDOWN_OVF and IRQFLAGSTAT.RDY"]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqstatSpec;
impl crate::RegisterSpec for IrqstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irqstat::R`](R) reader structure"]
impl crate::Readable for IrqstatSpec {}
#[doc = "`write(|w| ..)` method takes [`irqstat::W`](W) writer structure"]
impl crate::Writable for IrqstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRQSTAT to value 0"]
impl crate::Resettable for IrqstatSpec {
    const RESET_VALUE: u32 = 0;
}
