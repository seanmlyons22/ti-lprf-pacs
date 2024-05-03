#[doc = "Register `DACCTL` reader"]
pub type R = crate::R<DacctlSpec>;
#[doc = "Register `DACCTL` writer"]
pub type W = crate::W<DacctlSpec>;
#[doc = "2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DacVoutSel {
    #[doc = "4: Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    CompaIn = 4,
    #[doc = "2: Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    CompaRef = 2,
    #[doc = "1: Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    CompbRef = 1,
    #[doc = "0: Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    Nc = 0,
}
impl From<DacVoutSel> for u8 {
    #[inline(always)]
    fn from(variant: DacVoutSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DacVoutSel {
    type Ux = u8;
}
impl crate::IsEnum for DacVoutSel {}
#[doc = "Field `DAC_VOUT_SEL` reader - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
pub type DacVoutSelR = crate::FieldReader<DacVoutSel>;
impl DacVoutSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DacVoutSel> {
        match self.bits {
            4 => Some(DacVoutSel::CompaIn),
            2 => Some(DacVoutSel::CompaRef),
            1 => Some(DacVoutSel::CompbRef),
            0 => Some(DacVoutSel::Nc),
            _ => None,
        }
    }
    #[doc = "Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    #[inline(always)]
    pub fn is_compa_in(&self) -> bool {
        *self == DacVoutSel::CompaIn
    }
    #[doc = "Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    #[inline(always)]
    pub fn is_compa_ref(&self) -> bool {
        *self == DacVoutSel::CompaRef
    }
    #[doc = "Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    #[inline(always)]
    pub fn is_compb_ref(&self) -> bool {
        *self == DacVoutSel::CompbRef
    }
    #[doc = "Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == DacVoutSel::Nc
    }
}
#[doc = "Field `DAC_VOUT_SEL` writer - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
pub type DacVoutSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, DacVoutSel>;
impl<'a, REG> DacVoutSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Connect to COMPA_IN analog node. Required setting to drive external load selected in ADI_4_AUX:MUX1.COMPA_IN."]
    #[inline(always)]
    pub fn compa_in(self) -> &'a mut crate::W<REG> {
        self.variant(DacVoutSel::CompaIn)
    }
    #[doc = "Connect to COMPA_REF analog node. It is not possible to drive external loads connected to COMPA_REF I/O mux with this setting."]
    #[inline(always)]
    pub fn compa_ref(self) -> &'a mut crate::W<REG> {
        self.variant(DacVoutSel::CompaRef)
    }
    #[doc = "Connect to COMPB_REF analog node. Required setting to use Comparator B."]
    #[inline(always)]
    pub fn compb_ref(self) -> &'a mut crate::W<REG> {
        self.variant(DacVoutSel::CompbRef)
    }
    #[doc = "Connect to nothing It is recommended to use NC as intermediate step when you change DAC_VOUT_SEL."]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(DacVoutSel::Nc)
    }
}
#[doc = "Field `DAC_PRECHARGE_EN` reader - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
pub type DacPrechargeEnR = crate::BitReader;
#[doc = "Field `DAC_PRECHARGE_EN` writer - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
pub type DacPrechargeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_BUFFER_EN` reader - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
pub type DacBufferEnR = crate::BitReader;
#[doc = "Field `DAC_BUFFER_EN` writer - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
pub type DacBufferEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_EN` reader - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
pub type DacEnR = crate::BitReader;
#[doc = "Field `DAC_EN` writer - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
pub type DacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    pub fn dac_vout_sel(&self) -> DacVoutSelR {
        DacVoutSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    pub fn dac_precharge_en(&self) -> DacPrechargeEnR {
        DacPrechargeEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    pub fn dac_buffer_en(&self) -> DacBufferEnR {
        DacBufferEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline(always)]
    pub fn dac_en(&self) -> DacEnR {
        DacEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
DAC output connection. An analog node must only have one driver. Other drivers for the following analog nodes are configured in \\[ANATOP_MMAP::ADI_4_AUX:*\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dac_vout_sel(&mut self) -> DacVoutSelW<DacctlSpec> {
        DacVoutSelW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
DAC precharge enable. Only enable precharge when ADI_4_AUX:MUX2.DAC_VREF_SEL equals DCOUPL and VDDS is higher than 2.65 V. DAC output voltage range: 0: 0 V to 1.28 V. 1: 1.28 V to 2.56 V. Otherwise, see ADI_4_AUX:MUX2.DAC_VREF_SEL for DAC output voltage range. Enable precharge 1 us before you enable the DAC and the buffer."]
    #[inline(always)]
    #[must_use]
    pub fn dac_precharge_en(&mut self) -> DacPrechargeEnW<DacctlSpec> {
        DacPrechargeEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
DAC buffer enable. DAC buffer reduces the time required to produce the programmed voltage at the expense of increased current consumption. 0: Disable DAC buffer. 1: Enable DAC buffer. Enable buffer when DAC_VOUT_SEL equals COMPA_IN. Do not enable the buffer when AUX_SYSIF:OPMODEREQ.REQ equals PDA or PDLP."]
    #[inline(always)]
    #[must_use]
    pub fn dac_buffer_en(&mut self) -> DacBufferEnW<DacctlSpec> {
        DacBufferEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
DAC module enable. 0: Disable DAC. 1: Enable DAC. The System CPU must not use the DAC when AUX_SYSIF:OPMODEREQ.REQ equals PDA in Standby TI-RTOS power mode. The System CPU must set AUX_SYSIF:PEROPRATE.ANAIF_DAC_OP_RATE to BUS_RATE to use the DAC in Active and Idle TI-RTOS power modes."]
    #[inline(always)]
    #[must_use]
    pub fn dac_en(&mut self) -> DacEnW<DacctlSpec> {
        DacEnW::new(self, 5)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<DacctlSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "DAC Control This register controls the analog part of the DAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacctlSpec;
impl crate::RegisterSpec for DacctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dacctl::R`](R) reader structure"]
impl crate::Readable for DacctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dacctl::W`](W) writer structure"]
impl crate::Writable for DacctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DACCTL to value 0"]
impl crate::Resettable for DacctlSpec {
    const RESET_VALUE: u32 = 0;
}
