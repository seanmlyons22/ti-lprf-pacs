#[doc = "Register `I2SSTMPSEL0` reader"]
pub struct R(crate::R<I2SSTMPSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SSTMPSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SSTMPSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SSTMPSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SSTMPSEL0` writer"]
pub struct W(crate::W<I2SSTMPSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SSTMPSEL0_SPEC>;
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
impl From<crate::W<I2SSTMPSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SSTMPSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 95"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
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
            121 => Some(EV_A::ALWAYS_ACTIVE),
            0 => Some(EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SSTMPSEL0_SPEC, u8, EV_A, 7, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
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
#[doc = "Output Selection for I2S Subscriber 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sstmpsel0](index.html) module"]
pub struct I2SSTMPSEL0_SPEC;
impl crate::RegisterSpec for I2SSTMPSEL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sstmpsel0::R](R) reader structure"]
impl crate::Readable for I2SSTMPSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sstmpsel0::W](W) writer structure"]
impl crate::Writable for I2SSTMPSEL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SSTMPSEL0 to value 0x5f"]
impl crate::Resettable for I2SSTMPSEL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x5f;
}
