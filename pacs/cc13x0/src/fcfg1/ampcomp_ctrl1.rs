#[doc = "Register `AMPCOMP_CTRL1` reader"]
pub type R = crate::R<AmpcompCtrl1Spec>;
#[doc = "Register `AMPCOMP_CTRL1` writer"]
pub type W = crate::W<AmpcompCtrl1Spec>;
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
#[doc = "Field `RESERVED0` reader - 29:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 29:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AMPCOMP_REQ_MODE` reader - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompReqModeR = crate::BitReader;
#[doc = "Field `AMPCOMP_REQ_MODE` writer - 30:30\\]
Internal. Only to be used through TI provided API."]
pub type AmpcompReqModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ampcomp_req_mode(&self) -> AmpcompReqModeR {
        AmpcompReqModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibiascap_hptolp_ol_cnt(&mut self) -> IbiascapHptolpOlCntW<AmpcompCtrl1Spec> {
        IbiascapHptolpOlCntW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cap_step(&mut self) -> CapStepW<AmpcompCtrl1Spec> {
        CapStepW::new(self, 4)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt_final(&mut self) -> LpmIbiasWaitCntFinalW<AmpcompCtrl1Spec> {
        LpmIbiasWaitCntFinalW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_init(&mut self) -> IbiasInitW<AmpcompCtrl1Spec> {
        IbiasInitW::new(self, 16)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibias_offset(&mut self) -> IbiasOffsetW<AmpcompCtrl1Spec> {
        IbiasOffsetW::new(self, 20)
    }
    #[doc = "Bits 24:29 - 29:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<AmpcompCtrl1Spec> {
        Reserved0W::new(self, 24)
    }
    #[doc = "Bit 30 - 30:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ampcomp_req_mode(&mut self) -> AmpcompReqModeW<AmpcompCtrl1Spec> {
        AmpcompReqModeW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<AmpcompCtrl1Spec> {
        Reserved1W::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpcompCtrl1Spec;
impl crate::RegisterSpec for AmpcompCtrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcomp_ctrl1::R`](R) reader structure"]
impl crate::Readable for AmpcompCtrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcomp_ctrl1::W`](W) writer structure"]
impl crate::Writable for AmpcompCtrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCOMP_CTRL1 to value 0xff18_3f47"]
impl crate::Resettable for AmpcompCtrl1Spec {
    const RESET_VALUE: u32 = 0xff18_3f47;
}
