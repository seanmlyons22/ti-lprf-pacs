#[doc = "Register `I2SCLKGS` reader"]
pub struct R(crate::R<I2SCLKGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCLKGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCLKGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCLKGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCLKGS` writer"]
pub struct W(crate::W<I2SCLKGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCLKGS_SPEC>;
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
impl From<crate::W<I2SCLKGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCLKGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SCLKGS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Gate For Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sclkgs](index.html) module"]
pub struct I2SCLKGS_SPEC;
impl crate::RegisterSpec for I2SCLKGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sclkgs::R](R) reader structure"]
impl crate::Readable for I2SCLKGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sclkgs::W](W) writer structure"]
impl crate::Writable for I2SCLKGS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCLKGS to value 0"]
impl crate::Resettable for I2SCLKGS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
