#[doc = "Register `SETREQMASK` reader"]
pub type R = crate::R<SetreqmaskSpec>;
#[doc = "Register `SETREQMASK` writer"]
pub type W = crate::W<SetreqmaskSpec>;
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsR = crate::FieldReader<u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type ChnlsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> ChnlsR {
        ChnlsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> ChnlsW<SetreqmaskSpec> {
        ChnlsW::new(self, 0)
    }
}
#[doc = "Channel Set Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setreqmask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setreqmask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetreqmaskSpec;
impl crate::RegisterSpec for SetreqmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`setreqmask::R`](R) reader structure"]
impl crate::Readable for SetreqmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`setreqmask::W`](W) writer structure"]
impl crate::Writable for SetreqmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SETREQMASK to value 0"]
impl crate::Resettable for SetreqmaskSpec {
    const RESET_VALUE: u32 = 0;
}
