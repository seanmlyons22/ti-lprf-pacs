#[doc = "Register `RLAR` reader"]
pub struct R(crate::R<RLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RLAR` writer"]
pub struct W(crate::W<RLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RLAR_SPEC>;
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
impl From<crate::W<RLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Region enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
Region enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, bool, O>;
#[doc = "Field `ATTRINDX` reader - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type ATTRINDX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ATTRINDX` writer - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
pub type ATTRINDX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLAR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED4` reader - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED4` writer - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::BitWriter<'a, u32, RLAR_SPEC, bool, O>;
#[doc = "Field `LIMIT` reader - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LIMIT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LIMIT` writer - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
pub type LIMIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RLAR_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Region enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    pub fn attrindx(&self) -> ATTRINDX_R {
        ATTRINDX_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    pub fn limit(&self) -> LIMIT_R {
        LIMIT_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Region enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Associates a set of attributes in the MPU_MAIR0 and MPU_MAIR1 fields"]
    #[inline(always)]
    #[must_use]
    pub fn attrindx(&mut self) -> ATTRINDX_W<1> {
        ATTRINDX_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the upper inclusive limit of the selected MPU memory region. This value is postfixed with 0x1F to provide the limit address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn limit(&mut self) -> LIMIT_W<5> {
        LIMIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides indirect read and write access to the limit address of the currently selected MPU region\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rlar](index.html) module"]
pub struct RLAR_SPEC;
impl crate::RegisterSpec for RLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rlar::R](R) reader structure"]
impl crate::Readable for RLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rlar::W](W) writer structure"]
impl crate::Writable for RLAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RLAR to value 0"]
impl crate::Resettable for RLAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
