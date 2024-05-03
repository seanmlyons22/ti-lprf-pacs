#[doc = "Register `GPIOCLKGDS` reader"]
pub type R = crate::R<GpioclkgdsSpec>;
#[doc = "Register `GPIOCLKGDS` writer"]
pub type W = crate::W<GpioclkgdsSpec>;
#[doc = "Field `CLK_EN` reader - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by GPIOCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by GPIOCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by GPIOCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Disable clock 1: Enable clock Can be forced on by GPIOCLKGR.AM_CLK_EN For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<GpioclkgdsSpec> {
        ClkEnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<GpioclkgdsSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "GPIO Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioclkgds::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioclkgds::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioclkgdsSpec;
impl crate::RegisterSpec for GpioclkgdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioclkgds::R`](R) reader structure"]
impl crate::Readable for GpioclkgdsSpec {}
#[doc = "`write(|w| ..)` method takes [`gpioclkgds::W`](W) writer structure"]
impl crate::Writable for GpioclkgdsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOCLKGDS to value 0"]
impl crate::Resettable for GpioclkgdsSpec {
    const RESET_VALUE: u32 = 0;
}
