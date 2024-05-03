#[doc = "Register `I2SCLKGDS` reader"]
pub type R = crate::R<I2sclkgdsSpec>;
#[doc = "Register `I2SCLKGDS` writer"]
pub type W = crate::W<I2sclkgdsSpec>;
#[doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<I2sclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "I2S Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sclkgdsSpec;
impl crate::RegisterSpec for I2sclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclkgds::R`](R) reader structure"]
impl crate::Readable for I2sclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sclkgds::W`](W) writer structure"]
impl crate::Writable for I2sclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCLKGDS to value 0"]
impl crate::Resettable for I2sclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
