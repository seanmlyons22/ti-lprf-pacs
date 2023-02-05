#[doc = "Register `SMIS` reader"]
pub struct R(crate::R<SMIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMIS` writer"]
pub struct W(crate::W<SMIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SMIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAMIS` reader - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DATAMIS_R = crate::BitReader<bool>;
#[doc = "Field `DATAMIS` writer - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DATAMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, O>;
#[doc = "Field `STARTMIS` reader - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub type STARTMIS_R = crate::BitReader<bool>;
#[doc = "Field `STARTMIS` writer - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
pub type STARTMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, O>;
#[doc = "Field `STOPMIS` reader - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub type STOPMIS_R = crate::BitReader<bool>;
#[doc = "Field `STOPMIS` writer - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
pub type STOPMIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SMIS_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMIS_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn datamis(&self) -> DATAMIS_R {
        DATAMIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    pub fn startmis(&self) -> STARTMIS_R {
        STARTMIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    pub fn stopmis(&self) -> STOPMIS_R {
        STOPMIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Data masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    #[must_use]
    pub fn datamis(&mut self) -> DATAMIS_W<0> {
        DATAMIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Start condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STARTIC."]
    #[inline(always)]
    #[must_use]
    pub fn startmis(&mut self) -> STARTMIS_W<1> {
        STARTMIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition masked interrupt status 0: An interrupt has not occurred or is masked/disabled. 1: An unmasked Stop condition interrupt is pending. This bit is cleared by writing a 1 to the SICR.STOPIC."]
    #[inline(always)]
    #[must_use]
    pub fn stopmis(&mut self) -> STOPMIS_W<2> {
        STOPMIS_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<3> {
        RESERVED3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Masked Interrupt Status This register show which interrupt is active (based on result from SRIS and SIMR).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](index.html) module"]
pub struct SMIS_SPEC;
impl crate::RegisterSpec for SMIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smis::R](R) reader structure"]
impl crate::Readable for SMIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smis::W](W) writer structure"]
impl crate::Writable for SMIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMIS to value 0"]
impl crate::Resettable for SMIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
