#[doc = "Register `I2SWCLKDIV` reader"]
pub type R = crate::R<I2swclkdivSpec>;
#[doc = "Register `I2SWCLKDIV` writer"]
pub type W = crate::W<I2swclkdivSpec>;
#[doc = "Field `WDIV` reader - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WdivR = crate::FieldReader<u16>;
#[doc = "Field `WDIV` writer - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type WdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn wdiv(&self) -> WdivR {
        WdivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
If I2SCLKCTL.WCLK_PHASE = 0, Single phase. WCLK is high one BCLK period and low WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(WDIV\\[9:0\\]
+ 1) \\[Hz\\]
MCUCLK is 48MHz in normal mode. For powerdown mode the frequency is defined by AON_WUC:MCUCLK.PWR_DWN_SRC If I2SCLKCTL.WCLK_PHASE = 1, Dual phase. Each phase on WCLK (50% duty cycle) is WDIV\\[9:0\\]
(unsigned, \\[1-1023\\]) BCLK periods. WCLK = MCUCLK / BDIV*(2*WDIV\\[9:0\\]) \\[Hz\\]
If I2SCLKCTL.WCLK_PHASE = 2, User defined. WCLK is high WDIV\\[7:0\\]
(unsigned, \\[1-255\\]) BCLK periods and low WDIV\\[15:8\\]
(unsigned, \\[1-255\\]) BCLK periods. WCLK = MCUCLK / (BDIV*(WDIV\\[7:0\\]
+ WDIV\\[15:8\\]) \\[Hz\\]
For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn wdiv(&mut self) -> WdivW<I2swclkdivSpec> {
        WdivW::new(self, 0)
    }
}
#[doc = "WCLK Division Ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2swclkdiv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2swclkdiv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2swclkdivSpec;
impl crate::RegisterSpec for I2swclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2swclkdiv::R`](R) reader structure"]
impl crate::Readable for I2swclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`i2swclkdiv::W`](W) writer structure"]
impl crate::Writable for I2swclkdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2SWCLKDIV to value 0"]
impl crate::Resettable for I2swclkdivSpec {
    const RESET_VALUE: u32 = 0;
}
