#[doc = "Register `DEVARCH` reader"]
pub struct R(crate::R<DEVARCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVARCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVARCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVARCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVARCH` writer"]
pub struct W(crate::W<DEVARCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVARCH_SPEC>;
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
impl From<crate::W<DEVARCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVARCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARCHPART` reader - 11:0\\]
Defines the architecture of the component"]
pub type ARCHPART_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARCHPART` writer - 11:0\\]
Defines the architecture of the component"]
pub type ARCHPART_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVARCH_SPEC, u16, u16, 12, O>;
#[doc = "Field `ARCHVER` reader - 15:12\\]
Defines the architecture version of the component"]
pub type ARCHVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARCHVER` writer - 15:12\\]
Defines the architecture version of the component"]
pub type ARCHVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVARCH_SPEC, u8, u8, 4, O>;
#[doc = "Field `REVISION` reader - 19:16\\]
Defines the architecture revision of the component"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` writer - 19:16\\]
Defines the architecture revision of the component"]
pub type REVISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVARCH_SPEC, u8, u8, 4, O>;
#[doc = "Field `PRESENT` reader - 20:20\\]
Defines that the DEVARCH register is present"]
pub type PRESENT_R = crate::BitReader<bool>;
#[doc = "Field `PRESENT` writer - 20:20\\]
Defines that the DEVARCH register is present"]
pub type PRESENT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEVARCH_SPEC, bool, O>;
#[doc = "Field `ARCHITECT` reader - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
pub type ARCHITECT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARCHITECT` writer - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
pub type ARCHITECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEVARCH_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the architecture of the component"]
    #[inline(always)]
    pub fn archpart(&self) -> ARCHPART_R {
        ARCHPART_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Defines the architecture version of the component"]
    #[inline(always)]
    pub fn archver(&self) -> ARCHVER_R {
        ARCHVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision of the component"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Defines that the DEVARCH register is present"]
    #[inline(always)]
    pub fn present(&self) -> PRESENT_R {
        PRESENT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
    #[inline(always)]
    pub fn architect(&self) -> ARCHITECT_R {
        ARCHITECT_R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Defines the architecture of the component"]
    #[inline(always)]
    #[must_use]
    pub fn archpart(&mut self) -> ARCHPART_W<0> {
        ARCHPART_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Defines the architecture version of the component"]
    #[inline(always)]
    #[must_use]
    pub fn archver(&mut self) -> ARCHVER_W<12> {
        ARCHVER_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the architecture revision of the component"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<16> {
        REVISION_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Defines that the DEVARCH register is present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PRESENT_W<20> {
        PRESENT_W::new(self)
    }
    #[doc = "Bits 21:31 - 31:21\\]
Defines the architect of the component. Bits \\[31:28\\]
are the JEP106 continuation code (JEP106 bank ID, minus 1) and bits \\[27:21\\]
are the JEP106 ID code."]
    #[inline(always)]
    #[must_use]
    pub fn architect(&mut self) -> ARCHITECT_W<21> {
        ARCHITECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides CoreSight discovery information for the DWT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devarch](index.html) module"]
pub struct DEVARCH_SPEC;
impl crate::RegisterSpec for DEVARCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devarch::R](R) reader structure"]
impl crate::Readable for DEVARCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devarch::W](W) writer structure"]
impl crate::Writable for DEVARCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEVARCH to value 0"]
impl crate::Resettable for DEVARCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
