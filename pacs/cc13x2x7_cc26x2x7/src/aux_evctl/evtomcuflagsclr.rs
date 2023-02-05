#[doc = "Register `EVTOMCUFLAGSCLR` reader"]
pub struct R(crate::R<EVTOMCUFLAGSCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTOMCUFLAGSCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTOMCUFLAGSCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTOMCUFLAGSCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTOMCUFLAGSCLR` writer"]
pub struct W(crate::W<EVTOMCUFLAGSCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTOMCUFLAGSCLR_SPEC>;
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
impl From<crate::W<EVTOMCUFLAGSCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTOMCUFLAGSCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUX_WU_EV` reader - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
pub type AUX_WU_EV_R = crate::BitReader<bool>;
#[doc = "Field `AUX_WU_EV` writer - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
pub type AUX_WU_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
pub type AUX_COMPA_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
pub type AUX_COMPB_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TDC_DONE` reader - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
pub type AUX_TDC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TDC_DONE` writer - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
pub type AUX_TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER0_EV` reader - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
pub type AUX_TIMER0_EV_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER0_EV` writer - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
pub type AUX_TIMER0_EV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER1_EV` reader - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
pub type AUX_TIMER1_EV_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER1_EV` writer - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
pub type AUX_TIMER1_EV_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` reader - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
pub type AUX_SMPH_AUTOTAKE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_SMPH_AUTOTAKE_DONE` writer - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
pub type AUX_SMPH_AUTOTAKE_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_DONE` reader - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
pub type AUX_ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_DONE` writer - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
pub type AUX_ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub type AUX_ADC_FIFO_ALMOST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
pub type AUX_ADC_FIFO_ALMOST_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `MCU_OBSMUX0` reader - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub type MCU_OBSMUX0_R = crate::BitReader<bool>;
#[doc = "Field `MCU_OBSMUX0` writer - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
pub type MCU_OBSMUX0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_ADC_IRQ` reader - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
pub type AUX_ADC_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `AUX_ADC_IRQ` writer - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
pub type AUX_ADC_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2_EV0` reader - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
pub type AUX_TIMER2_EV0_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2_EV0` writer - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
pub type AUX_TIMER2_EV0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2_EV1` reader - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
pub type AUX_TIMER2_EV1_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2_EV1` writer - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
pub type AUX_TIMER2_EV1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2_EV2` reader - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
pub type AUX_TIMER2_EV2_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2_EV2` writer - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
pub type AUX_TIMER2_EV2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2_EV3` reader - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
pub type AUX_TIMER2_EV3_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2_EV3` writer - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
pub type AUX_TIMER2_EV3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `AUX_TIMER2_PULSE` reader - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
pub type AUX_TIMER2_PULSE_R = crate::BitReader<bool>;
#[doc = "Field `AUX_TIMER2_PULSE` writer - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
pub type AUX_TIMER2_PULSE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EVTOMCUFLAGSCLR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
    #[inline(always)]
    pub fn aux_wu_ev(&self) -> AUX_WU_EV_R {
        AUX_WU_EV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn aux_tdc_done(&self) -> AUX_TDC_DONE_R {
        AUX_TDC_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer0_ev(&self) -> AUX_TIMER0_EV_R {
        AUX_TIMER0_EV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer1_ev(&self) -> AUX_TIMER1_EV_R {
        AUX_TIMER1_EV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    pub fn aux_smph_autotake_done(&self) -> AUX_SMPH_AUTOTAKE_DONE_R {
        AUX_SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline(always)]
    pub fn aux_adc_done(&self) -> AUX_ADC_DONE_R {
        AUX_ADC_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(&self) -> AUX_ADC_FIFO_ALMOST_FULL_R {
        AUX_ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    pub fn mcu_obsmux0(&self) -> MCU_OBSMUX0_R {
        MCU_OBSMUX0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
    #[inline(always)]
    pub fn aux_adc_irq(&self) -> AUX_ADC_IRQ_R {
        AUX_ADC_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer2_ev0(&self) -> AUX_TIMER2_EV0_R {
        AUX_TIMER2_EV0_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer2_ev1(&self) -> AUX_TIMER2_EV1_R {
        AUX_TIMER2_EV1_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer2_ev2(&self) -> AUX_TIMER2_EV2_R {
        AUX_TIMER2_EV2_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer2_ev3(&self) -> AUX_TIMER2_EV3_R {
        AUX_TIMER2_EV3_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
    #[inline(always)]
    pub fn aux_timer2_pulse(&self) -> AUX_TIMER2_PULSE_R {
        AUX_TIMER2_PULSE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write 1 to clear EVTOMCUFLAGS.AUX_WU_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_wu_ev(&mut self) -> AUX_WU_EV_W<0> {
        AUX_WU_EV_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPA. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<1> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Write 1 to clear EVTOMCUFLAGS.AUX_COMPB. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<2> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TDC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_tdc_done(&mut self) -> AUX_TDC_DONE_W<3> {
        AUX_TDC_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER0_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer0_ev(&mut self) -> AUX_TIMER0_EV_W<4> {
        AUX_TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER1_EV. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer1_ev(&mut self) -> AUX_TIMER1_EV_W<5> {
        AUX_TIMER1_EV_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Write 1 to clear EVTOMCUFLAGS.AUX_SMPH_AUTOTAKE_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_smph_autotake_done(&mut self) -> AUX_SMPH_AUTOTAKE_DONE_W<6> {
        AUX_SMPH_AUTOTAKE_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_DONE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_done(&mut self) -> AUX_ADC_DONE_W<7> {
        AUX_ADC_DONE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_FIFO_ALMOST_FULL. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_fifo_almost_full(&mut self) -> AUX_ADC_FIFO_ALMOST_FULL_W<8> {
        AUX_ADC_FIFO_ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Write 1 to clear EVTOMCUFLAGS.MCU_OBSMUX0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_obsmux0(&mut self) -> MCU_OBSMUX0_W<9> {
        MCU_OBSMUX0_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Write 1 to clear EVTOMCUFLAGS.AUX_ADC_IRQ. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_adc_irq(&mut self) -> AUX_ADC_IRQ_W<10> {
        AUX_ADC_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV0. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev0(&mut self) -> AUX_TIMER2_EV0_W<11> {
        AUX_TIMER2_EV0_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV1. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev1(&mut self) -> AUX_TIMER2_EV1_W<12> {
        AUX_TIMER2_EV1_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV2. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev2(&mut self) -> AUX_TIMER2_EV2_W<13> {
        AUX_TIMER2_EV2_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_EV3. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_ev3(&mut self) -> AUX_TIMER2_EV3_W<14> {
        AUX_TIMER2_EV3_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Write 1 to clear EVTOMCUFLAGS.AUX_TIMER2_PULSE. Read value is 0."]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_pulse(&mut self) -> AUX_TIMER2_PULSE_W<15> {
        AUX_TIMER2_PULSE_W::new(self)
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
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtomcuflagsclr](index.html) module"]
pub struct EVTOMCUFLAGSCLR_SPEC;
impl crate::RegisterSpec for EVTOMCUFLAGSCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evtomcuflagsclr::R](R) reader structure"]
impl crate::Readable for EVTOMCUFLAGSCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evtomcuflagsclr::W](W) writer structure"]
impl crate::Writable for EVTOMCUFLAGSCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVTOMCUFLAGSCLR to value 0"]
impl crate::Resettable for EVTOMCUFLAGSCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
