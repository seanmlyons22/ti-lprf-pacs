#[doc = "Register `IABR1` reader"]
pub struct R(crate::R<IABR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IABR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IABR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IABR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IABR1` writer"]
pub struct W(crate::W<IABR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IABR1_SPEC>;
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
impl From<crate::W<IABR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IABR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE` reader - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ACTIVE` writer - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
pub type ACTIVE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IABR1_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IABR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
For ACTIVE\\[m\\]
in NVIC_IABR*n, indicates the active state for interrupt 32*n+m"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<0> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iabr1](index.html) module"]
pub struct IABR1_SPEC;
impl crate::RegisterSpec for IABR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iabr1::R](R) reader structure"]
impl crate::Readable for IABR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iabr1::W](W) writer structure"]
impl crate::Writable for IABR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IABR1 to value 0"]
impl crate::Resettable for IABR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
