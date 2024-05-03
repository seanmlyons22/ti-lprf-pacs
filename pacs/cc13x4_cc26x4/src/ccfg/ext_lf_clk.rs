#[doc = "Register `EXT_LF_CLK` reader"]
pub type R = crate::R<ExtLfClkSpec>;
#[doc = "Register `EXT_LF_CLK` writer"]
pub type W = crate::W<ExtLfClkSpec>;
#[doc = "Field `RTC_INCREMENT` reader - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
pub type RtcIncrementR = crate::FieldReader<u32>;
#[doc = "Field `RTC_INCREMENT` writer - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
pub type RtcIncrementW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `DIO` reader - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL."]
pub type DioR = crate::FieldReader;
#[doc = "Field `DIO` writer - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL."]
pub type DioW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    pub fn rtc_increment(&self) -> RtcIncrementR {
        RtcIncrementR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL."]
    #[inline(always)]
    pub fn dio(&self) -> DioR {
        DioR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Unsigned integer, defining the input frequency of the external clock and is written to AON_RTC:SUBSECINC.VALUEINC. Defined as follows: EXT_LF_CLK.RTC_INCREMENT = 2^38/InputClockFrequency in Hertz (e.g.: RTC_INCREMENT=0x800000 for InputClockFrequency=32768 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_increment(&mut self) -> RtcIncrementW<ExtLfClkSpec> {
        RtcIncrementW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Unsigned integer, selecting the DIO to supply external 32 kHz clock as SCLK_LF when MODE_CONF.SCLK_LF_OPTION is set to EXTERNAL."]
    #[inline(always)]
    #[must_use]
    pub fn dio(&mut self) -> DioW<ExtLfClkSpec> {
        DioW::new(self, 24)
    }
}
#[doc = "Extern LF clock configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lf_clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lf_clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtLfClkSpec;
impl crate::RegisterSpec for ExtLfClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_lf_clk::R`](R) reader structure"]
impl crate::Readable for ExtLfClkSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_lf_clk::W`](W) writer structure"]
impl crate::Writable for ExtLfClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_LF_CLK to value 0xffff_ffff"]
impl crate::Resettable for ExtLfClkSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
