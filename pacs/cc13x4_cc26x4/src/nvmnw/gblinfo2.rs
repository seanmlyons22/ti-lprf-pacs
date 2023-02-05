#[doc = "Register `GBLINFO2` reader"]
pub struct R(crate::R<GBLINFO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GBLINFO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GBLINFO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GBLINFO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GBLINFO2` writer"]
pub struct W(crate::W<GBLINFO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GBLINFO2_SPEC>;
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
impl From<crate::W<GBLINFO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GBLINFO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAREGISTERS` reader - 3:0\\]
Number of data registers present."]
pub type DATAREGISTERS_R = crate::FieldReader<u8, DATAREGISTERS_A>;
#[doc = "3:0\\]
Number of data registers present.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATAREGISTERS_A {
    #[doc = "8: Maximum value of DATAREGISTERS"]
    MAXIMUM = 8,
    #[doc = "1: Minimum value of DATAREGISTERS"]
    MINIMUM = 1,
}
impl From<DATAREGISTERS_A> for u8 {
    #[inline(always)]
    fn from(variant: DATAREGISTERS_A) -> Self {
        variant as _
    }
}
impl DATAREGISTERS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATAREGISTERS_A> {
        match self.bits {
            8 => Some(DATAREGISTERS_A::MAXIMUM),
            1 => Some(DATAREGISTERS_A::MINIMUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == DATAREGISTERS_A::MAXIMUM
    }
    #[doc = "Checks if the value of the field is `MINIMUM`"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == DATAREGISTERS_A::MINIMUM
    }
}
#[doc = "Field `DATAREGISTERS` writer - 3:0\\]
Number of data registers present."]
pub type DATAREGISTERS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GBLINFO2_SPEC, u8, DATAREGISTERS_A, 4, O>;
impl<'a, const O: u8> DATAREGISTERS_W<'a, O> {
    #[doc = "Maximum value of DATAREGISTERS"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(DATAREGISTERS_A::MAXIMUM)
    }
    #[doc = "Minimum value of DATAREGISTERS"]
    #[inline(always)]
    pub fn minimum(self) -> &'a mut W {
        self.variant(DATAREGISTERS_A::MINIMUM)
    }
}
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GBLINFO2_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Number of data registers present."]
    #[inline(always)]
    pub fn dataregisters(&self) -> DATAREGISTERS_R {
        DATAREGISTERS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Number of data registers present."]
    #[inline(always)]
    #[must_use]
    pub fn dataregisters(&mut self) -> DATAREGISTERS_W<0> {
        DATAREGISTERS_W::new(self)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Info 2 Register Read only register detailing information about the number of data registers present.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gblinfo2](index.html) module"]
pub struct GBLINFO2_SPEC;
impl crate::RegisterSpec for GBLINFO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gblinfo2::R](R) reader structure"]
impl crate::Readable for GBLINFO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gblinfo2::W](W) writer structure"]
impl crate::Writable for GBLINFO2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GBLINFO2 to value 0x04"]
impl crate::Resettable for GBLINFO2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
