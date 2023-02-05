#[doc = "Register `UDMACH14SSEL` reader"]
pub struct R(crate::R<UDMACH14SSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDMACH14SSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDMACH14SSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDMACH14SSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDMACH14SSEL` writer"]
pub struct W(crate::W<UDMACH14SSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDMACH14SSEL_SPEC>;
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
impl From<crate::W<UDMACH14SSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDMACH14SSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type EV_R = crate::FieldReader<u32, EV_A>;
#[doc = "31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EV_A {
    #[doc = "1: AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    AON_PROG0 = 1,
}
impl From<EV_A> for u32 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            1 => Some(EV_A::AON_PROG0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AON_PROG0`"]
    #[inline(always)]
    pub fn is_aon_prog0(&self) -> bool {
        *self == EV_A::AON_PROG0
    }
}
#[doc = "Field `EV` writer - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDMACH14SSEL_SPEC, u32, EV_A, 32, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "AON programmable event 0. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG0_EV"]
    #[inline(always)]
    pub fn aon_prog0(self) -> &'a mut W {
        self.variant(EV_A::AON_PROG0)
    }
}
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach14ssel](index.html) module"]
pub struct UDMACH14SSEL_SPEC;
impl crate::RegisterSpec for UDMACH14SSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udmach14ssel::R](R) reader structure"]
impl crate::Readable for UDMACH14SSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udmach14ssel::W](W) writer structure"]
impl crate::Writable for UDMACH14SSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDMACH14SSEL to value 0x01"]
impl crate::Resettable for UDMACH14SSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
