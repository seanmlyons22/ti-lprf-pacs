#[doc = "Register `I2SWCLKDIV` reader"]
pub struct R(crate::R<I2SWCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SWCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SWCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SWCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SWCLKDIV` writer"]
pub struct W(crate::W<I2SWCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SWCLKDIV_SPEC>;
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
impl From<crate::W<I2SWCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SWCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
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
pub type WDIV_R = crate::FieldReader<u16, u16>;
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
pub type WDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SWCLKDIV_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SWCLKDIV_SPEC, u16, u16, 16, O>;
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
    pub fn wdiv(&self) -> WDIV_R {
        WDIV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
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
    pub fn wdiv(&mut self) -> WDIV_W<0> {
        WDIV_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2swclkdiv](index.html) module"]
pub struct I2SWCLKDIV_SPEC;
impl crate::RegisterSpec for I2SWCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2swclkdiv::R](R) reader structure"]
impl crate::Readable for I2SWCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2swclkdiv::W](W) writer structure"]
impl crate::Writable for I2SWCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SWCLKDIV to value 0"]
impl crate::Resettable for I2SWCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
