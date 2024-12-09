#[doc = "Register `WAITONREQ` reader"]
pub type R = crate::R<WaitonreqSpec>;
#[doc = "Register `WAITONREQ` writer"]
pub type W = crate::W<WaitonreqSpec>;
#[doc = "Field `CHNLSTATUS` reader - 7:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
pub type ChnlstatusR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    pub fn chnlstatus(&self) -> ChnlstatusR {
        ChnlstatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Channel Wait On Request Status Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitonreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waitonreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaitonreqSpec;
impl crate::RegisterSpec for WaitonreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waitonreq::R`](R) reader structure"]
impl crate::Readable for WaitonreqSpec {}
#[doc = "`write(|w| ..)` method takes [`waitonreq::W`](W) writer structure"]
impl crate::Writable for WaitonreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WAITONREQ to value 0xff"]
impl crate::Resettable for WaitonreqSpec {
    const RESET_VALUE: u32 = 0xff;
}
