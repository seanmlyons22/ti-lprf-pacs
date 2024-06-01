#[doc = "Register `DMASWRESET` reader"]
pub type R = crate::R<DmaswresetSpec>;
#[doc = "Register `DMASWRESET` writer"]
pub type W = crate::W<DmaswresetSpec>;
#[doc = "Field `SWRES` reader - 0:0\\]
Software reset enable 0 : Disabled 1 : Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMASTAT"]
pub type SwresR = crate::BitReader;
#[doc = "Field `SWRES` writer - 0:0\\]
Software reset enable 0 : Disabled 1 : Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMASTAT"]
pub type SwresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset enable 0 : Disabled 1 : Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMASTAT"]
    #[inline(always)]
    pub fn swres(&self) -> SwresR {
        SwresR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset enable 0 : Disabled 1 : Enabled (self-cleared to 0) Completion of the software reset must be checked through the DMASTAT"]
    #[inline(always)]
    #[must_use]
    pub fn swres(&mut self) -> SwresW<DmaswresetSpec> {
        SwresW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<DmaswresetSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "DMAC Software Reset Software reset is used to reset the DMAC to stop all transfers and clears the port error status register. After the software reset is performed, all the channels are disabled and no new requests are performed by the channels. The DMAC waits for the existing (active) requests to finish and accordingly sets the DMASTAT.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaswreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaswreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaswresetSpec;
impl crate::RegisterSpec for DmaswresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaswreset::R`](R) reader structure"]
impl crate::Readable for DmaswresetSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaswreset::W`](W) writer structure"]
impl crate::Writable for DmaswresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASWRESET to value 0"]
impl crate::Resettable for DmaswresetSpec {
    const RESET_VALUE: u32 = 0;
}
