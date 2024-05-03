#[doc = "Register `WUSTAT` reader"]
pub type R = crate::R<WustatSpec>;
#[doc = "Register `WUSTAT` writer"]
pub type W = crate::W<WustatSpec>;
#[doc = "7:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvSignals {
    #[doc = "128: Internal. Only to be used through TI provided API."]
    ScewevProg = 128,
    #[doc = "64: Internal. Only to be used through TI provided API."]
    AuxAdcFifoNotEmpty = 64,
    #[doc = "32: Internal. Only to be used through TI provided API."]
    AuxTimer1EvOrIdle = 32,
    #[doc = "16: Internal. Only to be used through TI provided API."]
    AuxTimer0EvOrIdle = 16,
    #[doc = "8: Internal. Only to be used through TI provided API."]
    AuxTdcDone = 8,
    #[doc = "4: Internal. Only to be used through TI provided API."]
    AuxCompb = 4,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    AuxCompa = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    AuxProgDlyIdle = 1,
}
impl From<EvSignals> for u8 {
    #[inline(always)]
    fn from(variant: EvSignals) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvSignals {
    type Ux = u8;
}
impl crate::IsEnum for EvSignals {}
#[doc = "Field `EV_SIGNALS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvSignalsR = crate::FieldReader<EvSignals>;
impl EvSignalsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvSignals> {
        match self.bits {
            128 => Some(EvSignals::ScewevProg),
            64 => Some(EvSignals::AuxAdcFifoNotEmpty),
            32 => Some(EvSignals::AuxTimer1EvOrIdle),
            16 => Some(EvSignals::AuxTimer0EvOrIdle),
            8 => Some(EvSignals::AuxTdcDone),
            4 => Some(EvSignals::AuxCompb),
            2 => Some(EvSignals::AuxCompa),
            1 => Some(EvSignals::AuxProgDlyIdle),
            _ => None,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_scewev_prog(&self) -> bool {
        *self == EvSignals::ScewevProg
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_adc_fifo_not_empty(&self) -> bool {
        *self == EvSignals::AuxAdcFifoNotEmpty
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_timer1_ev_or_idle(&self) -> bool {
        *self == EvSignals::AuxTimer1EvOrIdle
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_timer0_ev_or_idle(&self) -> bool {
        *self == EvSignals::AuxTimer0EvOrIdle
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_tdc_done(&self) -> bool {
        *self == EvSignals::AuxTdcDone
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_compb(&self) -> bool {
        *self == EvSignals::AuxCompb
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_compa(&self) -> bool {
        *self == EvSignals::AuxCompa
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_aux_prog_dly_idle(&self) -> bool {
        *self == EvSignals::AuxProgDlyIdle
    }
}
#[doc = "Field `EV_SIGNALS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvSignalsW<'a, REG> = crate::FieldWriter<'a, REG, 8, EvSignals>;
impl<'a, REG> EvSignalsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn scewev_prog(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::ScewevProg)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_adc_fifo_not_empty(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxAdcFifoNotEmpty)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer1_ev_or_idle(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxTimer1EvOrIdle)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_timer0_ev_or_idle(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxTimer0EvOrIdle)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_tdc_done(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxTdcDone)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compb(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxCompb)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_compa(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxCompa)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn aux_prog_dly_idle(self) -> &'a mut crate::W<REG> {
        self.variant(EvSignals::AuxProgDlyIdle)
    }
}
#[doc = "Field `WU_SIGNAL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WuSignalR = crate::BitReader;
#[doc = "Field `WU_SIGNAL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WuSignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EXC_VECTOR` reader - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type ExcVectorR = crate::FieldReader;
#[doc = "Field `EXC_VECTOR` writer - 18:16\\]
Internal. Only to be used through TI provided API."]
pub type ExcVectorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED20` reader - 31:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EvSignalsR {
        EvSignalsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WuSignalR {
        WuSignalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> ExcVectorR {
        ExcVectorR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ev_signals(&mut self) -> EvSignalsW<WustatSpec> {
        EvSignalsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wu_signal(&mut self) -> WuSignalW<WustatSpec> {
        WuSignalW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<WustatSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exc_vector(&mut self) -> ExcVectorW<WustatSpec> {
        ExcVectorW::new(self, 16)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<WustatSpec> {
        Reserved20W::new(self, 19)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wustat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wustat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WustatSpec;
impl crate::RegisterSpec for WustatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wustat::R`](R) reader structure"]
impl crate::Readable for WustatSpec {}
#[doc = "`write(|w| ..)` method takes [`wustat::W`](W) writer structure"]
impl crate::Writable for WustatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUSTAT to value 0"]
impl crate::Resettable for WustatSpec {
    const RESET_VALUE: u32 = 0;
}
