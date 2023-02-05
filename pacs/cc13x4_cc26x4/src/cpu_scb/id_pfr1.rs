#[doc = "Register `ID_PFR1` reader"]
pub struct R(crate::R<ID_PFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID_PFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID_PFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID_PFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID_PFR1` writer"]
pub struct W(crate::W<ID_PFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID_PFR1_SPEC>;
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
impl From<crate::W<ID_PFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID_PFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECURITY` reader - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
pub type SECURITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY` writer - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
pub type SECURITY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `MICROCONTROLLER_PROGRAMMERS_MODEL` reader - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
pub type MICROCONTROLLER_PROGRAMMERS_MODEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MICROCONTROLLER_PROGRAMMERS_MODEL` writer - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
pub type MICROCONTROLLER_PROGRAMMERS_MODEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ID_PFR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID_PFR1_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    pub fn microcontroller_programmers_model(&self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_R {
        MICROCONTROLLER_PROGRAMMERS_MODEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Security. Identifies whether the Security Extension is implemented"]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<4> {
        SECURITY_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Microcontroller programmer's model 0x0: Not supported 0x2: Two-stack support"]
    #[inline(always)]
    #[must_use]
    pub fn microcontroller_programmers_model(&mut self) -> MICROCONTROLLER_PROGRAMMERS_MODEL_W<8> {
        MICROCONTROLLER_PROGRAMMERS_MODEL_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor Feature 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id_pfr1](index.html) module"]
pub struct ID_PFR1_SPEC;
impl crate::RegisterSpec for ID_PFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id_pfr1::R](R) reader structure"]
impl crate::Readable for ID_PFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id_pfr1::W](W) writer structure"]
impl crate::Writable for ID_PFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ID_PFR1 to value 0x0210"]
impl crate::Resettable for ID_PFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0210;
}
