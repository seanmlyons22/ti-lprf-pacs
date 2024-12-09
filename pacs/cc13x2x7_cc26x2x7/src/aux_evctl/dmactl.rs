#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DmactlSpec>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DmactlSpec>;
#[doc = "0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "1: UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    AuxAdcFifoAlmostFull = 1,
    #[doc = "0: UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    AuxAdcFifoNotEmpty = 0,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            true => Sel::AuxAdcFifoAlmostFull,
            false => Sel::AuxAdcFifoNotEmpty,
        }
    }
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    #[inline(always)]
    pub fn is_aux_adc_fifo_almost_full(&self) -> bool {
        *self == Sel::AuxAdcFifoAlmostFull
    }
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == Sel::AuxAdcFifoNotEmpty
    }
}
#[doc = "Field `SEL` writer - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UDMA0 trigger event will be generated when the ADC FIFO is almost full (3/4 full)."]
    #[inline(always)]
    pub fn aux_adc_fifo_almost_full(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AuxAdcFifoAlmostFull)
    }
    #[doc = "UDMA0 trigger event will be generated when there are samples in the ADC FIFO."]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::AuxAdcFifoNotEmpty)
    }
}
#[doc = "Field `EN` reader - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "2:2\\]
UDMA0 Request mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReqMode {
    #[doc = "1: Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    Single = 1,
    #[doc = "0: Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    Burst = 0,
}
impl From<ReqMode> for bool {
    #[inline(always)]
    fn from(variant: ReqMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ_MODE` reader - 2:2\\]
UDMA0 Request mode"]
pub type ReqModeR = crate::BitReader<ReqMode>;
impl ReqModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ReqMode {
        match self.bits {
            true => ReqMode::Single,
            false => ReqMode::Burst,
        }
    }
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == ReqMode::Single
    }
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == ReqMode::Burst
    }
}
#[doc = "Field `REQ_MODE` writer - 2:2\\]
UDMA0 Request mode"]
pub type ReqModeW<'a, REG> = crate::BitWriter<'a, REG, ReqMode>;
impl<'a, REG> ReqModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(ReqMode::Single)
    }
    #[doc = "Burst requests are generated on UDMA0 channel 7 when the condition configured in SEL is met."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(ReqMode::Burst)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    pub fn req_mode(&self) -> ReqModeR {
        ReqModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select FIFO watermark level required to trigger a UDMA0 transfer of ADC FIFO data."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<DmactlSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
uDMA ADC interface enable. 0: Disable UDMA0 interface to ADC. 1: Enable UDMA0 interface to ADC."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<DmactlSpec> {
        EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
UDMA0 Request mode"]
    #[inline(always)]
    #[must_use]
    pub fn req_mode(&mut self) -> ReqModeW<DmactlSpec> {
        ReqModeW::new(self, 2)
    }
}
#[doc = "Direct Memory Access Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactlSpec;
impl crate::RegisterSpec for DmactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DmactlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DmactlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DmactlSpec {
    const RESET_VALUE: u32 = 0;
}
