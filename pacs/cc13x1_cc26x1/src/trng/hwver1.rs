#[doc = "Register `HWVER1` reader"]
pub struct R(crate::R<HWVER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWVER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWVER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWVER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWVER1` writer"]
pub struct W(crate::W<HWVER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWVER1_SPEC>;
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
impl From<crate::W<HWVER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWVER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REV` reader - 7:0\\]
The revision number of this module is Rev 2.0."]
pub type REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REV` writer - 7:0\\]
The revision number of this module is Rev 2.0."]
pub type REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER1_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWVER1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
The revision number of this module is Rev 2.0."]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> REV_W<0> {
        REV_W::new(self)
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
#[doc = "HW Version 1 TRNG Revision Number\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwver1](index.html) module"]
pub struct HWVER1_SPEC;
impl crate::RegisterSpec for HWVER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwver1::R](R) reader structure"]
impl crate::Readable for HWVER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwver1::W](W) writer structure"]
impl crate::Writable for HWVER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWVER1 to value 0x20"]
impl crate::Resettable for HWVER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
