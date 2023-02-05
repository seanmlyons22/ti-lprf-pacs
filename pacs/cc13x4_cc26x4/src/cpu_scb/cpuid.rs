#[doc = "Register `CPUID` reader"]
pub struct R(crate::R<CPUID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUID` writer"]
pub struct W(crate::W<CPUID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUID_SPEC>;
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
impl From<crate::W<CPUID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REVISION` reader - 3:0\\]
Implementation defined revision number."]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` writer - 3:0\\]
Implementation defined revision number."]
pub type REVISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUID_SPEC, u8, u8, 4, O>;
#[doc = "Field `PARTNO` reader - 15:4\\]
Number of processor within family."]
pub type PARTNO_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PARTNO` writer - 15:4\\]
Number of processor within family."]
pub type PARTNO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUID_SPEC, u16, u16, 12, O>;
#[doc = "Field `CONSTANT` reader - 19:16\\]
Reads as 0xF"]
pub type CONSTANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CONSTANT` writer - 19:16\\]
Reads as 0xF"]
pub type CONSTANT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUID_SPEC, u8, u8, 4, O>;
#[doc = "Field `VARIANT` reader - 23:20\\]
Implementation defined variant number."]
pub type VARIANT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VARIANT` writer - 23:20\\]
Implementation defined variant number."]
pub type VARIANT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUID_SPEC, u8, u8, 4, O>;
#[doc = "Field `IMPLEMENTER` reader - 31:24\\]
Implementor code."]
pub type IMPLEMENTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IMPLEMENTER` writer - 31:24\\]
Implementor code."]
pub type IMPLEMENTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUID_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    pub fn constant(&self) -> CONSTANT_R {
        CONSTANT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    pub fn variant(&self) -> VARIANT_R {
        VARIANT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    pub fn implementer(&self) -> IMPLEMENTER_R {
        IMPLEMENTER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Implementation defined revision number."]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<0> {
        REVISION_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Number of processor within family."]
    #[inline(always)]
    #[must_use]
    pub fn partno(&mut self) -> PARTNO_W<4> {
        PARTNO_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Reads as 0xF"]
    #[inline(always)]
    #[must_use]
    pub fn constant(&mut self) -> CONSTANT_W<16> {
        CONSTANT_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Implementation defined variant number."]
    #[inline(always)]
    #[must_use]
    pub fn variant(&mut self) -> VARIANT_W<20> {
        VARIANT_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Implementor code."]
    #[inline(always)]
    #[must_use]
    pub fn implementer(&mut self) -> IMPLEMENTER_W<24> {
        IMPLEMENTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPUID Base This register determines the ID number of the processor core, the version number of the processor core and the implementation details of the processor core.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpuid](index.html) module"]
pub struct CPUID_SPEC;
impl crate::RegisterSpec for CPUID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpuid::R](R) reader structure"]
impl crate::Readable for CPUID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpuid::W](W) writer structure"]
impl crate::Writable for CPUID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUID to value 0x410f_d214"]
impl crate::Resettable for CPUID_SPEC {
    const RESET_VALUE: Self::Ux = 0x410f_d214;
}
