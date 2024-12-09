#[doc = "Register `HFXTSTAT` reader"]
pub type R = crate::R<HfxtstatSpec>;
#[doc = "Register `HFXTSTAT` writer"]
pub type W = crate::W<HfxtstatSpec>;
#[doc = "Field `GOOD` reader - 0:0\\]
HFXT clock available. The frequency is not necessarily good enough for radio operation."]
pub type GoodR = crate::BitReader;
#[doc = "Field `FAULT` reader - 1:1\\]
HFXT clock fault Indicates a lower than expected HFXT frequency. HFXT will not recover from this fault, disabling and re-enabling HFXT is required."]
pub type FaultR = crate::BitReader;
#[doc = "Field `RESERVED2` reader - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `STARTUPTIME` reader - 30:16\\]
HFXT startup time Can be used by software to plan starting HFXT ahead in time. Measured whenever HFXT is enabled in CLKULL periods (24MHz), from HFXTCTL.EN until the clock is good for radio operation (amplitude compensation is settled)."]
pub type StartuptimeR = crate::FieldReader<u16>;
#[doc = "Field `RESERVED31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
HFXT clock available. The frequency is not necessarily good enough for radio operation."]
    #[inline(always)]
    pub fn good(&self) -> GoodR {
        GoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
HFXT clock fault Indicates a lower than expected HFXT frequency. HFXT will not recover from this fault, disabling and re-enabling HFXT is required."]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - 15:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:30 - 30:16\\]
HFXT startup time Can be used by software to plan starting HFXT ahead in time. Measured whenever HFXT is enabled in CLKULL periods (24MHz), from HFXTCTL.EN until the clock is good for radio operation (amplitude compensation is settled)."]
    #[inline(always)]
    pub fn startuptime(&self) -> StartuptimeR {
        StartuptimeR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved31(&self) -> Reserved31R {
        Reserved31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "HFXT status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HfxtstatSpec;
impl crate::RegisterSpec for HfxtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hfxtstat::R`](R) reader structure"]
impl crate::Readable for HfxtstatSpec {}
#[doc = "`write(|w| ..)` method takes [`hfxtstat::W`](W) writer structure"]
impl crate::Writable for HfxtstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HFXTSTAT to value 0"]
impl crate::Resettable for HfxtstatSpec {
    const RESET_VALUE: u32 = 0;
}
