#[doc = "Register `ALTCTRL` reader"]
pub struct R(crate::R<ALTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALTCTRL` writer"]
pub struct W(crate::W<ALTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALTCTRL_SPEC>;
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
impl From<crate::W<ALTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASEPTR` reader - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
pub type BASEPTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BASEPTR` writer - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
pub type BASEPTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALTCTRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
    #[inline(always)]
    pub fn baseptr(&self) -> BASEPTR_R {
        BASEPTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register shows the base address for the alternate data structures and is calculated by module, thus read only"]
    #[inline(always)]
    #[must_use]
    pub fn baseptr(&mut self) -> BASEPTR_W<0> {
        BASEPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Control Data Base Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altctrl](index.html) module"]
pub struct ALTCTRL_SPEC;
impl crate::RegisterSpec for ALTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altctrl::R](R) reader structure"]
impl crate::Readable for ALTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [altctrl::W](W) writer structure"]
impl crate::Writable for ALTCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALTCTRL to value 0x0200"]
impl crate::Resettable for ALTCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
