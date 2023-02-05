#[doc = "Register `OSCICR` reader"]
pub struct R(crate::R<OSCICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCICR` writer"]
pub struct W(crate::W<OSCICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCICR_SPEC>;
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
impl From<crate::W<OSCICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCHFC` reader - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCHFC_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCHFC` writer - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `XOSCHFC` reader - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
pub type XOSCHFC_R = crate::BitReader<bool>;
#[doc = "Field `XOSCHFC` writer - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
pub type XOSCHFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `RCOSCLFC` reader - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCLFC_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCLFC` writer - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCLFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `RCOSCDLFC` reader - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCDLFC_R = crate::BitReader<bool>;
#[doc = "Field `RCOSCDLFC` writer - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type RCOSCDLFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `XOSCLFC` reader - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
pub type XOSCLFC_R = crate::BitReader<bool>;
#[doc = "Field `XOSCLFC` writer - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
pub type XOSCLFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `XOSCDLFC` reader - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type XOSCDLFC_R = crate::BitReader<bool>;
#[doc = "Field `XOSCDLFC` writer - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
pub type XOSCDLFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `LFSRCDONEC` reader - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
pub type LFSRCDONEC_R = crate::BitReader<bool>;
#[doc = "Field `LFSRCDONEC` writer - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
pub type LFSRCDONEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `HFSRCPENDC` reader - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
pub type HFSRCPENDC_R = crate::BitReader<bool>;
#[doc = "Field `HFSRCPENDC` writer - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
pub type HFSRCPENDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCICR_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCICR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoschfc(&self) -> RCOSCHFC_R {
        RCOSCHFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoschfc(&self) -> XOSCHFC_R {
        XOSCHFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcosclfc(&self) -> RCOSCLFC_R {
        RCOSCLFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn rcoscdlfc(&self) -> RCOSCDLFC_R {
        RCOSCDLFC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xosclfc(&self) -> XOSCLFC_R {
        XOSCLFC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn xoscdlfc(&self) -> XOSCDLFC_R {
        XOSCDLFC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn lfsrcdonec(&self) -> LFSRCDONEC_R {
        LFSRCDONEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    pub fn hfsrcpendc(&self) -> HFSRCPENDC_R {
        HFSRCPENDC_R::new(((self.bits >> 7) & 1) != 0)
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
Writing 1 to this field clears the RCOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschfc(&mut self) -> RCOSCHFC_W<0> {
        RCOSCHFC_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 1 to this field clears the XOSCHF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xoschfc(&mut self) -> XOSCHFC_W<1> {
        XOSCHFC_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 1 to this field clears the RCOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcosclfc(&mut self) -> RCOSCLFC_W<2> {
        RCOSCLFC_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 1 to this field clears the RCOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rcoscdlfc(&mut self) -> RCOSCDLFC_W<3> {
        RCOSCDLFC_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 1 to this field clears the XOSCLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xosclfc(&mut self) -> XOSCLFC_W<4> {
        XOSCLFC_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 1 to this field clears the XOSCDLF raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn xoscdlfc(&mut self) -> XOSCDLFC_W<5> {
        XOSCDLFC_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 1 to this field clears the LFSRCDONE raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn lfsrcdonec(&mut self) -> LFSRCDONEC_W<6> {
        LFSRCDONEC_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 1 to this field clears the HFSRCPEND raw interrupt status. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn hfsrcpendc(&mut self) -> HFSRCPENDC_W<7> {
        HFSRCPENDC_W::new(self)
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
#[doc = "Oscillator Raw Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscicr](index.html) module"]
pub struct OSCICR_SPEC;
impl crate::RegisterSpec for OSCICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscicr::R](R) reader structure"]
impl crate::Readable for OSCICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscicr::W](W) writer structure"]
impl crate::Writable for OSCICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCICR to value 0"]
impl crate::Resettable for OSCICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
