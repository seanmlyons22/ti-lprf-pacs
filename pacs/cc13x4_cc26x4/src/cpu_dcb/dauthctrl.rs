#[doc = "Register `DAUTHCTRL` reader"]
pub struct R(crate::R<DAUTHCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAUTHCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAUTHCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAUTHCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAUTHCTRL` writer"]
pub struct W(crate::W<DAUTHCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAUTHCTRL_SPEC>;
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
impl From<crate::W<DAUTHCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAUTHCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIDENSEL` reader - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
pub type SPIDENSEL_R = crate::BitReader<bool>;
#[doc = "Field `SPIDENSEL` writer - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
pub type SPIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAUTHCTRL_SPEC, bool, O>;
#[doc = "Field `INTSPIDEN` reader - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
pub type INTSPIDEN_R = crate::BitReader<bool>;
#[doc = "Field `INTSPIDEN` writer - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
pub type INTSPIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAUTHCTRL_SPEC, bool, O>;
#[doc = "Field `SPNIDENSEL` reader - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
pub type SPNIDENSEL_R = crate::BitReader<bool>;
#[doc = "Field `SPNIDENSEL` writer - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
pub type SPNIDENSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAUTHCTRL_SPEC, bool, O>;
#[doc = "Field `INTSPNIDEN` reader - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
pub type INTSPNIDEN_R = crate::BitReader<bool>;
#[doc = "Field `INTSPNIDEN` writer - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
pub type INTSPNIDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DAUTHCTRL_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAUTHCTRL_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
    #[inline(always)]
    pub fn spidensel(&self) -> SPIDENSEL_R {
        SPIDENSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
    #[inline(always)]
    pub fn intspiden(&self) -> INTSPIDEN_R {
        INTSPIDEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
    #[inline(always)]
    pub fn spnidensel(&self) -> SPNIDENSEL_R {
        SPNIDENSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
    #[inline(always)]
    pub fn intspniden(&self) -> INTSPNIDEN_R {
        INTSPNIDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Secure invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure invasive debug."]
    #[inline(always)]
    #[must_use]
    pub fn spidensel(&mut self) -> SPIDENSEL_W<0> {
        SPIDENSEL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Secure invasive debug enable. Overrides the external Secure invasive debug authentication Interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn intspiden(&mut self) -> INTSPIDEN_W<1> {
        INTSPIDEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Secure non-invasive debug enable select. Selects between DAUTHCTRL and the external authentication interface for control of Secure non-invasive debug"]
    #[inline(always)]
    #[must_use]
    pub fn spnidensel(&mut self) -> SPNIDENSEL_W<2> {
        SPNIDENSEL_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal Secure non-invasive debug enable. Overrides the external Secure non-invasive debug authentication interface"]
    #[inline(always)]
    #[must_use]
    pub fn intspniden(&mut self) -> INTSPNIDEN_W<3> {
        INTSPNIDEN_W::new(self)
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
#[doc = "This register allows the external authentication interface to be overridden from software.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dauthctrl](index.html) module"]
pub struct DAUTHCTRL_SPEC;
impl crate::RegisterSpec for DAUTHCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dauthctrl::R](R) reader structure"]
impl crate::Readable for DAUTHCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dauthctrl::W](W) writer structure"]
impl crate::Writable for DAUTHCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAUTHCTRL to value 0"]
impl crate::Resettable for DAUTHCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
