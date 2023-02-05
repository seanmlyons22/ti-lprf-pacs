#[doc = "Register `SICR` reader"]
pub struct R(crate::R<SICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SICR` writer"]
pub struct W(crate::W<SICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SICR_SPEC>;
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
impl From<crate::W<SICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIC` reader - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
pub type DATAIC_R = crate::BitReader<bool>;
#[doc = "Field `DATAIC` writer - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
pub type DATAIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SICR_SPEC, bool, O>;
#[doc = "Field `STARTIC` reader - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
pub type STARTIC_R = crate::BitReader<bool>;
#[doc = "Field `STARTIC` writer - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
pub type STARTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SICR_SPEC, bool, O>;
#[doc = "Field `STOPIC` reader - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub type STOPIC_R = crate::BitReader<bool>;
#[doc = "Field `STOPIC` writer - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
pub type STOPIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SICR_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SICR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    pub fn dataic(&self) -> DATAIC_R {
        DATAIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    pub fn startic(&self) -> STARTIC_R {
        STARTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    pub fn stopic(&self) -> STOPIC_R {
        STOPIC_R::new(((self.bits >> 2) & 1) != 0)
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
Data interrupt clear Writing 1 to this bit clears SRIS.DATARIS SMIS.DATAMIS."]
    #[inline(always)]
    #[must_use]
    pub fn dataic(&mut self) -> DATAIC_W<0> {
        DATAIC_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Start condition interrupt clear Writing 1 to this bit clears SRIS.STARTRIS SMIS.STARTMIS."]
    #[inline(always)]
    #[must_use]
    pub fn startic(&mut self) -> STARTIC_W<1> {
        STARTIC_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Stop condition interrupt clear Writing 1 to this bit clears SRIS.STOPRIS and SMIS.STOPMIS."]
    #[inline(always)]
    #[must_use]
    pub fn stopic(&mut self) -> STOPIC_W<2> {
        STOPIC_W::new(self)
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
#[doc = "Slave Interrupt Clear This register clears the raw interrupt SRIS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](index.html) module"]
pub struct SICR_SPEC;
impl crate::RegisterSpec for SICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sicr::R](R) reader structure"]
impl crate::Readable for SICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sicr::W](W) writer structure"]
impl crate::Writable for SICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SICR to value 0"]
impl crate::Resettable for SICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
