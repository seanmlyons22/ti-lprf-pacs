#[doc = "Register `UDMACH5BSEL` reader"]
pub type R = crate::R<Udmach5bselSpec>;
#[doc = "Register `UDMACH5BSEL` writer"]
pub type W = crate::W<Udmach5bselSpec>;
#[doc = "Field `RESERVED0` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits)
    }
}
impl W {}
#[doc = "Output Selection for DMA Channel 5 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach5bselSpec;
impl crate::RegisterSpec for Udmach5bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach5bsel::R`](R) reader structure"]
impl crate::Readable for Udmach5bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach5bsel::W`](W) writer structure"]
impl crate::Writable for Udmach5bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH5BSEL to value 0x39"]
impl crate::Resettable for Udmach5bselSpec {
    const RESET_VALUE: u32 = 0x39;
}
