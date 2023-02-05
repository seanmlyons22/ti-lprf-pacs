#[doc = "Register `SPICLKGR` reader"]
pub struct R(crate::R<SPICLKGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPICLKGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPICLKGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPICLKGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPICLKGR` writer"]
pub struct W(crate::W<SPICLKGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPICLKGR_SPEC>;
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
impl From<crate::W<SPICLKGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPICLKGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_EN_A {
    #[doc = "8: Enable clock for SPI3"]
    SPI3 = 8,
    #[doc = "4: Enable clock for SPI2"]
    SPI2 = 4,
    #[doc = "2: Enable clock for SPI1"]
    SPI1 = 2,
    #[doc = "1: Enable clock for SPI0"]
    SPI0 = 1,
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
            8 => Some(CLK_EN_A::SPI3),
            4 => Some(CLK_EN_A::SPI2),
            2 => Some(CLK_EN_A::SPI1),
            1 => Some(CLK_EN_A::SPI0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI3`"]
    #[inline(always)]
    pub fn is_spi3(&self) -> bool {
        *self == CLK_EN_A::SPI3
    }
    #[doc = "Checks if the value of the field is `SPI2`"]
    #[inline(always)]
    pub fn is_spi2(&self) -> bool {
        *self == CLK_EN_A::SPI2
    }
    #[doc = "Checks if the value of the field is `SPI1`"]
    #[inline(always)]
    pub fn is_spi1(&self) -> bool {
        *self == CLK_EN_A::SPI1
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == CLK_EN_A::SPI0
    }
}
#[doc = "Field `CLK_EN` writer - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPICLKGR_SPEC, u8, CLK_EN_A, 4, O>;
impl<'a, const O: u8> CLK_EN_W<'a, O> {
    #[doc = "Enable clock for SPI3"]
    #[inline(always)]
    pub fn spi3(self) -> &'a mut W {
        self.variant(CLK_EN_A::SPI3)
    }
    #[doc = "Enable clock for SPI2"]
    #[inline(always)]
    pub fn spi2(self) -> &'a mut W {
        self.variant(CLK_EN_A::SPI2)
    }
    #[doc = "Enable clock for SPI1"]
    #[inline(always)]
    pub fn spi1(self) -> &'a mut W {
        self.variant(CLK_EN_A::SPI1)
    }
    #[doc = "Enable clock for SPI0"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(CLK_EN_A::SPI0)
    }
}
#[doc = "Field `RESERVED4` reader - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPICLKGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AM_CLK_EN` reader - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SPICLKGS.CLK_EN and SPICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_R = crate::FieldReader<u8, AM_CLK_EN_A>;
#[doc = "11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SPICLKGS.CLK_EN and SPICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AM_CLK_EN_A {
    #[doc = "8: Enable clock for SPI3"]
    SPI3 = 8,
    #[doc = "4: Enable clock for SPI2"]
    SPI2 = 4,
    #[doc = "2: Enable clock for SPI1"]
    SPI1 = 2,
    #[doc = "1: Enable clock for SPI0"]
    SPI0 = 1,
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
            8 => Some(AM_CLK_EN_A::SPI3),
            4 => Some(AM_CLK_EN_A::SPI2),
            2 => Some(AM_CLK_EN_A::SPI1),
            1 => Some(AM_CLK_EN_A::SPI0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI3`"]
    #[inline(always)]
    pub fn is_spi3(&self) -> bool {
        *self == AM_CLK_EN_A::SPI3
    }
    #[doc = "Checks if the value of the field is `SPI2`"]
    #[inline(always)]
    pub fn is_spi2(&self) -> bool {
        *self == AM_CLK_EN_A::SPI2
    }
    #[doc = "Checks if the value of the field is `SPI1`"]
    #[inline(always)]
    pub fn is_spi1(&self) -> bool {
        *self == AM_CLK_EN_A::SPI1
    }
    #[doc = "Checks if the value of the field is `SPI0`"]
    #[inline(always)]
    pub fn is_spi0(&self) -> bool {
        *self == AM_CLK_EN_A::SPI0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SPICLKGS.CLK_EN and SPICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AM_CLK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPICLKGR_SPEC, u8, AM_CLK_EN_A, 4, O>;
impl<'a, const O: u8> AM_CLK_EN_W<'a, O> {
    #[doc = "Enable clock for SPI3"]
    #[inline(always)]
    pub fn spi3(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SPI3)
    }
    #[doc = "Enable clock for SPI2"]
    #[inline(always)]
    pub fn spi2(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SPI2)
    }
    #[doc = "Enable clock for SPI1"]
    #[inline(always)]
    pub fn spi1(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SPI1)
    }
    #[doc = "Enable clock for SPI0"]
    #[inline(always)]
    pub fn spi0(self) -> &'a mut W {
        self.variant(AM_CLK_EN_A::SPI0)
    }
}
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPICLKGR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SPICLKGS.CLK_EN and SPICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AM_CLK_EN_R {
        AM_CLK_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, SPICLKGS.CLK_EN and SPICLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AM_CLK_EN_W<8> {
        AM_CLK_EN_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Clock Gate For Run And All Modes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiclkgr](index.html) module"]
pub struct SPICLKGR_SPEC;
impl crate::RegisterSpec for SPICLKGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spiclkgr::R](R) reader structure"]
impl crate::Readable for SPICLKGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spiclkgr::W](W) writer structure"]
impl crate::Writable for SPICLKGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPICLKGR to value 0"]
impl crate::Resettable for SPICLKGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
