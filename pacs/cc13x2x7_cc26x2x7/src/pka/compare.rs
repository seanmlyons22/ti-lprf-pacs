#[doc = "Register `COMPARE` reader"]
pub struct R(crate::R<COMPARE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMPARE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMPARE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMPARE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMPARE` writer"]
pub struct W(crate::W<COMPARE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMPARE_SPEC>;
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
impl From<crate::W<COMPARE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMPARE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A_EQUALS_B` reader - 0:0\\]
Vector_A is equal to Vector_B"]
pub type A_EQUALS_B_R = crate::BitReader<bool>;
#[doc = "Field `A_EQUALS_B` writer - 0:0\\]
Vector_A is equal to Vector_B"]
pub type A_EQUALS_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPARE_SPEC, bool, O>;
#[doc = "Field `A_LESS_THAN_B` reader - 1:1\\]
Vector_A is less than Vector_B"]
pub type A_LESS_THAN_B_R = crate::BitReader<bool>;
#[doc = "Field `A_LESS_THAN_B` writer - 1:1\\]
Vector_A is less than Vector_B"]
pub type A_LESS_THAN_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPARE_SPEC, bool, O>;
#[doc = "Field `A_GREATER_THAN_B` reader - 2:2\\]
Vector_A is greater than Vector_B"]
pub type A_GREATER_THAN_B_R = crate::BitReader<bool>;
#[doc = "Field `A_GREATER_THAN_B` writer - 2:2\\]
Vector_A is greater than Vector_B"]
pub type A_GREATER_THAN_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMPARE_SPEC, bool, O>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Ignore on read"]
pub type RESERVED3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Ignore on read"]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMPARE_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    pub fn a_equals_b(&self) -> A_EQUALS_B_R {
        A_EQUALS_B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    pub fn a_less_than_b(&self) -> A_LESS_THAN_B_R {
        A_LESS_THAN_B_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    pub fn a_greater_than_b(&self) -> A_GREATER_THAN_B_R {
        A_GREATER_THAN_B_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Vector_A is equal to Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_equals_b(&mut self) -> A_EQUALS_B_W<0> {
        A_EQUALS_B_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Vector_A is less than Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_less_than_b(&mut self) -> A_LESS_THAN_B_W<1> {
        A_LESS_THAN_B_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Vector_A is greater than Vector_B"]
    #[inline(always)]
    #[must_use]
    pub fn a_greater_than_b(&mut self) -> A_GREATER_THAN_B_W<2> {
        A_GREATER_THAN_B_W::new(self)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Ignore on read"]
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
#[doc = "PKA compare result This register provides the result of a basic PKCP compare operation. It is updated when the FUNCTION.RUN bit is reset at the end of that operation. Status after a complex sequencer operation is unknown\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compare](index.html) module"]
pub struct COMPARE_SPEC;
impl crate::RegisterSpec for COMPARE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [compare::R](R) reader structure"]
impl crate::Readable for COMPARE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [compare::W](W) writer structure"]
impl crate::Writable for COMPARE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMPARE to value 0x01"]
impl crate::Resettable for COMPARE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
