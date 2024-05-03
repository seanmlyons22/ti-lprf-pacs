#[doc = "Register `I2SBCLKDIV` reader"]
pub type R = crate::R<I2sbclkdivSpec>;
#[doc = "Register `I2SBCLKDIV` writer"]
pub type W = crate::W<I2sbclkdivSpec>;
#[doc = "Field `BDIV` reader - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type BdivR = crate::FieldReader<u16>;
#[doc = "Field `BDIV` writer - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type BdivW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved10W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn bdiv(&self) -> BdivR {
        BdivR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn bdiv(&mut self) -> BdivW<I2sbclkdivSpec> {
        BdivW::new(self, 0)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> Reserved10W<I2sbclkdivSpec> {
        Reserved10W::new(self, 10)
    }
}
#[doc = "BCLK Division Ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sbclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sbclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sbclkdivSpec;
impl crate::RegisterSpec for I2sbclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sbclkdiv::R`](R) reader structure"]
impl crate::Readable for I2sbclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`i2sbclkdiv::W`](W) writer structure"]
impl crate::Writable for I2sbclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SBCLKDIV to value 0"]
impl crate::Resettable for I2sbclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
