#[doc = "Register `SSICLKGS` reader"]
pub struct R(crate::R<SSICLKGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSICLKGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSICLKGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSICLKGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSICLKGS` writer"]
pub struct W(crate::W<SSICLKGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSICLKGS_SPEC>;
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
impl From<crate::W<SSICLKGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSICLKGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by SSICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by SSICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "1: Enable clock for SSI0"]
    SSI0 = 1,
}
impl From<CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_EN_A) -> Self {
        variant as _
    }
}
impl CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_EN_A> {
        match self.bits {
            1 => Some(CLK_EN_A::SSI0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SSI0`"]
    #[inline(always)]
    pub fn is_ssi0(&self) -> bool {
        *self == CLK_EN_A::SSI0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by SSICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
pub type CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICLKGS_SPEC, u8, CLK_EN_A, 2, O>;
impl<'a, const O: u8> CLK_EN_W<'a, O> {
    #[doc = "Enable clock for SSI0"]
    #[inline(always)]
    pub fn ssi0(self) -> &'a mut W {
        self.variant(CLK_EN_A::SSI0)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SSICLKGS_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by SSICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by SSICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written NOTE: MSB is reserverd Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SSI Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssiclkgs](index.html) module"]
pub struct SSICLKGS_SPEC;
impl crate::RegisterSpec for SSICLKGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ssiclkgs::R](R) reader structure"]
impl crate::Readable for SSICLKGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssiclkgs::W](W) writer structure"]
impl crate::Writable for SSICLKGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSICLKGS to value 0"]
impl crate::Resettable for SSICLKGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
