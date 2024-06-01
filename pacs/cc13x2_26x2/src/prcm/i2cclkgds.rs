#[doc = "Register `I2CCLKGDS` reader"]
pub type R = crate::R<I2cclkgdsSpec>;
#[doc = "Register `I2CCLKGDS` writer"]
pub type W = crate::W<I2cclkgdsSpec>;
#[doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by I2CCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by I2CCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by I2CCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare1(&self) -> Spare1R {
        Spare1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by I2CCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<I2cclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> Spare1W<I2cclkgdsSpec> {
        Spare1W::new(self, 1)
    }
}
#[doc = "I2C Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cclkgdsSpec;
impl crate::RegisterSpec for I2cclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cclkgds::R`](R) reader structure"]
impl crate::Readable for I2cclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cclkgds::W`](W) writer structure"]
impl crate::Writable for I2cclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2CCLKGDS to value 0"]
impl crate::Resettable for I2cclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
