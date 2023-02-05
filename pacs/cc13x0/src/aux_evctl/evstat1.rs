#[doc = "Register `EVSTAT1` reader"]
pub struct R(crate::R<EVSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT1` writer"]
pub struct W(crate::W<EVSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT1_SPEC>;
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
impl From<crate::W<EVSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXIO3` reader - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type AUXIO3_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO3` writer - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type AUXIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO4` reader - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type AUXIO4_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO4` writer - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type AUXIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO5` reader - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type AUXIO5_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO5` writer - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type AUXIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO6` reader - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type AUXIO6_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO6` writer - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type AUXIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO7` reader - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type AUXIO7_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO7` writer - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type AUXIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO8` reader - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type AUXIO8_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO8` writer - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type AUXIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO9` reader - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type AUXIO9_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO9` writer - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type AUXIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO10` reader - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type AUXIO10_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO10` writer - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type AUXIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO11` reader - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type AUXIO11_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO11` writer - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type AUXIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO12` reader - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type AUXIO12_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO12` writer - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type AUXIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO13` reader - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type AUXIO13_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO13` writer - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type AUXIO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO14` reader - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type AUXIO14_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO14` writer - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type AUXIO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO15` reader - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type AUXIO15_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO15` writer - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type AUXIO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `ACLK_REF` reader - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub type ACLK_REF_R = crate::BitReader<bool>;
#[doc = "Field `ACLK_REF` writer - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub type ACLK_REF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `MCU_EV` reader - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type MCU_EV_R = crate::BitReader<bool>;
#[doc = "Field `MCU_EV` writer - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type MCU_EV_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `ADC_IRQ` reader - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type ADC_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ADC_IRQ` writer - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type ADC_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> AUXIO3_R {
        AUXIO3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> AUXIO4_R {
        AUXIO4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> AUXIO5_R {
        AUXIO5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> AUXIO6_R {
        AUXIO6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> AUXIO7_R {
        AUXIO7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> AUXIO8_R {
        AUXIO8_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> AUXIO9_R {
        AUXIO9_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> AUXIO10_R {
        AUXIO10_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> AUXIO11_R {
        AUXIO11_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> AUXIO12_R {
        AUXIO12_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> AUXIO13_R {
        AUXIO13_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> AUXIO14_R {
        AUXIO14_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> AUXIO15_R {
        AUXIO15_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> ACLK_REF_R {
        ACLK_REF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> MCU_EV_R {
        MCU_EV_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn adc_irq(&self) -> ADC_IRQ_R {
        ADC_IRQ_R::new(((self.bits >> 15) & 1) != 0)
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
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio3(&mut self) -> AUXIO3_W<0> {
        AUXIO3_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio4(&mut self) -> AUXIO4_W<1> {
        AUXIO4_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio5(&mut self) -> AUXIO5_W<2> {
        AUXIO5_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio6(&mut self) -> AUXIO6_W<3> {
        AUXIO6_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio7(&mut self) -> AUXIO7_W<4> {
        AUXIO7_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio8(&mut self) -> AUXIO8_W<5> {
        AUXIO8_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio9(&mut self) -> AUXIO9_W<6> {
        AUXIO9_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio10(&mut self) -> AUXIO10_W<7> {
        AUXIO10_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio11(&mut self) -> AUXIO11_W<8> {
        AUXIO11_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio12(&mut self) -> AUXIO12_W<9> {
        AUXIO12_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio13(&mut self) -> AUXIO13_W<10> {
        AUXIO13_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio14(&mut self) -> AUXIO14_W<11> {
        AUXIO14_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio15(&mut self) -> AUXIO15_W<12> {
        AUXIO15_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref(&mut self) -> ACLK_REF_W<13> {
        ACLK_REF_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ev(&mut self) -> MCU_EV_W<14> {
        MCU_EV_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> ADC_IRQ_W<15> {
        ADC_IRQ_W::new(self)
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
#[doc = "Event Status 1 Current event source levels, 31:16\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1](index.html) module"]
pub struct EVSTAT1_SPEC;
impl crate::RegisterSpec for EVSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat1::R](R) reader structure"]
impl crate::Readable for EVSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat1::W](W) writer structure"]
impl crate::Writable for EVSTAT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT1 to value 0"]
impl crate::Resettable for EVSTAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
