#[doc = "Register `UDMACH5SSEL` reader"]
pub type R = crate::R<Udmach5sselSpec>;
#[doc = "Register `UDMACH5SSEL` writer"]
pub type W = crate::W<Udmach5sselSpec>;
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
#[doc = "Output Selection for DMA Channel 5 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach5sselSpec;
impl crate::RegisterSpec for Udmach5sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach5ssel::R`](R) reader structure"]
impl crate::Readable for Udmach5sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach5ssel::W`](W) writer structure"]
impl crate::Writable for Udmach5sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH5SSEL to value 0x3a"]
impl crate::Resettable for Udmach5sselSpec {
    const RESET_VALUE: u32 = 0x3a;
}
