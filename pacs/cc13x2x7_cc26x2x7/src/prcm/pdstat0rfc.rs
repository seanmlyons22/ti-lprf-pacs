#[doc = "Register `PDSTAT0RFC` reader"]
pub type R = crate::R<Pdstat0rfcSpec>;
#[doc = "Register `PDSTAT0RFC` writer"]
pub type W = crate::W<Pdstat0rfcSpec>;
#[doc = "Field `ON` reader - 0:0\\]
Alias for PDSTAT0.RFC_ON"]
pub type OnR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Alias for PDSTAT0.RFC_ON"]
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
#[doc = "RFC Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0rfc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0rfc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdstat0rfcSpec;
impl crate::RegisterSpec for Pdstat0rfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdstat0rfc::R`](R) reader structure"]
impl crate::Readable for Pdstat0rfcSpec {}
#[doc = "`write(|w| ..)` method takes [`pdstat0rfc::W`](W) writer structure"]
impl crate::Writable for Pdstat0rfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDSTAT0RFC to value 0"]
impl crate::Resettable for Pdstat0rfcSpec {
    const RESET_VALUE: u32 = 0;
}
