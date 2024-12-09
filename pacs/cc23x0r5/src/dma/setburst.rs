#[doc = "Register `SETBURST` reader"]
pub type R = crate::R<SetburstSpec>;
#[doc = "Register `SETBURST` writer"]
pub type W = crate::W<SetburstSpec>;
#[doc = "Field `CHNLS` reader - 7:0\\]
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsR = crate::FieldReader;
#[doc = "Field `CHNLS` writer - 7:0\\]
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
Returns the useburst status, or disables individual channels from generating single uDMA requests. The value R is the arbitration rate and stored in the controller data structure. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch responds to both burst and single requests on channel C. The controller performs 2^R, or single, bus transfers. Bit \\[Ch\\]
= 1: uDMA channel Ch does not respond to single transfer requests. The controller only responds to burst transfer requests and performs 2^R transfers. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARBURST.CHNLS to set bit \\[Ch\\]
to 0. Bit \\[Ch\\]
= 1: Disables single transfer requests on channel Ch. The controller performs 2^R transfers for burst requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SetburstSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Set UseBurst Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setburst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setburst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetburstSpec;
impl crate::RegisterSpec for SetburstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setburst::R`](R) reader structure"]
impl crate::Readable for SetburstSpec {}
#[doc = "`write(|w| ..)` method takes [`setburst::W`](W) writer structure"]
impl crate::Writable for SetburstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETBURST to value 0"]
impl crate::Resettable for SetburstSpec {
    const RESET_VALUE: u32 = 0;
}
