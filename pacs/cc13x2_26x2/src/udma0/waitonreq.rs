#[doc = "Register `WAITONREQ` reader"]
pub type R = crate::R<WaitonreqSpec>;
#[doc = "Register `WAITONREQ` writer"]
pub type W = crate::W<WaitonreqSpec>;
#[doc = "Field `CHNLSTATUS` reader - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
pub type ChnlstatusR = crate::FieldReader<u32>;
#[doc = "Field `CHNLSTATUS` writer - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
pub type ChnlstatusW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    pub fn chnlstatus(&self) -> ChnlstatusR {
        ChnlstatusR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    #[must_use]
    pub fn chnlstatus(&mut self) -> ChnlstatusW<WaitonreqSpec> {
        ChnlstatusW::new(self, 0)
    }
}
#[doc = "Channel Wait On Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitonreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waitonreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets WAITONREQ to value 0xffff_1eff"]
impl crate::Resettable for WaitonreqSpec {
    const RESET_VALUE: u32 = 0xffff_1eff;
}
