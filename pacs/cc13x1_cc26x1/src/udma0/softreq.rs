#[doc = "Register `SOFTREQ` reader"]
pub type R = crate::R<SoftreqSpec>;
#[doc = "Register `SOFTREQ` writer"]
pub type W = crate::W<SoftreqSpec>;
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
pub type ChnlsR = crate::FieldReader<u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SoftreqSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Software Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softreq::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softreq::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftreqSpec;
impl crate::RegisterSpec for SoftreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softreq::R`](R) reader structure"]
impl crate::Readable for SoftreqSpec {}
#[doc = "`write(|w| ..)` method takes [`softreq::W`](W) writer structure"]
impl crate::Writable for SoftreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTREQ to value 0"]
impl crate::Resettable for SoftreqSpec {
    const RESET_VALUE: u32 = 0;
}
