#[doc = "Register `CLEARBURST` reader"]
pub type R = crate::R<ClearburstSpec>;
#[doc = "Register `CLEARBURST` writer"]
pub type W = crate::W<ClearburstSpec>;
#[doc = "Field `CHNLS` reader - 7:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
pub type ChnlsR = crate::FieldReader;
#[doc = "Field `CHNLS` writer - 7:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a uDMA channel is not implemented has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<ClearburstSpec> {
        ChnlsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<ClearburstSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Channel Clear UseBurst Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearburst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearburst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClearburstSpec;
impl crate::RegisterSpec for ClearburstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clearburst::R`](R) reader structure"]
impl crate::Readable for ClearburstSpec {}
#[doc = "`write(|w| ..)` method takes [`clearburst::W`](W) writer structure"]
impl crate::Writable for ClearburstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEARBURST to value 0"]
impl crate::Resettable for ClearburstSpec {
    const RESET_VALUE: u32 = 0;
}