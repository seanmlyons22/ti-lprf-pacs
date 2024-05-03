#[doc = "Register `SMIS` reader"]
pub type R = crate::R<SmisSpec>;
#[doc = "Register `SMIS` writer"]
pub type W = crate::W<SmisSpec>;
#[doc = "Field `DATAMIS` reader - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DatamisR = crate::BitReader;
#[doc = "Field `DATAMIS` writer - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DatamisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTMIS` reader - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub type StartmisR = crate::BitReader;
#[doc = "Field `STARTMIS` writer - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub type StartmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPMIS` reader - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub type StopmisR = crate::BitReader;
#[doc = "Field `STOPMIS` writer - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub type StopmisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn datamis(&self) -> DatamisR {
        DatamisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    pub fn startmis(&self) -> StartmisR {
        StartmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    pub fn stopmis(&self) -> StopmisR {
        StopmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    #[must_use]
    pub fn datamis(&mut self) -> DatamisW<SmisSpec> {
        DatamisW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    #[must_use]
    pub fn startmis(&mut self) -> StartmisW<SmisSpec> {
        StartmisW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    #[must_use]
    pub fn stopmis(&mut self) -> StopmisW<SmisSpec> {
        StopmisW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SmisSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmisSpec;
impl crate::RegisterSpec for SmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smis::R`](R) reader structure"]
impl crate::Readable for SmisSpec {}
#[doc = "`write(|w| ..)` method takes [`smis::W`](W) writer structure"]
impl crate::Writable for SmisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMIS to value 0"]
impl crate::Resettable for SmisSpec {
    const RESET_VALUE: u32 = 0;
}
