#[doc = "Register `DACSMPLCFG0` reader"]
pub struct R(crate::R<DACSMPLCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACSMPLCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACSMPLCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACSMPLCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACSMPLCFG0` writer"]
pub struct W(crate::W<DACSMPLCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACSMPLCFG0_SPEC>;
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
impl From<crate::W<DACSMPLCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACSMPLCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - 5:0\\]
Clock division. AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE divided by (CLKDIV + 1) determines the sample clock base frequency. 0: Divide by 1. 1: Divide by 2. ... 63: Divide by 64."]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - 5:0\\]
Clock division. AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE divided by (CLKDIV + 1) determines the sample clock base frequency. 0: Divide by 1. 1: Divide by 2. ... 63: Divide by 64."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACSMPLCFG0_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DACSMPLCFG0_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Clock division. AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE divided by (CLKDIV + 1) determines the sample clock base frequency. 0: Divide by 1. 1: Divide by 2. ... 63: Divide by 64."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Clock division. AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE divided by (CLKDIV + 1) determines the sample clock base frequency. 0: Divide by 1. 1: Divide by 2. ... 63: Divide by 64."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Sample Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacsmplcfg0](index.html) module"]
pub struct DACSMPLCFG0_SPEC;
impl crate::RegisterSpec for DACSMPLCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacsmplcfg0::R](R) reader structure"]
impl crate::Readable for DACSMPLCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacsmplcfg0::W](W) writer structure"]
impl crate::Writable for DACSMPLCFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACSMPLCFG0 to value 0"]
impl crate::Resettable for DACSMPLCFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
