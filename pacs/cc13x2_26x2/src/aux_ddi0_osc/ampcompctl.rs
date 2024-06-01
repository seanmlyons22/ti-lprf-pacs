#[doc = "Register `AMPCOMPCTL` reader"]
pub type R = crate::R<AmpcompctlSpec>;
#[doc = "Register `AMPCOMPCTL` writer"]
pub type W = crate::W<AmpcompctlSpec>;
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IbiascapHptolpOlCntR = crate::FieldReader;
#[doc = "Field `IBIASCAP_HPTOLP_OL_CNT` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type IbiascapHptolpOlCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CAP_STEP` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CapStepR = crate::FieldReader;
#[doc = "Field `CAP_STEP` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type CapStepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntFinalR = crate::FieldReader;
#[doc = "Field `LPM_IBIAS_WAIT_CNT_FINAL` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type LpmIbiasWaitCntFinalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IBIAS_INIT` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IbiasInitR = crate::FieldReader;
#[doc = "Field `IBIAS_INIT` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type IbiasInitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IBIAS_OFFSET` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IbiasOffsetR = crate::FieldReader;
#[doc = "Field `IBIAS_OFFSET` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type IbiasOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED24` reader - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AMPCOMP_SW_EN` reader - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompSwEnR = crate::BitReader;
#[doc = "Field `AMPCOMP_SW_EN` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompSwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMPCOMP_SW_CTRL` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompSwCtrlR = crate::BitReader;
#[doc = "Field `AMPCOMP_SW_CTRL` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompSwCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "29:28\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AmpcompFsmUpdateRate {
    #[doc = "3: Internal. Only to be used through TI provided API."]
    _250khz = 3,
    #[doc = "2: Internal. Only to be used through TI provided API."]
    _500khz = 2,
    #[doc = "1: Internal. Only to be used through TI provided API."]
    _1mhz = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    _2mhz = 0,
}
impl From<AmpcompFsmUpdateRate> for u8 {
    #[inline(always)]
    fn from(variant: AmpcompFsmUpdateRate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AmpcompFsmUpdateRate {
    type Ux = u8;
}
impl crate::IsEnum for AmpcompFsmUpdateRate {}
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` reader - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompFsmUpdateRateR = crate::FieldReader<AmpcompFsmUpdateRate>;
impl AmpcompFsmUpdateRateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AmpcompFsmUpdateRate {
        match self.bits {
            3 => AmpcompFsmUpdateRate::_250khz,
            2 => AmpcompFsmUpdateRate::_500khz,
            1 => AmpcompFsmUpdateRate::_1mhz,
            0 => AmpcompFsmUpdateRate::_2mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_250khz(&self) -> bool {
        *self == AmpcompFsmUpdateRate::_250khz
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_500khz(&self) -> bool {
        *self == AmpcompFsmUpdateRate::_500khz
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        *self == AmpcompFsmUpdateRate::_1mhz
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_2mhz(&self) -> bool {
        *self == AmpcompFsmUpdateRate::_2mhz
    }
}
#[doc = "Field `AMPCOMP_FSM_UPDATE_RATE` writer - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompFsmUpdateRateW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, AmpcompFsmUpdateRate, crate::Safe>;
impl<'a, REG> AmpcompFsmUpdateRateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _250khz(self) -> &'a mut crate::W<REG> {
        self.variant(AmpcompFsmUpdateRate::_250khz)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _500khz(self) -> &'a mut crate::W<REG> {
        self.variant(AmpcompFsmUpdateRate::_500khz)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut crate::W<REG> {
        self.variant(AmpcompFsmUpdateRate::_1mhz)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn _2mhz(self) -> &'a mut crate::W<REG> {
        self.variant(AmpcompFsmUpdateRate::_2mhz)
    }
}
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompReqModeR = crate::BitReader;
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompReqModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPARE31` reader - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare31R = crate::BitReader;
#[doc = "Field `SPARE31` writer - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Spare31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_hptolp_ol_cnt(&self) -> IbiascapHptolpOlCntR {
        IbiascapHptolpOlCntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cap_step(&self) -> CapStepR {
        CapStepR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt_final(&self) -> LpmIbiasWaitCntFinalR {
        LpmIbiasWaitCntFinalR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_init(&self) -> IbiasInitR {
        IbiasInitR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibias_offset(&self) -> IbiasOffsetR {
        IbiasOffsetR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_en(&self) -> AmpcompSwEnR {
        AmpcompSwEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_sw_ctrl(&self) -> AmpcompSwCtrlR {
        AmpcompSwCtrlR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_fsm_update_rate(&self) -> AmpcompFsmUpdateRateR {
        AmpcompFsmUpdateRateR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AmpcompReqModeR {
        AmpcompReqModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare31(&self) -> Spare31R {
        Spare31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IbiascapHptolpOlCntW<AmpcompctlSpec> {
        IbiascapHptolpOlCntW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cap_step(&mut self) -> CapStepW<AmpcompctlSpec> {
        CapStepW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LpmIbiasWaitCntFinalW<AmpcompctlSpec> {
        LpmIbiasWaitCntFinalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_init(&mut self) -> IbiasInitW<AmpcompctlSpec> {
        IbiasInitW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_offset(&mut self) -> IbiasOffsetW<AmpcompctlSpec> {
        IbiasOffsetW::new(self, 20)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<AmpcompctlSpec> {
        Reserved24W::new(self, 24)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_sw_en(&mut self) -> AmpcompSwEnW<AmpcompctlSpec> {
        AmpcompSwEnW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_sw_ctrl(&mut self) -> AmpcompSwCtrlW<AmpcompctlSpec> {
        AmpcompSwCtrlW::new(self, 27)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_fsm_update_rate(&mut self) -> AmpcompFsmUpdateRateW<AmpcompctlSpec> {
        AmpcompFsmUpdateRateW::new(self, 28)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req_mode(&mut self) -> AmpcompReqModeW<AmpcompctlSpec> {
        AmpcompReqModeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare31(&mut self) -> Spare31W<AmpcompctlSpec> {
        Spare31W::new(self, 31)
    }
}
#[doc = "Amplitude Compensation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcompctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcompctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpcompctlSpec;
impl crate::RegisterSpec for AmpcompctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcompctl::R`](R) reader structure"]
impl crate::Readable for AmpcompctlSpec {}
#[doc = "`write(|w| ..)` method takes [`ampcompctl::W`](W) writer structure"]
impl crate::Writable for AmpcompctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCOMPCTL to value 0"]
impl crate::Resettable for AmpcompctlSpec {
    const RESET_VALUE: u32 = 0;
}
