#[doc = "Register `EVSTAT3` reader"]
pub struct R(crate::R<EVSTAT3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT3` writer"]
pub struct W(crate::W<EVSTAT3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT3_SPEC>;
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
impl From<crate::W<EVSTAT3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT3_SPEC, u8, u8, 5, O>;
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
pub type AUX_TIMER1_EV_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
pub type AUX_TIMER1_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER0_EV` reader - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
pub type AUX_TIMER0_EV_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER0_EV` writer - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
pub type AUX_TIMER0_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_TDC_DONE` reader - 7:7\\]
AUX_TDC:STAT.DONE"]
pub type AUX_TDC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TDC_DONE` writer - 7:7\\]
AUX_TDC:STAT.DONE"]
pub type AUX_TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_ISRC_RESET_N` reader - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
pub type AUX_ISRC_RESET_N_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ISRC_RESET_N` writer - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
pub type AUX_ISRC_RESET_N_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_DONE` reader - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
pub type AUX_ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_DONE` writer - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
pub type AUX_ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AUX_ADC_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AUX_ADC_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
pub type AUX_ADC_FIFO_ALMOST_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_FIFO_NOT_EMPTY` reader - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
pub type AUX_ADC_FIFO_NOT_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_FIFO_NOT_EMPTY` writer - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
pub type AUX_ADC_FIFO_NOT_EMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
pub type AUX_SMPH_AUTOTAKE_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `AUX_DAC_HOLD_ACTIVE` reader - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
pub type AUX_DAC_HOLD_ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_DAC_HOLD_ACTIVE` writer - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
pub type AUX_DAC_HOLD_ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `RESERVED15` reader - 15:15\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
pub type RESERVED15_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED15` writer - 15:15\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
pub type RESERVED15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT3_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    pub fn aux_isrc_reset_n(&self) -> AUX_ISRC_RESET_N_R {
        AUX_ISRC_RESET_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(&self) -> AUX_ADC_FIFO_NOT_EMPTY_R {
        AUX_ADC_FIFO_NOT_EMPTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    pub fn aux_dac_hold_active(&self) -> AUX_DAC_HOLD_ACTIVE_R {
        AUX_DAC_HOLD_ACTIVE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    pub fn reserved15(&self) -> RESERVED15_R {
        RESERVED15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
AUX_TIMER1_EV event, see AUX_TIMER01:T1TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W<5> {
        AUX_TIMER1_EV_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
AUX_TIMER0_EV event, see AUX_TIMER01:T0TARGET for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W<6> {
        AUX_TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
AUX_TDC:STAT.DONE"]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W<7> {
        AUX_TDC_DONE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
AUX_ANAIF:ISRCCTL.RESET_N"]
    #[inline(always)]
    #[must_use]
    pub fn aux_isrc_reset_n(&mut self) -> AUX_ISRC_RESET_N_W<8> {
        AUX_ISRC_RESET_N_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
AUX_ANAIF ADC conversion done event. Event is synchronized at AUX bus rate."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W<9> {
        AUX_ADC_DONE_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AUX_ADC_IRQ_W<10> {
        AUX_ADC_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
AUX_ANAIF:ADCFIFOSTAT.ALMOST_FULL"]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AUX_ADC_FIFO_ALMOST_FULL_W<11> {
        AUX_ADC_FIFO_ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
AUX_ANAIF:ADCFIFOSTAT.EMPTY negated"]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_not_empty(&mut self) -> AUX_ADC_FIFO_NOT_EMPTY_W<12> {
        AUX_ADC_FIFO_NOT_EMPTY_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
See AUX_SMPH:AUTOTAKE.SMPH_ID for description."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AUX_SMPH_AUTOTAKE_DONE_W<13> {
        AUX_SMPH_AUTOTAKE_DONE_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
AUX_ANAIF:DACSTAT.HOLD_ACTIVE"]
    #[inline(always)]
    #[must_use]
    pub fn aux_dac_hold_active(&mut self) -> AUX_DAC_HOLD_ACTIVE_W<14> {
        AUX_DAC_HOLD_ACTIVE_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Software should not rely on the value of a reserved bit. To provide compatibility with future products, the value of a reserved bit should be preserved across a read-modify-write operation."]
    #[inline(always)]
    #[must_use]
    pub fn reserved15(&mut self) -> RESERVED15_W<15> {
        RESERVED15_W::new(self)
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
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat3](index.html) module"]
pub struct EVSTAT3_SPEC;
impl crate::RegisterSpec for EVSTAT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat3::R](R) reader structure"]
impl crate::Readable for EVSTAT3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat3::W](W) writer structure"]
impl crate::Writable for EVSTAT3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT3 to value 0"]
impl crate::Resettable for EVSTAT3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
