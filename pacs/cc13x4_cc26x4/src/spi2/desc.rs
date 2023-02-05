#[doc = "Register `DESC` reader"]
pub struct R(crate::R<DESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DESC` writer"]
pub struct W(crate::W<DESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DESC_SPEC>;
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
impl From<crate::W<DESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision of the IP"]
pub type MINREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINREV` writer - 3:0\\]
Minor revision of the IP"]
pub type MINREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision of the IP"]
pub type MAJREV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJREV` writer - 7:4\\]
Major revision of the IP"]
pub type MAJREV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED8` writer - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `FEATUREVER` reader - 15:12\\]
Feature set version for this module instance."]
pub type FEATUREVER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FEATUREVER` writer - 15:12\\]
Feature set version for this module instance."]
pub type FEATUREVER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u8, u8, 4, O>;
#[doc = "Field `MODULEID` reader - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type MODULEID_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MODULEID` writer - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type MODULEID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DESC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of the IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of the IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set version for this module instance."]
    #[inline(always)]
    pub fn featurever(&self) -> FEATUREVER_R {
        FEATUREVER_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    pub fn moduleid(&self) -> MODULEID_R {
        MODULEID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MINREV_W<0> {
        MINREV_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MAJREV_W<4> {
        MAJREV_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set version for this module instance."]
    #[inline(always)]
    #[must_use]
    pub fn featurever(&mut self) -> FEATUREVER_W<12> {
        FEATUREVER_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    #[must_use]
    pub fn moduleid(&mut self) -> MODULEID_W<16> {
        MODULEID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register identifies the peripheral and its exact version.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [desc](index.html) module"]
pub struct DESC_SPEC;
impl crate::RegisterSpec for DESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [desc::R](R) reader structure"]
impl crate::Readable for DESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [desc::W](W) writer structure"]
impl crate::Writable for DESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DESC to value 0x1411_0010"]
impl crate::Resettable for DESC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1411_0010;
}
