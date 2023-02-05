#[doc = "Register `COMBEVTOMCUMASK` reader"]
pub struct R(crate::R<COMBEVTOMCUMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMBEVTOMCUMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMBEVTOMCUMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMBEVTOMCUMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMBEVTOMCUMASK` writer"]
pub struct W(crate::W<COMBEVTOMCUMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBEVTOMCUMASK_SPEC>;
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
impl From<crate::W<COMBEVTOMCUMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBEVTOMCUMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AON_WU_EV` reader - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AON_WU_EV_R = crate::BitReader<bool>;
#[doc = "Field `AON_WU_EV` writer - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AON_WU_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `AUX_COMPA` reader - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AUX_COMPA_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPA` writer - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type AUX_COMPA_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `AUX_COMPB` reader - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
pub type AUX_COMPB_R = crate::BitReader<bool>;
#[doc = "Field `AUX_COMPB` writer - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
pub type AUX_COMPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `TDC_DONE` reader - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TDC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `TDC_DONE` writer - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TDC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `TIMER0_EV` reader - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TIMER0_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER0_EV` writer - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TIMER0_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `TIMER1_EV` reader - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TIMER1_EV_R = crate::BitReader<bool>;
#[doc = "Field `TIMER1_EV` writer - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type TIMER1_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` reader - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type SMPH_AUTOTAKE_DONE_R = crate::BitReader<bool>;
#[doc = "Field `SMPH_AUTOTAKE_DONE` writer - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type SMPH_AUTOTAKE_DONE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `ADC_DONE` reader - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_DONE_R = crate::BitReader<bool>;
#[doc = "Field `ADC_DONE` writer - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` reader - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_FIFO_ALMOST_FULL_R = crate::BitReader<bool>;
#[doc = "Field `ADC_FIFO_ALMOST_FULL` writer - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_FIFO_ALMOST_FULL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `OBSMUX0` reader - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type OBSMUX0_R = crate::BitReader<bool>;
#[doc = "Field `OBSMUX0` writer - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type OBSMUX0_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `ADC_IRQ` reader - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IRQ` writer - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
pub type ADC_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMBEVTOMCUMASK_SPEC, bool, O>;
#[doc = "Field `RESERVED11` reader - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED11` writer - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED11_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMBEVTOMCUMASK_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aon_wu_ev(&self) -> AON_WU_EV_R {
        AON_WU_EV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn aux_compa(&self) -> AUX_COMPA_R {
        AUX_COMPA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline(always)]
    pub fn aux_compb(&self) -> AUX_COMPB_R {
        AUX_COMPB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn tdc_done(&self) -> TDC_DONE_R {
        TDC_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn timer0_ev(&self) -> TIMER0_EV_R {
        TIMER0_EV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn timer1_ev(&self) -> TIMER1_EV_R {
        TIMER1_EV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn smph_autotake_done(&self) -> SMPH_AUTOTAKE_DONE_R {
        SMPH_AUTOTAKE_DONE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn adc_done(&self) -> ADC_DONE_R {
        ADC_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn adc_fifo_almost_full(&self) -> ADC_FIFO_ALMOST_FULL_R {
        ADC_FIFO_ALMOST_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn obsmux0(&self) -> OBSMUX0_R {
        OBSMUX0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved11(&self) -> RESERVED11_R {
        RESERVED11_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EVTOMCUFLAGS.AON_WU_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aon_wu_ev(&mut self) -> AON_WU_EV_W<0> {
        AON_WU_EV_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
EVTOMCUFLAGS.AUX_COMPA contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa(&mut self) -> AUX_COMPA_W<1> {
        AUX_COMPA_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
EVTOMCUFLAGS.AUX_COMPB contribution to the AUX_COMB event. 0: Exclude 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb(&mut self) -> AUX_COMPB_W<2> {
        AUX_COMPB_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
EVTOMCUFLAGS.TDC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn tdc_done(&mut self) -> TDC_DONE_W<3> {
        TDC_DONE_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
EVTOMCUFLAGS.TIMER0_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn timer0_ev(&mut self) -> TIMER0_EV_W<4> {
        TIMER0_EV_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
EVTOMCUFLAGS.TIMER1_EV contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn timer1_ev(&mut self) -> TIMER1_EV_W<5> {
        TIMER1_EV_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
EVTOMCUFLAGS.SMPH_AUTOTAKE_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn smph_autotake_done(&mut self) -> SMPH_AUTOTAKE_DONE_W<6> {
        SMPH_AUTOTAKE_DONE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
EVTOMCUFLAGS.ADC_DONE contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_done(&mut self) -> ADC_DONE_W<7> {
        ADC_DONE_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
EVTOMCUFLAGS.ADC_FIFO_ALMOST_FULL contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_fifo_almost_full(&mut self) -> ADC_FIFO_ALMOST_FULL_W<8> {
        ADC_FIFO_ALMOST_FULL_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
EVTOMCUFLAGS.MCU_OBSMUX0 contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn obsmux0(&mut self) -> OBSMUX0_W<9> {
        OBSMUX0_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
EVTOMCUFLAGS.ADC_IRQ contribution to the AUX_COMB event. 0: Exclude. 1: Include."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W<10> {
        ADC_IRQ_W::new(self)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved11(&mut self) -> RESERVED11_W<11> {
        RESERVED11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combevtomcumask](index.html) module"]
pub struct COMBEVTOMCUMASK_SPEC;
impl crate::RegisterSpec for COMBEVTOMCUMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [combevtomcumask::R](R) reader structure"]
impl crate::Readable for COMBEVTOMCUMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [combevtomcumask::W](W) writer structure"]
impl crate::Writable for COMBEVTOMCUMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMBEVTOMCUMASK to value 0"]
impl crate::Resettable for COMBEVTOMCUMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
