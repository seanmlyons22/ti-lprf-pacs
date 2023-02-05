#[doc = "Register `I2CCLKGR` reader"]
pub struct R(crate::R<I2CCLKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CCLKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CCLKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CCLKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CCLKGR` writer"]
pub struct W(crate::W<I2CCLKGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CCLKGR_SPEC>;
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
impl From<crate::W<I2CCLKGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CCLKGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "2: Enable clock for I2C1"]
    I2C1 = 2,
    #[doc = "1: Enable clock for I2C0"]
    I2C0 = 1,
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
            2 => Some(CLK_EN_A::I2C1),
            1 => Some(CLK_EN_A::I2C0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == CLK_EN_A::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == CLK_EN_A::I2C0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2CCLKGR_SPEC, u8, CLK_EN_A, 2, O>;
impl<'a, const O: u8> CLK_EN_W<'a, O> {
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(CLK_EN_A::I2C1)
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(CLK_EN_A::I2C0)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2CCLKGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `AM_CLK_EN` reader - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_R = crate::FieldReader<u8, AM_CLK_EN_A>;
#[doc = "9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM_CLK_EN_A {
    #[doc = "2: Enable clock for I2C1"]
    I2C1 = 2,
    #[doc = "1: Enable clock for I2C0"]
    I2C0 = 1,
}
impl From<AM_CLK_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: AM_CLK_EN_A) -> Self {
        variant as _
    }
}
impl AM_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AM_CLK_EN_A> {
        match self.bits {
            2 => Some(AM_CLK_EN_A::I2C1),
            1 => Some(AM_CLK_EN_A::I2C0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == AM_CLK_EN_A::I2C1
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == AM_CLK_EN_A::I2C0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2CCLKGR_SPEC, u8, AM_CLK_EN_A, 2, O>;
impl<'a, const O: u8> AM_CLK_EN_W<'a, O> {
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::I2C1)
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::I2C0)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2CCLKGR_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AM_CLK_EN_R {
        AM_CLK_EN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AM_CLK_EN_W<8> {
        AM_CLK_EN_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cclkgr](index.html) module"]
pub struct I2CCLKGR_SPEC;
impl crate::RegisterSpec for I2CCLKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cclkgr::R](R) reader structure"]
impl crate::Readable for I2CCLKGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cclkgr::W](W) writer structure"]
impl crate::Writable for I2CCLKGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2CCLKGR to value 0"]
impl crate::Resettable for I2CCLKGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
