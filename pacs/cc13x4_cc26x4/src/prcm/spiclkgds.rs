#[doc = "Register `SPICLKGDS` reader"]
pub struct R(crate::R<SPICLKGDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPICLKGDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPICLKGDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPICLKGDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPICLKGDS` writer"]
pub struct W(crate::W<SPICLKGDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPICLKGDS_SPEC>;
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
impl From<crate::W<SPICLKGDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPICLKGDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLK_EN` reader - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by SPICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_R = crate::FieldReader<u8, CLK_EN_A>;
#[doc = "3:0\\]
0: Disable clock 1: Enable clock Can be forced on by SPICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
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
0: Disable clock 1: Enable clock Can be forced on by SPICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type CLK_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPICLKGDS_SPEC, u8, CLK_EN_A, 4, O>;
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
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPICLKGDS_SPEC, u32, u32, 28, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by SPICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
0: Disable clock 1: Enable clock Can be forced on by SPICLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<0> {
        CLK_EN_W::new(self)
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
#[doc = "SPI Clock Gate For Deep Sleep Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spiclkgds](index.html) module"]
pub struct SPICLKGDS_SPEC;
impl crate::RegisterSpec for SPICLKGDS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spiclkgds::R](R) reader structure"]
impl crate::Readable for SPICLKGDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spiclkgds::W](W) writer structure"]
impl crate::Writable for SPICLKGDS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPICLKGDS to value 0"]
impl crate::Resettable for SPICLKGDS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
