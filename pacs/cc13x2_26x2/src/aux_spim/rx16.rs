#[doc = "Register `RX16` reader"]
pub type R = crate::R<Rx16Spec>;
#[doc = "Register `RX16` writer"]
pub type W = crate::W<Rx16Spec>;
#[doc = "Field `DATA` reader - 15:0\\]
Latest 16 bits received on MISO."]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Latest 16 bits received on MISO."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Receive 16 Bit Read operation stalls until current transfer completes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rx16Spec;
impl crate::RegisterSpec for Rx16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx16::R`](R) reader structure"]
impl crate::Readable for Rx16Spec {}
#[doc = "`write(|w| ..)` method takes [`rx16::W`](W) writer structure"]
impl crate::Writable for Rx16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RX16 to value 0"]
impl crate::Resettable for Rx16Spec {
    const RESET_VALUE: u32 = 0;
}
