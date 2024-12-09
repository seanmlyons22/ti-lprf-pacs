#[doc = "Register `SRIS` reader"]
pub type R = crate::R<SrisSpec>;
#[doc = "Register `SRIS` writer"]
pub type W = crate::W<SrisSpec>;
#[doc = "Field `DATARIS` reader - 0:0\\]
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DatarisR = crate::BitReader;
#[doc = "Field `STARTRIS` reader - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
pub type StartrisR = crate::BitReader;
#[doc = "Field `STOPRIS` reader - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
pub type StoprisR = crate::BitReader;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn dataris(&self) -> DatarisR {
        DatarisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
    #[inline(always)]
    pub fn startris(&self) -> StartrisR {
        StartrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
    #[inline(always)]
    pub fn stopris(&self) -> StoprisR {
        StoprisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrisSpec;
impl crate::RegisterSpec for SrisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sris::R`](R) reader structure"]
impl crate::Readable for SrisSpec {}
#[doc = "`write(|w| ..)` method takes [`sris::W`](W) writer structure"]
impl crate::Writable for SrisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRIS to value 0"]
impl crate::Resettable for SrisSpec {
    const RESET_VALUE: u32 = 0;
}
