#[doc = "Register `SRIS` reader"]
pub struct R(crate::R<SRIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRIS` writer"]
pub struct W(crate::W<SRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRIS_SPEC>;
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
impl From<crate::W<SRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATARIS` reader - 0:0\\]
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DATARIS_R = crate::BitReader<bool>;
#[doc = "Field `DATARIS` writer - 0:0\\]
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
pub type DATARIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, O>;
#[doc = "Field `STARTRIS` reader - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
pub type STARTRIS_R = crate::BitReader<bool>;
#[doc = "Field `STARTRIS` writer - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
pub type STARTRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, O>;
#[doc = "Field `STOPRIS` reader - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
pub type STOPRIS_R = crate::BitReader<bool>;
#[doc = "Field `STOPRIS` writer - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
pub type STOPRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRIS_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRIS_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    pub fn dataris(&self) -> DATARIS_R {
        DATARIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
    #[inline(always)]
    pub fn startris(&self) -> STARTRIS_R {
        STARTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
    #[inline(always)]
    pub fn stopris(&self) -> STOPRIS_R {
        STOPRIS_R::new(((self.bits >> 2) & 1) != 0)
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
Data raw interrupt status 0: No interrupt 1: A data received or data requested interrupt is pending. This bit is cleared by writing a 1 to the SICR.DATAIC."]
    #[inline(always)]
    #[must_use]
    pub fn dataris(&mut self) -> DATARIS_W<0> {
        DATARIS_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition raw interrupt status 0: No interrupt 1: A Start condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STARTIC."]
    #[inline(always)]
    #[must_use]
    pub fn startris(&mut self) -> STARTRIS_W<1> {
        STARTRIS_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition raw interrupt status 0: No interrupt 1: A Stop condition interrupt is pending. This bit is cleared by writing a 1 to SICR.STOPIC."]
    #[inline(always)]
    #[must_use]
    pub fn stopris(&mut self) -> STOPRIS_W<2> {
        STOPRIS_W::new(self)
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
#[doc = "Slave Raw Interrupt Status This register shows the unmasked interrupt status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sris](index.html) module"]
pub struct SRIS_SPEC;
impl crate::RegisterSpec for SRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sris::R](R) reader structure"]
impl crate::Readable for SRIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sris::W](W) writer structure"]
impl crate::Writable for SRIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRIS to value 0"]
impl crate::Resettable for SRIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
