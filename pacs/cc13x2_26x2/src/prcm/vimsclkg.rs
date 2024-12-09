#[doc = "Register `VIMSCLKG` reader"]
pub type R = crate::R<VimsclkgSpec>;
#[doc = "Register `VIMSCLKG` writer"]
pub type W = crate::W<VimsclkgSpec>;
#[doc = "Field `CLK_EN` reader - 1:0\\]
00: Disable clock 01: Disable clock when SYSBUS clock is disabled 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnR = crate::FieldReader;
#[doc = "Field `CLK_EN` writer - 1:0\\]
00: Disable clock 01: Disable clock when SYSBUS clock is disabled 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type ClkEnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
00: Disable clock 01: Disable clock when SYSBUS clock is disabled 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn clk_en(&self) -> ClkEnR {
        ClkEnR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
00: Disable clock 01: Disable clock when SYSBUS clock is disabled 11: Enable clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> ClkEnW<VimsclkgSpec> {
        ClkEnW::new(self, 0)
    }
}
#[doc = "VIMS Clock Gate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vimsclkg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vimsclkg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VimsclkgSpec;
impl crate::RegisterSpec for VimsclkgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vimsclkg::R`](R) reader structure"]
impl crate::Readable for VimsclkgSpec {}
#[doc = "`write(|w| ..)` method takes [`vimsclkg::W`](W) writer structure"]
impl crate::Writable for VimsclkgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIMSCLKG to value 0x03"]
impl crate::Resettable for VimsclkgSpec {
    const RESET_VALUE: u32 = 0x03;
}
