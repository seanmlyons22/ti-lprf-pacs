#[doc = "Register `OSCIMSC` reader"]
pub struct R(crate::R<OSCIMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCIMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCIMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCIMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCIMSC` writer"]
pub struct W(crate::W<OSCIMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCIMSC_SPEC>;
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
impl From<crate::W<OSCIMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCIMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCHFIM` reader - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
pub type RCOSCHFIM_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHFIM` writer - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
pub type RCOSCHFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `XOSCHFIM` reader - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
pub type XOSCHFIM_R = crate::BitReader<bool>;
#[doc = "Field `XOSCHFIM` writer - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
pub type XOSCHFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `RCOSCLFIM` reader - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
pub type RCOSCLFIM_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCLFIM` writer - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
pub type RCOSCLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `RCOSCDLFIM` reader - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
pub type RCOSCDLFIM_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCDLFIM` writer - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
pub type RCOSCDLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `XOSCLFIM` reader - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
pub type XOSCLFIM_R = crate::BitReader<bool>;
#[doc = "Field `XOSCLFIM` writer - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
pub type XOSCLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `XOSCDLFIM` reader - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
pub type XOSCDLFIM_R = crate::BitReader<bool>;
#[doc = "Field `XOSCDLFIM` writer - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
pub type XOSCDLFIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `LFSRCDONEIM` reader - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
pub type LFSRCDONEIM_R = crate::BitReader<bool>;
#[doc = "Field `LFSRCDONEIM` writer - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
pub type LFSRCDONEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `HFSRCPENDIM` reader - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
pub type HFSRCPENDIM_R = crate::BitReader<bool>;
#[doc = "Field `HFSRCPENDIM` writer - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
pub type HFSRCPENDIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCIMSC_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCIMSC_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    pub fn rcoschfim(&self) -> RCOSCHFIM_R {
        RCOSCHFIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    pub fn xoschfim(&self) -> XOSCHFIM_R {
        XOSCHFIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    pub fn rcosclfim(&self) -> RCOSCLFIM_R {
        RCOSCLFIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    pub fn rcoscdlfim(&self) -> RCOSCDLFIM_R {
        RCOSCDLFIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    pub fn xosclfim(&self) -> XOSCLFIM_R {
        XOSCLFIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    pub fn xoscdlfim(&self) -> XOSCDLFIM_R {
        XOSCDLFIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    pub fn lfsrcdoneim(&self) -> LFSRCDONEIM_R {
        LFSRCDONEIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    pub fn hfsrcpendim(&self) -> HFSRCPENDIM_R {
        HFSRCPENDIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable interrupt generation when RCOSCHF is qualified 1: Enable interrupt generation when RCOSCHF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfim(&mut self) -> RCOSCHFIM_W<0> {
        RCOSCHFIM_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Disable interrupt generation when XOSCHF is qualified 1: Enable interrupt generation when XOSCHF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xoschfim(&mut self) -> XOSCHFIM_W<1> {
        XOSCHFIM_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disable interrupt generation when RCOSCLF is qualified 1: Enable interrupt generation when RCOSCLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfim(&mut self) -> RCOSCLFIM_W<2> {
        RCOSCLFIM_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Disable interrupt generation when RCOSCDLF is qualified 1: Enable interrupt generation when RCOSCDLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfim(&mut self) -> RCOSCDLFIM_W<3> {
        RCOSCDLFIM_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Disable interrupt generation when XOSCLF is qualified 1: Enable interrupt generation when XOSCLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xosclfim(&mut self) -> XOSCLFIM_W<4> {
        XOSCLFIM_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Disable interrupt generation when XOSCDLF is qualified 1: Enable interrupt generation when XOSCDLF is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfim(&mut self) -> XOSCDLFIM_W<5> {
        XOSCDLFIM_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Disable interrupt generation when LFSRCDONE is qualified 1: Enable interrupt generation when LFSRCDONE is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdoneim(&mut self) -> LFSRCDONEIM_W<6> {
        LFSRCDONEIM_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Disable interrupt generation when HFSRCPEND is qualified 1: Enable interrupt generation when HFSRCPEND is qualified"]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendim(&mut self) -> HFSRCPENDIM_W<7> {
        HFSRCPENDIM_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Interrupt Mask Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscimsc](index.html) module"]
pub struct OSCIMSC_SPEC;
impl crate::RegisterSpec for OSCIMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscimsc::R](R) reader structure"]
impl crate::Readable for OSCIMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscimsc::W](W) writer structure"]
impl crate::Writable for OSCIMSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCIMSC to value 0x36"]
impl crate::Resettable for OSCIMSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x36;
}
