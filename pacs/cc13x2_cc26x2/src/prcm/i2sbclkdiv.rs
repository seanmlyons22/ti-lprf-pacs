#[doc = "Register `I2SBCLKDIV` reader"]
pub struct R(crate::R<I2SBCLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SBCLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SBCLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SBCLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SBCLKDIV` writer"]
pub struct W(crate::W<I2SBCLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SBCLKDIV_SPEC>;
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
impl From<crate::W<I2SBCLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SBCLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDIV` reader - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type BDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BDIV` writer - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
pub type BDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, I2SBCLKDIV_SPEC, u16, u16, 10, O>;
#[doc = "Field `RESERVED10` reader - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED10` writer - 31:10\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED10_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2SBCLKDIV_SPEC, u32, u32, 22, O>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    pub fn bdiv(&self) -> BDIV_R {
        BDIV_R::new((self.bits & 0x03ff) as u16)
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
An unsigned factor of the division ratio used to generate I2S BCLK \\[2-1024\\]: BCLK = MCUCLK/BDIV\\[Hz\\]
MCUCLK is 48MHz. A value of 0 is interpreted as 1024. A value of 1 is invalid. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 0, the low phase of the clock is one MCUCLK period longer than the high phase. If BDIV is odd and I2SCLKCTL.SMPL_ON_POSEDGE = 1 , the high phase of the clock is one MCUCLK period longer than the low phase. For changes to take effect, CLKLOADCTL.LOAD needs to be written"]
    #[inline(always)]
    #[must_use]
    pub fn bdiv(&mut self) -> BDIV_W<0> {
        BDIV_W::new(self)
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
#[doc = "BCLK Division Ratio\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sbclkdiv](index.html) module"]
pub struct I2SBCLKDIV_SPEC;
impl crate::RegisterSpec for I2SBCLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sbclkdiv::R](R) reader structure"]
impl crate::Readable for I2SBCLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sbclkdiv::W](W) writer structure"]
impl crate::Writable for I2SBCLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SBCLKDIV to value 0"]
impl crate::Resettable for I2SBCLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
