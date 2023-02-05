#[doc = "Register `MAIR1` reader"]
pub struct R(crate::R<MAIR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAIR1` writer"]
pub struct W(crate::W<MAIR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAIR1_SPEC>;
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
impl From<crate::W<MAIR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAIR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR4` reader - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type ATTR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR4` writer - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
pub type ATTR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR5` reader - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type ATTR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR5` writer - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
pub type ATTR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR6` reader - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type ATTR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR6` writer - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
pub type ATTR6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR7` reader - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type ATTR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR7` writer - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
pub type ATTR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    pub fn attr4(&self) -> ATTR4_R {
        ATTR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    pub fn attr5(&self) -> ATTR5_R {
        ATTR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    pub fn attr6(&self) -> ATTR6_R {
        ATTR6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    pub fn attr7(&self) -> ATTR7_R {
        ATTR7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 4"]
    #[inline(always)]
    #[must_use]
    pub fn attr4(&mut self) -> ATTR4_W<0> {
        ATTR4_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 5"]
    #[inline(always)]
    #[must_use]
    pub fn attr5(&mut self) -> ATTR5_W<8> {
        ATTR5_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 6"]
    #[inline(always)]
    #[must_use]
    pub fn attr6(&mut self) -> ATTR6_W<16> {
        ATTR6_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 7"]
    #[inline(always)]
    #[must_use]
    pub fn attr7(&mut self) -> ATTR7_W<24> {
        ATTR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Along with MPU_MAIR0, provides the memory attribute encodings corresponding to the AttrIndex values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mair1](index.html) module"]
pub struct MAIR1_SPEC;
impl crate::RegisterSpec for MAIR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mair1::R](R) reader structure"]
impl crate::Readable for MAIR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mair1::W](W) writer structure"]
impl crate::Writable for MAIR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAIR1 to value 0"]
impl crate::Resettable for MAIR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
