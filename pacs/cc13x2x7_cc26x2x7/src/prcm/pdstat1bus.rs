#[doc = "Register `PDSTAT1BUS` reader"]
pub type R = crate::R<Pdstat1busSpec>;
#[doc = "Register `PDSTAT1BUS` writer"]
pub type W = crate::W<Pdstat1busSpec>;
#[doc = "Field `ON` reader - 0:0\\]
This is an alias for PDSTAT1.BUS_ON"]
pub type OnR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This is an alias for PDSTAT1.BUS_ON"]
    #[inline(always)]
    pub fn on(&self) -> OnR {
        OnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "BUS Power Domain Direct Read Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1bus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdstat1busSpec;
impl crate::RegisterSpec for Pdstat1busSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdstat1bus::R`](R) reader structure"]
impl crate::Readable for Pdstat1busSpec {}
#[doc = "`write(|w| ..)` method takes [`pdstat1bus::W`](W) writer structure"]
impl crate::Writable for Pdstat1busSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSTAT1BUS to value 0x01"]
impl crate::Resettable for Pdstat1busSpec {
    const RESET_VALUE: u32 = 0x01;
}
