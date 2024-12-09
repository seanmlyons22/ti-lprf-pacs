#[doc = "Register `I2CCLKGR` reader"]
pub type R = crate::R<I2cclkgrSpec>;
#[doc = "Register `I2CCLKGR` writer"]
pub type W = crate::W<I2cclkgrSpec>;
#[doc = "1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkEn {
    #[doc = "2: Enable clock for I2C1"]
    I2c1 = 2,
    #[doc = "1: Enable clock for I2C0"]
    I2c0 = 1,
}
impl From<ClkEn> for u8 {
    #[inline(always)]
    fn from(variant: ClkEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkEn {
    type Ux = u8;
}
impl crate::IsEnum for ClkEn {}
#[doc = "Field `CLK_EN` reader - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader<ClkEn>;
impl ClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkEn> {
        match self.bits {
            2 => Some(ClkEn::I2c1),
            1 => Some(ClkEn::I2c0),
            _ => None,
        }
    }
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == ClkEn::I2c1
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == ClkEn::I2c0
    }
}
#[doc = "Field `CLK_EN` writer - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkEn>;
impl<'a, REG> ClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::I2c1)
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkEn::I2c0)
    }
}
#[doc = "Field `RESERVED2` reader - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader;
#[doc = "9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AmClkEn {
    #[doc = "2: Enable clock for I2C1"]
    I2c1 = 2,
    #[doc = "1: Enable clock for I2C0"]
    I2c0 = 1,
}
impl From<AmClkEn> for u8 {
    #[inline(always)]
    fn from(variant: AmClkEn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AmClkEn {
    type Ux = u8;
}
impl crate::IsEnum for AmClkEn {}
#[doc = "Field `AM_CLK_EN` reader - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnR = crate::FieldReader<AmClkEn>;
impl AmClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AmClkEn> {
        match self.bits {
            2 => Some(AmClkEn::I2c1),
            1 => Some(AmClkEn::I2c0),
            _ => None,
        }
    }
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == AmClkEn::I2c1
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == AmClkEn::I2c0
    }
}
#[doc = "Field `AM_CLK_EN` writer - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type AmClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2, AmClkEn>;
impl<'a, REG> AmClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Enable clock for I2C1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::I2c1)
    }
    #[doc = "Enable clock for I2C0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut crate::W<REG> {
        self.variant(AmClkEn::I2c0)
    }
}
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn am_clk_en(&self) -> AmClkEnR {
        AmClkEnR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disable clock 1: Enable clock Can be forced on by AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<I2cclkgrSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
0: No force 1: Force clock on for all modes (Run, Sleep and Deep Sleep) Overrides CLK_EN, I2CCLKGS.CLK_EN and I2CCLKGDS.CLK_EN when enabled. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn am_clk_en(&mut self) -> AmClkEnW<I2cclkgrSpec> {
        AmClkEnW::new(self, 8)
    }
}
#[doc = "I2C Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cclkgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cclkgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cclkgrSpec;
impl crate::RegisterSpec for I2cclkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cclkgr::R`](R) reader structure"]
impl crate::Readable for I2cclkgrSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cclkgr::W`](W) writer structure"]
impl crate::Writable for I2cclkgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2CCLKGR to value 0"]
impl crate::Resettable for I2cclkgrSpec {
    const RESET_VALUE: u32 = 0;
}
