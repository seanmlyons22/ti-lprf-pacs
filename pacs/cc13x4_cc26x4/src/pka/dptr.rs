#[doc = "Register `DPTR` reader"]
pub struct R(crate::R<DPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPTR` writer"]
pub struct W(crate::W<DPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPTR_SPEC>;
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
impl From<crate::W<DPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPTR` reader - 10:0\\]
This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type DPTR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DPTR` writer - 10:0\\]
This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type DPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPTR_SPEC, u16, u16, 11, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Set to zero on write, ignore on read"]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Set to zero on write, ignore on read"]
pub type RESERVED11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DPTR_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn dptr(&self) -> DPTR_R {
        DPTR_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    #[must_use]
    pub fn dptr(&mut self) -> DPTR_W<0> {
        DPTR_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Set to zero on write, ignore on read"]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA Vector D Address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dptr](index.html) module"]
pub struct DPTR_SPEC;
impl crate::RegisterSpec for DPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dptr::R](R) reader structure"]
impl crate::Readable for DPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dptr::W](W) writer structure"]
impl crate::Writable for DPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DPTR to value 0"]
impl crate::Resettable for DPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
