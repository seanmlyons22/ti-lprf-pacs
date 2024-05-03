#[doc = "Register `WUEVCLR` reader"]
pub type R = crate::R<WuevclrSpec>;
#[doc = "Register `WUEVCLR` writer"]
pub type W = crate::W<WuevclrSpec>;
#[doc = "Field `AON_PROG_WU` reader - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
pub type AonProgWuR = crate::BitReader;
#[doc = "Field `AON_PROG_WU` writer - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
pub type AonProgWuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_SW` reader - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
pub type AonSwR = crate::BitReader;
#[doc = "Field `AON_SW` writer - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
pub type AonSwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AON_RTC_CH2` reader - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
pub type AonRtcCh2R = crate::BitReader;
#[doc = "Field `AON_RTC_CH2` writer - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
pub type AonRtcCh2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline(always)]
    pub fn aon_prog_wu(&self) -> AonProgWuR {
        AonProgWuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline(always)]
    pub fn aon_sw(&self) -> AonSwR {
        AonSwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline(always)]
    pub fn aon_rtc_ch2(&self) -> AonRtcCh2R {
        AonRtcCh2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set to clear the WUEVFLAGS.AON_PROG_WU wake-up event. Note only if an IO event is selected as wake-up event, is it possible to use this field to clear the source. Other sources cannot be cleared using this field. The IO pin needs to be assigned to AUX in the IOC and the input enable for the pin needs to be set in AIODIO0 or AIODIO1 for this clearing to take effect. This bit must remain set until WUEVFLAGS.AON_PROG_WU returns to 0."]
    #[inline(always)]
    #[must_use]
    pub fn aon_prog_wu(&mut self) -> AonProgWuW<WuevclrSpec> {
        AonProgWuW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set to clear the WUEVFLAGS.AON_SW wake-up event. This bit must remain set until WUEVFLAGS.AON_SW returns to 0."]
    #[inline(always)]
    #[must_use]
    pub fn aon_sw(&mut self) -> AonSwW<WuevclrSpec> {
        AonSwW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Set to clear the WUEVFLAGS.AON_RTC_CH2 wake-up event. Note that if RTC channel 2 is also set as source for AON_PROG_WU this field can also clear WUEVFLAGS.AON_PROG_WU This bit must remain set until WUEVFLAGS.AON_RTC_CH2 returns to 0."]
    #[inline(always)]
    #[must_use]
    pub fn aon_rtc_ch2(&mut self) -> AonRtcCh2W<WuevclrSpec> {
        AonRtcCh2W::new(self, 2)
    }
}
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuevclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuevclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WuevclrSpec;
impl crate::RegisterSpec for WuevclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wuevclr::R`](R) reader structure"]
impl crate::Readable for WuevclrSpec {}
#[doc = "`write(|w| ..)` method takes [`wuevclr::W`](W) writer structure"]
impl crate::Writable for WuevclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUEVCLR to value 0"]
impl crate::Resettable for WuevclrSpec {
    const RESET_VALUE: u32 = 0;
}
