#[doc = "Register `I2SBCLKSEL` reader"]
pub struct R(crate::R<I2SBCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SBCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SBCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SBCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SBCLKSEL` writer"]
pub struct W(crate::W<I2SBCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SBCLKSEL_SPEC>;
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
impl From<crate::W<I2SBCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SBCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC` reader - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SRC_R = crate::BitReader<bool>;
#[doc = "Field `SRC` writer - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, I2SBCLKSEL_SPEC, bool, O>;
#[doc = "Field `SPARE` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPARE` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SBCLKSEL_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SRC_W<0> {
        SRC_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<1> {
        SPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sbclksel](index.html) module"]
pub struct I2SBCLKSEL_SPEC;
impl crate::RegisterSpec for I2SBCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sbclksel::R](R) reader structure"]
impl crate::Readable for I2SBCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sbclksel::W](W) writer structure"]
impl crate::Writable for I2SBCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SBCLKSEL to value 0"]
impl crate::Resettable for I2SBCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
