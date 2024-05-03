#[doc = "Register `I2SCLKCTL` reader"]
pub type R = crate::R<I2sclkctlSpec>;
#[doc = "Register `I2SCLKCTL` writer"]
pub type W = crate::W<I2sclkctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCLK_PHASE` reader - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WclkPhaseR = crate::FieldReader;
#[doc = "Field `WCLK_PHASE` writer - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WclkPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SMPL_ON_POSEDGE` reader - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SmplOnPosedgeR = crate::BitReader;
#[doc = "Field `SMPL_ON_POSEDGE` writer - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type SmplOnPosedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED4` writer - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn wclk_phase(&self) -> WclkPhaseR {
        WclkPhaseR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn smpl_on_posedge(&self) -> SmplOnPosedgeR {
        SmplOnPosedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: MCLK, BCLK and WCLK will be static low 1: Enables the generation of MCLK, BCLK and WCLK For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<I2sclkctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Decides how the WCLK division ratio is calculated and used to generate different duty cycles (See I2SWCLKDIV.WDIV). 0: Single phase 1: Dual phase 2: User Defined 3: Reserved/Undefined For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn wclk_phase(&mut self) -> WclkPhaseW<I2sclkctlSpec> {
        WclkPhaseW::new(self, 1)
    }
    #[doc = "Bit 3 - 3:3\\]
On the I2S serial interface, data and WCLK is sampled and clocked out on opposite edges of BCLK. 0 - data and WCLK are sampled on the negative edge and clocked out on the positive edge. 1 - data and WCLK are sampled on the positive edge and clocked out on the negative edge. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn smpl_on_posedge(&mut self) -> SmplOnPosedgeW<I2sclkctlSpec> {
        SmplOnPosedgeW::new(self, 3)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<I2sclkctlSpec> {
        Reserved4W::new(self, 4)
    }
}
#[doc = "I2S Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sclkctlSpec;
impl crate::RegisterSpec for I2sclkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclkctl::R`](R) reader structure"]
impl crate::Readable for I2sclkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sclkctl::W`](W) writer structure"]
impl crate::Writable for I2sclkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SCLKCTL to value 0"]
impl crate::Resettable for I2sclkctlSpec {
    const RESET_VALUE: u32 = 0;
}
