#[doc = "Register `LGPT1TENSEL` reader"]
pub type R = crate::R<Lgpt1tenselSpec>;
#[doc = "Register `LGPT1TENSEL` writer"]
pub type W = crate::W<Lgpt1tenselSpec>;
#[doc = "5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "57: LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    Lgpt3Adc = 57,
    #[doc = "56: LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    Lgpt3Dma = 56,
    #[doc = "54: LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    Lgpt3c2 = 54,
    #[doc = "53: LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    Lgpt3c1 = 53,
    #[doc = "52: LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    Lgpt3c0 = 52,
    #[doc = "51: LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    Lgpt2Adc = 51,
    #[doc = "50: LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    Lgpt2Dma = 50,
    #[doc = "48: LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    Lgpt2c2 = 48,
    #[doc = "47: LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    Lgpt2c1 = 47,
    #[doc = "46: LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    Lgpt2c0 = 46,
    #[doc = "44: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    LrfdEvt2 = 44,
    #[doc = "43: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    LrfdEvt1 = 43,
    #[doc = "42: LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    LrfdEvt0 = 42,
    #[doc = "41: LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    Lgpt1Adc = 41,
    #[doc = "40: LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    Lgpt1Dma = 40,
    #[doc = "39: LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    Lgpt1c2 = 39,
    #[doc = "38: LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    Lgpt1c1 = 38,
    #[doc = "37: LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    Lgpt1c0 = 37,
    #[doc = "36: LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    Lgpt0Adc = 36,
    #[doc = "35: LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    Lgpt0Dma = 35,
    #[doc = "34: LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    Lgpt0c2 = 34,
    #[doc = "33: LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    Lgpt0c1 = 33,
    #[doc = "32: LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    Lgpt0c0 = 32,
    #[doc = "31: SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    Systim4 = 31,
    #[doc = "30: SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    Systim3 = 30,
    #[doc = "29: SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    Systim2 = 29,
    #[doc = "28: SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    Systim1 = 28,
    #[doc = "27: SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    Systim0 = 27,
    #[doc = "26: SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    SystimLt = 26,
    #[doc = "25: SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    SystimHb = 25,
    #[doc = "17: ADC general published event, interrupt flags can be found here ADC:MIS1"]
    AdcEvt = 17,
    #[doc = "10: GPIO generic published event, controlled by GPIO:EVTCFG"]
    GpioEvt = 10,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Pubid> for u8 {
    #[inline(always)]
    fn from(variant: Pubid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pubid {
    type Ux = u8;
}
impl crate::IsEnum for Pubid {}
#[doc = "Field `PUBID` reader - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidR = crate::FieldReader<Pubid>;
impl PubidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pubid> {
        match self.bits {
            57 => Some(Pubid::Lgpt3Adc),
            56 => Some(Pubid::Lgpt3Dma),
            54 => Some(Pubid::Lgpt3c2),
            53 => Some(Pubid::Lgpt3c1),
            52 => Some(Pubid::Lgpt3c0),
            51 => Some(Pubid::Lgpt2Adc),
            50 => Some(Pubid::Lgpt2Dma),
            48 => Some(Pubid::Lgpt2c2),
            47 => Some(Pubid::Lgpt2c1),
            46 => Some(Pubid::Lgpt2c0),
            44 => Some(Pubid::LrfdEvt2),
            43 => Some(Pubid::LrfdEvt1),
            42 => Some(Pubid::LrfdEvt0),
            41 => Some(Pubid::Lgpt1Adc),
            40 => Some(Pubid::Lgpt1Dma),
            39 => Some(Pubid::Lgpt1c2),
            38 => Some(Pubid::Lgpt1c1),
            37 => Some(Pubid::Lgpt1c0),
            36 => Some(Pubid::Lgpt0Adc),
            35 => Some(Pubid::Lgpt0Dma),
            34 => Some(Pubid::Lgpt0c2),
            33 => Some(Pubid::Lgpt0c1),
            32 => Some(Pubid::Lgpt0c0),
            31 => Some(Pubid::Systim4),
            30 => Some(Pubid::Systim3),
            29 => Some(Pubid::Systim2),
            28 => Some(Pubid::Systim1),
            27 => Some(Pubid::Systim0),
            26 => Some(Pubid::SystimLt),
            25 => Some(Pubid::SystimHb),
            17 => Some(Pubid::AdcEvt),
            10 => Some(Pubid::GpioEvt),
            0 => Some(Pubid::None),
            _ => None,
        }
    }
    #[doc = "LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt3_adc(&self) -> bool {
        *self == Pubid::Lgpt3Adc
    }
    #[doc = "LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt3_dma(&self) -> bool {
        *self == Pubid::Lgpt3Dma
    }
    #[doc = "LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c2(&self) -> bool {
        *self == Pubid::Lgpt3c2
    }
    #[doc = "LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c1(&self) -> bool {
        *self == Pubid::Lgpt3c1
    }
    #[doc = "LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt3c0(&self) -> bool {
        *self == Pubid::Lgpt3c0
    }
    #[doc = "LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt2_adc(&self) -> bool {
        *self == Pubid::Lgpt2Adc
    }
    #[doc = "LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt2_dma(&self) -> bool {
        *self == Pubid::Lgpt2Dma
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c2(&self) -> bool {
        *self == Pubid::Lgpt2c2
    }
    #[doc = "LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c1(&self) -> bool {
        *self == Pubid::Lgpt2c1
    }
    #[doc = "LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt2c0(&self) -> bool {
        *self == Pubid::Lgpt2c0
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    #[inline(always)]
    pub fn is_lrfd_evt2(&self) -> bool {
        *self == Pubid::LrfdEvt2
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    #[inline(always)]
    pub fn is_lrfd_evt1(&self) -> bool {
        *self == Pubid::LrfdEvt1
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    #[inline(always)]
    pub fn is_lrfd_evt0(&self) -> bool {
        *self == Pubid::LrfdEvt0
    }
    #[doc = "LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt1_adc(&self) -> bool {
        *self == Pubid::Lgpt1Adc
    }
    #[doc = "LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt1_dma(&self) -> bool {
        *self == Pubid::Lgpt1Dma
    }
    #[doc = "LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c2(&self) -> bool {
        *self == Pubid::Lgpt1c2
    }
    #[doc = "LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c1(&self) -> bool {
        *self == Pubid::Lgpt1c1
    }
    #[doc = "LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt1c0(&self) -> bool {
        *self == Pubid::Lgpt1c0
    }
    #[doc = "LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    #[inline(always)]
    pub fn is_lgpt0_adc(&self) -> bool {
        *self == Pubid::Lgpt0Adc
    }
    #[doc = "LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    #[inline(always)]
    pub fn is_lgpt0_dma(&self) -> bool {
        *self == Pubid::Lgpt0Dma
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c2(&self) -> bool {
        *self == Pubid::Lgpt0c2
    }
    #[doc = "LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c1(&self) -> bool {
        *self == Pubid::Lgpt0c1
    }
    #[doc = "LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    #[inline(always)]
    pub fn is_lgpt0c0(&self) -> bool {
        *self == Pubid::Lgpt0c0
    }
    #[doc = "SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    #[inline(always)]
    pub fn is_systim4(&self) -> bool {
        *self == Pubid::Systim4
    }
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn is_systim3(&self) -> bool {
        *self == Pubid::Systim3
    }
    #[doc = "SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    #[inline(always)]
    pub fn is_systim2(&self) -> bool {
        *self == Pubid::Systim2
    }
    #[doc = "SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    #[inline(always)]
    pub fn is_systim1(&self) -> bool {
        *self == Pubid::Systim1
    }
    #[doc = "SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    #[inline(always)]
    pub fn is_systim0(&self) -> bool {
        *self == Pubid::Systim0
    }
    #[doc = "SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    #[inline(always)]
    pub fn is_systim_lt(&self) -> bool {
        *self == Pubid::SystimLt
    }
    #[doc = "SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    #[inline(always)]
    pub fn is_systim_hb(&self) -> bool {
        *self == Pubid::SystimHb
    }
    #[doc = "ADC general published event, interrupt flags can be found here ADC:MIS1"]
    #[inline(always)]
    pub fn is_adc_evt(&self) -> bool {
        *self == Pubid::AdcEvt
    }
    #[doc = "GPIO generic published event, controlled by GPIO:EVTCFG"]
    #[inline(always)]
    pub fn is_gpio_evt(&self) -> bool {
        *self == Pubid::GpioEvt
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Pubid::None
    }
}
#[doc = "Field `PUBID` writer - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type PubidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pubid>;
impl<'a, REG> PubidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LGPT3 ADC trigger event, controlled by LGPT3:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt3_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Adc)
    }
    #[doc = "LGPT3 DMA request event, controlled by LGPT3:DMA setting"]
    #[inline(always)]
    pub fn lgpt3_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Dma)
    }
    #[doc = "LGPT3 compare/capture output event 2, controlled by LGPT3:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt3c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c2)
    }
    #[doc = "LGPT3 compare/capture output event 1, controlled by LGPT3:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt3c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c1)
    }
    #[doc = "LGPT3 compare/capture output event 0, controlled by LGPT3:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt3c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3c0)
    }
    #[doc = "LGPT2 ADC trigger event, controlled by LGPT2:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt2_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Adc)
    }
    #[doc = "LGPT2 DMA request event, controlled by LGPT2:DMA setting"]
    #[inline(always)]
    pub fn lgpt2_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2Dma)
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT2:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt2c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c2)
    }
    #[doc = "LGPT2 compare/capture output event 1, controlled by LGPT2:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt2c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c1)
    }
    #[doc = "LGPT2 compare/capture output event 0, controlled by LGPT2:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt2c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt2c0)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC2"]
    #[inline(always)]
    pub fn lrfd_evt2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt2)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC1"]
    #[inline(always)]
    pub fn lrfd_evt1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt1)
    }
    #[doc = "LRFD interrupt to SYSTIM, controlled by LRFDDBELL:SYSTIMOEV.SRC0"]
    #[inline(always)]
    pub fn lrfd_evt0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::LrfdEvt0)
    }
    #[doc = "LGPT1 ADC trigger event, controlled by LGPT1:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt1_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1Adc)
    }
    #[doc = "LGPT1 DMA request event, controlled by LGPT1:DMA setting"]
    #[inline(always)]
    pub fn lgpt1_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1Dma)
    }
    #[doc = "LGPT1 compare/capture output event 2, controlled by LGPT1:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt1c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c2)
    }
    #[doc = "LGPT1 compare/capture output event 1, controlled by LGPT1:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt1c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c1)
    }
    #[doc = "LGPT1 compare/capture output event 0, controlled by LGPT1:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt1c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt1c0)
    }
    #[doc = "LGPT0 ADC trigger event, controlled by LGPT0:ADCTRG setting"]
    #[inline(always)]
    pub fn lgpt0_adc(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0Adc)
    }
    #[doc = "LGPT0 DMA request event, controlled by LGPT0:DMA setting"]
    #[inline(always)]
    pub fn lgpt0_dma(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0Dma)
    }
    #[doc = "LGPT0 compare/capture output event 2, controlled by LGPT0:C2CFG setting"]
    #[inline(always)]
    pub fn lgpt0c2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c2)
    }
    #[doc = "LGPT0 compare/capture output event 1, controlled by LGPT0:C1CFG setting"]
    #[inline(always)]
    pub fn lgpt0c1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c1)
    }
    #[doc = "LGPT0 compare/capture output event 0, controlled by LGPT0:C0CFG setting"]
    #[inline(always)]
    pub fn lgpt0c0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt0c0)
    }
    #[doc = "SYSTIM Channel 4 event, event flag is SYSTIM:MIS.EVT4"]
    #[inline(always)]
    pub fn systim4(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim4)
    }
    #[doc = "SYSTIM Channel 3 event, event flag is SYSTIM:MIS.EVT3"]
    #[inline(always)]
    pub fn systim3(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim3)
    }
    #[doc = "SYSTIM Channel 2 event, event flag is SYSTIM:MIS.EVT2"]
    #[inline(always)]
    pub fn systim2(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim2)
    }
    #[doc = "SYSTIM Channel 1 event, event flag is SYSTIM:MIS.EVT1"]
    #[inline(always)]
    pub fn systim1(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim1)
    }
    #[doc = "SYSTIM Channel 0 event, event flag is SYSTIM:MIS.EVT0"]
    #[inline(always)]
    pub fn systim0(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Systim0)
    }
    #[doc = "SYSTIM interrupt driven by synchronizing LFTICK signal to SVT clock"]
    #[inline(always)]
    pub fn systim_lt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::SystimLt)
    }
    #[doc = "SYSTIM heartbeat, can be set by SYSTIM:TIMEBIT"]
    #[inline(always)]
    pub fn systim_hb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::SystimHb)
    }
    #[doc = "ADC general published event, interrupt flags can be found here ADC:MIS1"]
    #[inline(always)]
    pub fn adc_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::AdcEvt)
    }
    #[doc = "GPIO generic published event, controlled by GPIO:EVTCFG"]
    #[inline(always)]
    pub fn gpio_evt(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::GpioEvt)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::None)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn pubid(&self) -> PubidR {
        PubidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Read/write selection value. Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn pubid(&mut self) -> PubidW<Lgpt1tenselSpec> {
        PubidW::new(self, 0)
    }
}
#[doc = "Output Selection for CPU Interrupt LGPT1TEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lgpt1tensel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lgpt1tensel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lgpt1tenselSpec;
impl crate::RegisterSpec for Lgpt1tenselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lgpt1tensel::R`](R) reader structure"]
impl crate::Readable for Lgpt1tenselSpec {}
#[doc = "`write(|w| ..)` method takes [`lgpt1tensel::W`](W) writer structure"]
impl crate::Writable for Lgpt1tenselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LGPT1TENSEL to value 0"]
impl crate::Resettable for Lgpt1tenselSpec {
    const RESET_VALUE: u32 = 0;
}
