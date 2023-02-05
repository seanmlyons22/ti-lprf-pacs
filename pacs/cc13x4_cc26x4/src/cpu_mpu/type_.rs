#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TYPE` writer"]
pub struct W(crate::W<TYPE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TYPE_SPEC>;
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
impl From<crate::W<TYPE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TYPE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEPARATE` reader - 0:0\\]
Indicates support for separate instructions and data address regions"]
pub type SEPARATE_R = crate::BitReader<bool>;
#[doc = "Field `SEPARATE` writer - 0:0\\]
Indicates support for separate instructions and data address regions"]
pub type SEPARATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TYPE_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TYPE_SPEC, u8, u8, 7, O>;
#[doc = "Field `DREGION` reader - 15:8\\]
Number of regions supported by the MPU"]
pub type DREGION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DREGION` writer - 15:8\\]
Number of regions supported by the MPU"]
pub type DREGION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TYPE_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TYPE_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates support for separate instructions and data address regions"]
    #[inline(always)]
    pub fn separate(&self) -> SEPARATE_R {
        SEPARATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of regions supported by the MPU"]
    #[inline(always)]
    pub fn dregion(&self) -> DREGION_R {
        DREGION_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates support for separate instructions and data address regions"]
    #[inline(always)]
    #[must_use]
    pub fn separate(&mut self) -> SEPARATE_W<0> {
        SEPARATE_W::new(self)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of regions supported by the MPU"]
    #[inline(always)]
    #[must_use]
    pub fn dregion(&mut self) -> DREGION_W<8> {
        DREGION_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU supports\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [type_::W](W) writer structure"]
impl crate::Writable for TYPE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TYPE to value 0x0800"]
impl crate::Resettable for TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
