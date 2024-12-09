#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_UPD_EN` reader - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
pub type RtcUpdEnR = crate::BitReader;
#[doc = "Field `RTC_UPD_EN` writer - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
pub type RtcUpdEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_4KHZ_EN` reader - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
pub type Rtc4khzEnR = crate::BitReader;
#[doc = "Field `RTC_4KHZ_EN` writer - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
pub type Rtc4khzEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `RESET` writer - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvDelay {
    #[doc = "13: Delay by 144 clock cycles"]
    D144 = 13,
    #[doc = "12: Delay by 128 clock cycles"]
    D128 = 12,
    #[doc = "11: Delay by 112 clock cycles"]
    D112 = 11,
    #[doc = "10: Delay by 96 clock cycles"]
    D96 = 10,
    #[doc = "9: Delay by 80 clock cycles"]
    D80 = 9,
    #[doc = "8: Delay by 64 clock cycles"]
    D64 = 8,
    #[doc = "7: Delay by 48 clock cycles"]
    D48 = 7,
    #[doc = "6: Delay by 32 clock cycles"]
    D32 = 6,
    #[doc = "5: Delay by 16 clock cycles"]
    D16 = 5,
    #[doc = "4: Delay by 8 clock cycles"]
    D8 = 4,
    #[doc = "3: Delay by 4 clock cycles"]
    D4 = 3,
    #[doc = "2: Delay by 2 clock cycles"]
    D2 = 2,
    #[doc = "1: Delay by 1 clock cycles"]
    D1 = 1,
    #[doc = "0: No delay on delayed event"]
    D0 = 0,
}
impl From<EvDelay> for u8 {
    #[inline(always)]
    fn from(variant: EvDelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvDelay {
    type Ux = u8;
}
impl crate::IsEnum for EvDelay {}
#[doc = "Field `EV_DELAY` reader - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
pub type EvDelayR = crate::FieldReader<EvDelay>;
impl EvDelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvDelay> {
        match self.bits {
            13 => Some(EvDelay::D144),
            12 => Some(EvDelay::D128),
            11 => Some(EvDelay::D112),
            10 => Some(EvDelay::D96),
            9 => Some(EvDelay::D80),
            8 => Some(EvDelay::D64),
            7 => Some(EvDelay::D48),
            6 => Some(EvDelay::D32),
            5 => Some(EvDelay::D16),
            4 => Some(EvDelay::D8),
            3 => Some(EvDelay::D4),
            2 => Some(EvDelay::D2),
            1 => Some(EvDelay::D1),
            0 => Some(EvDelay::D0),
            _ => None,
        }
    }
    #[doc = "Delay by 144 clock cycles"]
    #[inline(always)]
    pub fn is_d144(&self) -> bool {
        *self == EvDelay::D144
    }
    #[doc = "Delay by 128 clock cycles"]
    #[inline(always)]
    pub fn is_d128(&self) -> bool {
        *self == EvDelay::D128
    }
    #[doc = "Delay by 112 clock cycles"]
    #[inline(always)]
    pub fn is_d112(&self) -> bool {
        *self == EvDelay::D112
    }
    #[doc = "Delay by 96 clock cycles"]
    #[inline(always)]
    pub fn is_d96(&self) -> bool {
        *self == EvDelay::D96
    }
    #[doc = "Delay by 80 clock cycles"]
    #[inline(always)]
    pub fn is_d80(&self) -> bool {
        *self == EvDelay::D80
    }
    #[doc = "Delay by 64 clock cycles"]
    #[inline(always)]
    pub fn is_d64(&self) -> bool {
        *self == EvDelay::D64
    }
    #[doc = "Delay by 48 clock cycles"]
    #[inline(always)]
    pub fn is_d48(&self) -> bool {
        *self == EvDelay::D48
    }
    #[doc = "Delay by 32 clock cycles"]
    #[inline(always)]
    pub fn is_d32(&self) -> bool {
        *self == EvDelay::D32
    }
    #[doc = "Delay by 16 clock cycles"]
    #[inline(always)]
    pub fn is_d16(&self) -> bool {
        *self == EvDelay::D16
    }
    #[doc = "Delay by 8 clock cycles"]
    #[inline(always)]
    pub fn is_d8(&self) -> bool {
        *self == EvDelay::D8
    }
    #[doc = "Delay by 4 clock cycles"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == EvDelay::D4
    }
    #[doc = "Delay by 2 clock cycles"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == EvDelay::D2
    }
    #[doc = "Delay by 1 clock cycles"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == EvDelay::D1
    }
    #[doc = "No delay on delayed event"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == EvDelay::D0
    }
}
#[doc = "Field `EV_DELAY` writer - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
pub type EvDelayW<'a, REG> = crate::FieldWriter<'a, REG, 4, EvDelay>;
impl<'a, REG> EvDelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay by 144 clock cycles"]
    #[inline(always)]
    pub fn d144(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D144)
    }
    #[doc = "Delay by 128 clock cycles"]
    #[inline(always)]
    pub fn d128(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D128)
    }
    #[doc = "Delay by 112 clock cycles"]
    #[inline(always)]
    pub fn d112(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D112)
    }
    #[doc = "Delay by 96 clock cycles"]
    #[inline(always)]
    pub fn d96(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D96)
    }
    #[doc = "Delay by 80 clock cycles"]
    #[inline(always)]
    pub fn d80(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D80)
    }
    #[doc = "Delay by 64 clock cycles"]
    #[inline(always)]
    pub fn d64(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D64)
    }
    #[doc = "Delay by 48 clock cycles"]
    #[inline(always)]
    pub fn d48(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D48)
    }
    #[doc = "Delay by 32 clock cycles"]
    #[inline(always)]
    pub fn d32(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D32)
    }
    #[doc = "Delay by 16 clock cycles"]
    #[inline(always)]
    pub fn d16(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D16)
    }
    #[doc = "Delay by 8 clock cycles"]
    #[inline(always)]
    pub fn d8(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D8)
    }
    #[doc = "Delay by 4 clock cycles"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D4)
    }
    #[doc = "Delay by 2 clock cycles"]
    #[inline(always)]
    pub fn d2(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D2)
    }
    #[doc = "Delay by 1 clock cycles"]
    #[inline(always)]
    pub fn d1(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D1)
    }
    #[doc = "No delay on delayed event"]
    #[inline(always)]
    pub fn d0(self) -> &'a mut crate::W<REG> {
        self.variant(EvDelay::D0)
    }
}
#[doc = "Field `RESERVED12` reader - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved12R = crate::FieldReader;
#[doc = "18:16\\]
Eventmask selecting which delayed events that form the combined event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CombEvMask {
    #[doc = "4: Use Channel 2 delayed event in combined event"]
    Ch2 = 4,
    #[doc = "2: Use Channel 1 delayed event in combined event"]
    Ch1 = 2,
    #[doc = "1: Use Channel 0 delayed event in combined event"]
    Ch0 = 1,
    #[doc = "0: No event is selected for combined event."]
    None = 0,
}
impl From<CombEvMask> for u8 {
    #[inline(always)]
    fn from(variant: CombEvMask) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CombEvMask {
    type Ux = u8;
}
impl crate::IsEnum for CombEvMask {}
#[doc = "Field `COMB_EV_MASK` reader - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
pub type CombEvMaskR = crate::FieldReader<CombEvMask>;
impl CombEvMaskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CombEvMask> {
        match self.bits {
            4 => Some(CombEvMask::Ch2),
            2 => Some(CombEvMask::Ch1),
            1 => Some(CombEvMask::Ch0),
            0 => Some(CombEvMask::None),
            _ => None,
        }
    }
    #[doc = "Use Channel 2 delayed event in combined event"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == CombEvMask::Ch2
    }
    #[doc = "Use Channel 1 delayed event in combined event"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == CombEvMask::Ch1
    }
    #[doc = "Use Channel 0 delayed event in combined event"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == CombEvMask::Ch0
    }
    #[doc = "No event is selected for combined event."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CombEvMask::None
    }
}
#[doc = "Field `COMB_EV_MASK` writer - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
pub type CombEvMaskW<'a, REG> = crate::FieldWriter<'a, REG, 3, CombEvMask>;
impl<'a, REG> CombEvMaskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use Channel 2 delayed event in combined event"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut crate::W<REG> {
        self.variant(CombEvMask::Ch2)
    }
    #[doc = "Use Channel 1 delayed event in combined event"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut crate::W<REG> {
        self.variant(CombEvMask::Ch1)
    }
    #[doc = "Use Channel 0 delayed event in combined event"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut crate::W<REG> {
        self.variant(CombEvMask::Ch0)
    }
    #[doc = "No event is selected for combined event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CombEvMask::None)
    }
}
#[doc = "Field `RESERVED19` reader - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    pub fn rtc_upd_en(&self) -> RtcUpdEnR {
        RtcUpdEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    pub fn rtc_4khz_en(&self) -> Rtc4khzEnR {
        Rtc4khzEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    pub fn ev_delay(&self) -> EvDelayR {
        EvDelayR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    pub fn comb_ev_mask(&self) -> CombEvMaskR {
        CombEvMaskR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable RTC counter 0: Halted (frozen) 1: Running"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<CtlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
RTC_UPD is a 16 KHz signal used to sync up the radio timer. The 16 Khz is SCLK_LF divided by 2 0: RTC_UPD signal is forced to 0 1: RTC_UPD signal is toggling @16 kHz"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_upd_en(&mut self) -> RtcUpdEnW<CtlSpec> {
        RtcUpdEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
RTC_4KHZ is a 4 KHz reference output, tapped from SUBSEC.VALUE bit 19 which is used by AUX timer. 0: RTC_4KHZ signal is forced to 0 1: RTC_4KHZ is enabled ( provied that RTC is enabled EN)"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_4khz_en(&mut self) -> Rtc4khzEnW<CtlSpec> {
        Rtc4khzEnW::new(self, 2)
    }
    #[doc = "Bit 7 - 7:7\\]
RTC Counter reset. Writing 1 to this bit will reset the RTC counter. This bit is cleared when reset takes effect"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<CtlSpec> {
        ResetW::new(self, 7)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Number of SCLK_LF clock cycles waited before generating delayed events. (Common setting for all RTC cannels) the delayed event is delayed"]
    #[inline(always)]
    #[must_use]
    pub fn ev_delay(&mut self) -> EvDelayW<CtlSpec> {
        EvDelayW::new(self, 8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Eventmask selecting which delayed events that form the combined event."]
    #[inline(always)]
    #[must_use]
    pub fn comb_ev_mask(&mut self) -> CombEvMaskW<CtlSpec> {
        CombEvMaskW::new(self, 16)
    }
}
#[doc = "Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
