#[doc = "Register `EXT_LF_CLK` reader"]
pub struct R(crate::R<EXT_LF_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXT_LF_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXT_LF_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXT_LF_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXT_LF_CLK` writer"]
pub struct W(crate::W<EXT_LF_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXT_LF_CLK_SPEC>;
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
impl From<crate::W<EXT_LF_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXT_LF_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_INCREMENT` reader - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
pub type RTC_INCREMENT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RTC_INCREMENT` writer - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
pub type RTC_INCREMENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EXT_LF_CLK_SPEC, u32, u32, 24, O>;
#[doc = "Field `DIO` reader - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
pub type DIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIO` writer - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
pub type DIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXT_LF_CLK_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    pub fn rtc_increment(&self) -> RTC_INCREMENT_R {
        RTC_INCREMENT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline(always)]
    pub fn dio(&self) -> DIO_R {
        DIO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_increment(&mut self) -> RTC_INCREMENT_W<0> {
        RTC_INCREMENT_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL. The selected DIO will be marked as reserved by the pin driver (TI-RTOS environment) and hence not selectable for other usage."]
    #[inline(always)]
    #[must_use]
    pub fn dio(&mut self) -> DIO_W<24> {
        DIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extern LF clock configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ext_lf_clk](index.html) module"]
pub struct EXT_LF_CLK_SPEC;
impl crate::RegisterSpec for EXT_LF_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ext_lf_clk::R](R) reader structure"]
impl crate::Readable for EXT_LF_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ext_lf_clk::W](W) writer structure"]
impl crate::Writable for EXT_LF_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXT_LF_CLK to value 0xffff_ffff"]
impl crate::Resettable for EXT_LF_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
