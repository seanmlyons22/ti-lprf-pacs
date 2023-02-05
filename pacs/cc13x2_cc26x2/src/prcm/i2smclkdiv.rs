#[doc = "Register `I2SMCLKDIV` reader"]
pub struct R(crate::R<I2SMCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SMCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SMCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SMCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SMCLKDIV` writer"]
pub struct W(crate::W<I2SMCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SMCLKDIV_SPEC>;
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
impl From<crate::W<I2SMCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SMCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MDIV` reader - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type MDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MDIV` writer - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type MDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SMCLKDIV_SPEC, u16, u16, 10, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SMCLKDIV_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved10(&self) -> RESERVED10_R {
        RESERVED10_R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate MCLK \\[2-1024\\]: MCLK = MCUCLK/MDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If MDIV is odd the low phase of the clock is one MCUCLK period longer than the high phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn mdiv(&mut self) -> MDIV_W<0> {
        MDIV_W::new(self)
    }
    #[doc = "Bits 10:31 - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved10(&mut self) -> RESERVED10_W<10> {
        RESERVED10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2smclkdiv](index.html) module"]
pub struct I2SMCLKDIV_SPEC;
impl crate::RegisterSpec for I2SMCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2smclkdiv::R](R) reader structure"]
impl crate::Readable for I2SMCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2smclkdiv::W](W) writer structure"]
impl crate::Writable for I2SMCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SMCLKDIV to value 0"]
impl crate::Resettable for I2SMCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
