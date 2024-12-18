#[doc = "Register `SYNCLF` reader"]
pub type R = crate::R<SynclfSpec>;
#[doc = "Register `SYNCLF` writer"]
pub type W = crate::W<SynclfSpec>;
#[doc = "Field `PHASE` reader - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
pub type PhaseR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit will always return the SCLK_LF phase. The return will delayed until a positive or negative edge of SCLK_LF is seen. 0: Falling edge of SCLK_LF 1: Rising edge of SCLK_LF"]
    #[inline(always)]
    pub fn phase(&self) -> PhaseR {
        PhaseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synclf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synclf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SynclfSpec;
impl crate::RegisterSpec for SynclfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synclf::R`](R) reader structure"]
impl crate::Readable for SynclfSpec {}
#[doc = "`write(|w| ..)` method takes [`synclf::W`](W) writer structure"]
impl crate::Writable for SynclfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNCLF to value 0"]
impl crate::Resettable for SynclfSpec {
    const RESET_VALUE: u32 = 0;
}
