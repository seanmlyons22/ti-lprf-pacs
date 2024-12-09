#[doc = "Register `AESTAGOUT` reader"]
pub type R = crate::R<AestagoutSpec>;
#[doc = "Register `AESTAGOUT` writer"]
pub type W = crate::W<AestagoutSpec>;
#[doc = "Field `AES_TAG` reader - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
pub type AesTagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&self) -> AesTagR {
        AesTagR::new(self.bits)
    }
}
impl W {}
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aestagout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aestagout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AestagoutSpec;
impl crate::RegisterSpec for AestagoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aestagout::R`](R) reader structure"]
impl crate::Readable for AestagoutSpec {}
#[doc = "`write(|w| ..)` method takes [`aestagout::W`](W) writer structure"]
impl crate::Writable for AestagoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AESTAGOUT to value 0"]
impl crate::Resettable for AestagoutSpec {
    const RESET_VALUE: u32 = 0;
}
