#[doc = "Register `CLZ` reader"]
pub struct R(crate::R<CLZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLZ` writer"]
pub struct W(crate::W<CLZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLZ_SPEC>;
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
impl From<crate::W<CLZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
pub type VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VALUE` writer - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLZ_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLZ_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Number of leading zero bits in the accumulator: 0x00: 0 leading zeros. 0x01: 1 leading zero. ... 0x28: 40 leading zeros (accumulator value is 0)."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Count Leading Zero\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clz](index.html) module"]
pub struct CLZ_SPEC;
impl crate::RegisterSpec for CLZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clz::R](R) reader structure"]
impl crate::Readable for CLZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clz::W](W) writer structure"]
impl crate::Writable for CLZ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLZ to value 0x28"]
impl crate::Resettable for CLZ_SPEC {
    const RESET_VALUE: Self::Ux = 0x28;
}
