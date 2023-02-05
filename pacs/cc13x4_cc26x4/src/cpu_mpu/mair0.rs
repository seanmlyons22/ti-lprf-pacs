#[doc = "Register `MAIR0` reader"]
pub struct R(crate::R<MAIR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAIR0` writer"]
pub struct W(crate::W<MAIR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAIR0_SPEC>;
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
impl From<crate::W<MAIR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAIR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTR0` reader - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type ATTR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR0` writer - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
pub type ATTR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR1` reader - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type ATTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR1` writer - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
pub type ATTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR2` reader - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type ATTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR2` writer - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
pub type ATTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ATTR3` reader - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type ATTR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTR3` writer - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
pub type ATTR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAIR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    pub fn attr0(&self) -> ATTR0_R {
        ATTR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    pub fn attr3(&self) -> ATTR3_R {
        ATTR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Memory attribute encoding for MPU regions with an AttrIndex of 0"]
    #[inline(always)]
    #[must_use]
    pub fn attr0(&mut self) -> ATTR0_W<0> {
        ATTR0_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Memory attribute encoding for MPU regions with an AttrIndex of 1"]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<8> {
        ATTR1_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Memory attribute encoding for MPU regions with an AttrIndex of 2"]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<16> {
        ATTR2_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Memory attribute encoding for MPU regions with an AttrIndex of 3"]
    #[inline(always)]
    #[must_use]
    pub fn attr3(&mut self) -> ATTR3_W<24> {
        ATTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Along with MPU_MAIR1, provides the memory attribute encodings corresponding to the AttrIndex values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mair0](index.html) module"]
pub struct MAIR0_SPEC;
impl crate::RegisterSpec for MAIR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mair0::R](R) reader structure"]
impl crate::Readable for MAIR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mair0::W](W) writer structure"]
impl crate::Writable for MAIR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAIR0 to value 0"]
impl crate::Resettable for MAIR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
