#[doc = "Register `EVSYNCRATE` reader"]
pub type R = crate::R<EvsyncrateSpec>;
#[doc = "Register `EVSYNCRATE` writer"]
pub type W = crate::W<EvsyncrateSpec>;
#[doc = "0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxTimer2SyncRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<AuxTimer2SyncRate> for bool {
    #[inline(always)]
    fn from(variant: AuxTimer2SyncRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_TIMER2_SYNC_RATE` reader - 0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
pub type AuxTimer2SyncRateR = crate::BitReader<AuxTimer2SyncRate>;
impl AuxTimer2SyncRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxTimer2SyncRate {
        match self.bits {
            true => AuxTimer2SyncRate::BusRate,
            false => AuxTimer2SyncRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AuxTimer2SyncRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AuxTimer2SyncRate::SceRate
    }
}
#[doc = "Field `AUX_TIMER2_SYNC_RATE` writer - 0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
pub type AuxTimer2SyncRateW<'a, REG> = crate::BitWriter<'a, REG, AuxTimer2SyncRate>;
impl<'a, REG> AuxTimer2SyncRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer2SyncRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxTimer2SyncRate::SceRate)
    }
}
#[doc = "1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompbSyncRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<AuxCompbSyncRate> for bool {
    #[inline(always)]
    fn from(variant: AuxCompbSyncRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPB_SYNC_RATE` reader - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
pub type AuxCompbSyncRateR = crate::BitReader<AuxCompbSyncRate>;
impl AuxCompbSyncRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompbSyncRate {
        match self.bits {
            true => AuxCompbSyncRate::BusRate,
            false => AuxCompbSyncRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AuxCompbSyncRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AuxCompbSyncRate::SceRate
    }
}
#[doc = "Field `AUX_COMPB_SYNC_RATE` writer - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
pub type AuxCompbSyncRateW<'a, REG> = crate::BitWriter<'a, REG, AuxCompbSyncRate>;
impl<'a, REG> AuxCompbSyncRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompbSyncRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompbSyncRate::SceRate)
    }
}
#[doc = "2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxCompaSyncRate {
    #[doc = "1: AUX bus rate"]
    BusRate = 1,
    #[doc = "0: SCE rate"]
    SceRate = 0,
}
impl From<AuxCompaSyncRate> for bool {
    #[inline(always)]
    fn from(variant: AuxCompaSyncRate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUX_COMPA_SYNC_RATE` reader - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
pub type AuxCompaSyncRateR = crate::BitReader<AuxCompaSyncRate>;
impl AuxCompaSyncRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AuxCompaSyncRate {
        match self.bits {
            true => AuxCompaSyncRate::BusRate,
            false => AuxCompaSyncRate::SceRate,
        }
    }
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn is_bus_rate(&self) -> bool {
        *self == AuxCompaSyncRate::BusRate
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn is_sce_rate(&self) -> bool {
        *self == AuxCompaSyncRate::SceRate
    }
}
#[doc = "Field `AUX_COMPA_SYNC_RATE` writer - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
pub type AuxCompaSyncRateW<'a, REG> = crate::BitWriter<'a, REG, AuxCompaSyncRate>;
impl<'a, REG> AuxCompaSyncRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUX bus rate"]
    #[inline(always)]
    pub fn bus_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompaSyncRate::BusRate)
    }
    #[doc = "SCE rate"]
    #[inline(always)]
    pub fn sce_rate(self) -> &'a mut crate::W<REG> {
        self.variant(AuxCompaSyncRate::SceRate)
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    pub fn aux_timer2_sync_rate(&self) -> AuxTimer2SyncRateR {
        AuxTimer2SyncRateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    pub fn aux_compb_sync_rate(&self) -> AuxCompbSyncRateR {
        AuxCompbSyncRateR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    pub fn aux_compa_sync_rate(&self) -> AuxCompaSyncRateR {
        AuxCompaSyncRateR::new(((self.bits >> 2) & 1) != 0)
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
Select synchronization rate for: - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3 - AUX_EVCTL:EVSTAT3.AUX_TIMER2_PULSE"]
    #[inline(always)]
    #[must_use]
    pub fn aux_timer2_sync_rate(&mut self) -> AuxTimer2SyncRateW<EvsyncrateSpec> {
        AuxTimer2SyncRateW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPB event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compb_sync_rate(&mut self) -> AuxCompbSyncRateW<EvsyncrateSpec> {
        AuxCompbSyncRateW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select synchronization rate for AUX_EVCTL:EVSTAT2.AUX_COMPA event."]
    #[inline(always)]
    #[must_use]
    pub fn aux_compa_sync_rate(&mut self) -> AuxCompaSyncRateW<EvsyncrateSpec> {
        AuxCompaSyncRateW::new(self, 2)
    }
}
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. You must select SCE rate when AUX_SCE uses the event. You must select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsyncrate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsyncrate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvsyncrateSpec;
impl crate::RegisterSpec for EvsyncrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evsyncrate::R`](R) reader structure"]
impl crate::Readable for EvsyncrateSpec {}
#[doc = "`write(|w| ..)` method takes [`evsyncrate::W`](W) writer structure"]
impl crate::Writable for EvsyncrateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVSYNCRATE to value 0"]
impl crate::Resettable for EvsyncrateSpec {
    const RESET_VALUE: u32 = 0;
}
