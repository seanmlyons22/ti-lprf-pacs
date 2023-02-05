#[doc = "Register `TIME` reader"]
pub struct R(crate::R<TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME` writer"]
pub struct W(crate::W<TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_SPEC>;
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
impl From<crate::W<TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBSEC_H` reader - 15:0\\]
Returns the upper halfword of SUBSEC register."]
pub type SUBSEC_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SUBSEC_H` writer - 15:0\\]
Returns the upper halfword of SUBSEC register."]
pub type SUBSEC_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_SPEC, u16, u16, 16, O>;
#[doc = "Field `SEC_L` reader - 31:16\\]
Returns the lower halfword of SEC register."]
pub type SEC_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SEC_L` writer - 31:16\\]
Returns the lower halfword of SEC register."]
pub type SEC_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIME_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Returns the upper halfword of SUBSEC register."]
    #[inline(always)]
    pub fn subsec_h(&self) -> SUBSEC_H_R {
        SUBSEC_H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Returns the lower halfword of SEC register."]
    #[inline(always)]
    pub fn sec_l(&self) -> SEC_L_R {
        SEC_L_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Returns the upper halfword of SUBSEC register."]
    #[inline(always)]
    #[must_use]
    pub fn subsec_h(&mut self) -> SUBSEC_H_W<0> {
        SUBSEC_H_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Returns the lower halfword of SEC register."]
    #[inline(always)]
    #[must_use]
    pub fn sec_l(&mut self) -> SEC_L_W<16> {
        SEC_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Current Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](index.html) module"]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time::R](R) reader structure"]
impl crate::Readable for TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time::W](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
