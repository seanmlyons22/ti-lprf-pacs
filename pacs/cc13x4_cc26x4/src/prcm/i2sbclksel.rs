#[doc = "Register `I2SBCLKSEL` reader"]
pub type R = crate::R<I2sbclkselSpec>;
#[doc = "Register `I2SBCLKSEL` writer"]
pub type W = crate::W<I2sbclkselSpec>;
#[doc = "Field `SRC` reader - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SrcR = crate::BitReader;
#[doc = "Field `SRC` writer - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1R = crate::FieldReader<u32>;
#[doc = "Field `SPARE1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 1) != 0)
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
BCLK source selector 0: Use external BCLK 1: Use internally generated clock For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<I2sbclkselSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare1(&mut self) -> Spare1W<I2sbclkselSpec> {
        Spare1W::new(self, 1)
    }
}
#[doc = "I2S Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sbclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sbclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sbclkselSpec;
impl crate::RegisterSpec for I2sbclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sbclksel::R`](R) reader structure"]
impl crate::Readable for I2sbclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sbclksel::W`](W) writer structure"]
impl crate::Writable for I2sbclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SBCLKSEL to value 0"]
impl crate::Resettable for I2sbclkselSpec {
    const RESET_VALUE: u32 = 0;
}
