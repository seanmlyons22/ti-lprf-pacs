#[doc = "Register `DEVTYPE` reader"]
pub struct R(crate::R<DEVTYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVTYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVTYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVTYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVTYPE` writer"]
pub struct W(crate::W<DEVTYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVTYPE_SPEC>;
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
impl From<crate::W<DEVTYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVTYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAJORTYPE` reader - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
pub type MAJORTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJORTYPE` writer - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
pub type MAJORTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVTYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `SUBTYPE` reader - 7:4\\]
Identifies the classification of the debug component"]
pub type SUBTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBTYPE` writer - 7:4\\]
Identifies the classification of the debug component"]
pub type SUBTYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVTYPE_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVTYPE_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
    #[inline(always)]
    pub fn majortype(&self) -> MAJORTYPE_R {
        MAJORTYPE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Identifies the classification of the debug component"]
    #[inline(always)]
    pub fn subtype(&self) -> SUBTYPE_R {
        SUBTYPE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Indicates this device is a trace sink and specifically a TPIU"]
    #[inline(always)]
    #[must_use]
    pub fn majortype(&mut self) -> MAJORTYPE_W<0> {
        MAJORTYPE_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Identifies the classification of the debug component"]
    #[inline(always)]
    #[must_use]
    pub fn subtype(&mut self) -> SUBTYPE_W<4> {
        SUBTYPE_W::new(self)
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
#[doc = "The Device Type Identification\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devtype](index.html) module"]
pub struct DEVTYPE_SPEC;
impl crate::RegisterSpec for DEVTYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devtype::R](R) reader structure"]
impl crate::Readable for DEVTYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devtype::W](W) writer structure"]
impl crate::Writable for DEVTYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVTYPE to value 0x11"]
impl crate::Resettable for DEVTYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x11;
}
