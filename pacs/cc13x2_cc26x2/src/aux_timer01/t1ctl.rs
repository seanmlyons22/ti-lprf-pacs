#[doc = "Register `T1CTL` reader"]
pub struct R(crate::R<T1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T1CTL` writer"]
pub struct W(crate::W<T1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1CTL_SPEC>;
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
impl From<crate::W<T1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, T1CTL_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, T1CTL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Timer 1 enable. 0: Disable Timer 1. 1: Enable Timer 1. The counter restarts from 0 when you enable Timer 1."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1ctl](index.html) module"]
pub struct T1CTL_SPEC;
impl crate::RegisterSpec for T1CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t1ctl::R](R) reader structure"]
impl crate::Readable for T1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t1ctl::W](W) writer structure"]
impl crate::Writable for T1CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1CTL to value 0"]
impl crate::Resettable for T1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
