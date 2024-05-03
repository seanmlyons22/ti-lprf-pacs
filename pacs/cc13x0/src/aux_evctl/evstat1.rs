#[doc = "Register `EVSTAT1` reader"]
pub type R = crate::R<Evstat1Spec>;
#[doc = "Register `EVSTAT1` writer"]
pub type W = crate::W<Evstat1Spec>;
#[doc = "Field `AUXIO3` reader - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type Auxio3R = crate::BitReader;
#[doc = "Field `AUXIO3` writer - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type Auxio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO4` reader - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type Auxio4R = crate::BitReader;
#[doc = "Field `AUXIO4` writer - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type Auxio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO5` reader - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type Auxio5R = crate::BitReader;
#[doc = "Field `AUXIO5` writer - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type Auxio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO6` reader - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type Auxio6R = crate::BitReader;
#[doc = "Field `AUXIO6` writer - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type Auxio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO7` reader - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type Auxio7R = crate::BitReader;
#[doc = "Field `AUXIO7` writer - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type Auxio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO8` reader - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type Auxio8R = crate::BitReader;
#[doc = "Field `AUXIO8` writer - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type Auxio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO9` reader - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type Auxio9R = crate::BitReader;
#[doc = "Field `AUXIO9` writer - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type Auxio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO10` reader - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type Auxio10R = crate::BitReader;
#[doc = "Field `AUXIO10` writer - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type Auxio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO11` reader - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type Auxio11R = crate::BitReader;
#[doc = "Field `AUXIO11` writer - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type Auxio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO12` reader - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type Auxio12R = crate::BitReader;
#[doc = "Field `AUXIO12` writer - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type Auxio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO13` reader - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type Auxio13R = crate::BitReader;
#[doc = "Field `AUXIO13` writer - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type Auxio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO14` reader - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type Auxio14R = crate::BitReader;
#[doc = "Field `AUXIO14` writer - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type Auxio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXIO15` reader - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type Auxio15R = crate::BitReader;
#[doc = "Field `AUXIO15` writer - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type Auxio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_REF` reader - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub type AclkRefR = crate::BitReader;
#[doc = "Field `ACLK_REF` writer - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
pub type AclkRefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_EV` reader - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type McuEvR = crate::BitReader;
#[doc = "Field `MCU_EV` writer - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
pub type McuEvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_IRQ` reader - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AdcIrqR = crate::BitReader;
#[doc = "Field `ADC_IRQ` writer - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
pub type AdcIrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> Auxio3R {
        Auxio3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> Auxio4R {
        Auxio4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> Auxio5R {
        Auxio5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> Auxio6R {
        Auxio6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> Auxio7R {
        Auxio7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> Auxio8R {
        Auxio8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> Auxio9R {
        Auxio9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> Auxio10R {
        Auxio10R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> Auxio11R {
        Auxio11R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> Auxio12R {
        Auxio12R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> Auxio13R {
        Auxio13R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> Auxio14R {
        Auxio14R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> Auxio15R {
        Auxio15R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    pub fn aclk_ref(&self) -> AclkRefR {
        AclkRefR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    pub fn mcu_ev(&self) -> McuEvR {
        McuEvR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    pub fn adc_irq(&self) -> AdcIrqR {
        AdcIrqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio3(&mut self) -> Auxio3W<Evstat1Spec> {
        Auxio3W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio4(&mut self) -> Auxio4W<Evstat1Spec> {
        Auxio4W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio5(&mut self) -> Auxio5W<Evstat1Spec> {
        Auxio5W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio6(&mut self) -> Auxio6W<Evstat1Spec> {
        Auxio6W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio7(&mut self) -> Auxio7W<Evstat1Spec> {
        Auxio7W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio8(&mut self) -> Auxio8W<Evstat1Spec> {
        Auxio8W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio9(&mut self) -> Auxio9W<Evstat1Spec> {
        Auxio9W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio10(&mut self) -> Auxio10W<Evstat1Spec> {
        Auxio10W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio11(&mut self) -> Auxio11W<Evstat1Spec> {
        Auxio11W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio12(&mut self) -> Auxio12W<Evstat1Spec> {
        Auxio12W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio13(&mut self) -> Auxio13W<Evstat1Spec> {
        Auxio13W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio14(&mut self) -> Auxio14W<Evstat1Spec> {
        Auxio14W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio15(&mut self) -> Auxio15W<Evstat1Spec> {
        Auxio15W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
TDC reference clock. It is configured by DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL and enabled by AUX_WUC:REFCLKCTL.REQ."]
    #[inline(always)]
    #[must_use]
    pub fn aclk_ref(&mut self) -> AclkRefW<Evstat1Spec> {
        AclkRefW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Event from EVENT configured by EVENT:AUXSEL0."]
    #[inline(always)]
    #[must_use]
    pub fn mcu_ev(&mut self) -> McuEvW<Evstat1Spec> {
        McuEvW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
The logical function for this event is configurable. When DMACTL.EN = 1 : Event = UDMA0 Channel 7 done event OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW When DMACTL.EN = 0 : Event = (NOT AUX_ANAIF:ADCFIFOSTAT.EMPTY) OR AUX_ANAIF:ADCFIFOSTAT.OVERFLOW OR AUX_ANAIF:ADCFIFOSTAT.UNDERFLOW Bit 7 in UDMA0:DONEMASK must be 0."]
    #[inline(always)]
    #[must_use]
    pub fn adc_irq(&mut self) -> AdcIrqW<Evstat1Spec> {
        AdcIrqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<Evstat1Spec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Event Status 1 Current event source levels, 31:16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Evstat1Spec;
impl crate::RegisterSpec for Evstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evstat1::R`](R) reader structure"]
impl crate::Readable for Evstat1Spec {}
#[doc = "`write(|w| ..)` method takes [`evstat1::W`](W) writer structure"]
impl crate::Writable for Evstat1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSTAT1 to value 0"]
impl crate::Resettable for Evstat1Spec {
    const RESET_VALUE: u32 = 0;
}
