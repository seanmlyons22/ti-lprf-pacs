#[doc = "Register `FSM_STAT` reader"]
pub type R = crate::R<FsmStatSpec>;
#[doc = "Register `FSM_STAT` writer"]
pub type W = crate::W<FsmStatSpec>;
#[doc = "Field `INV_DAT` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type InvDatR = crate::BitReader;
#[doc = "Field `INV_DAT` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type InvDatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_PUL_CNT` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type OvrPulCntR = crate::BitReader;
#[doc = "Field `OVR_PUL_CNT` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type OvrPulCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NON_OP` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type NonOpR = crate::BitReader;
#[doc = "Field `NON_OP` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type NonOpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn inv_dat(&self) -> InvDatR {
        InvDatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr_pul_cnt(&self) -> OvrPulCntR {
        OvrPulCntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn non_op(&self) -> NonOpR {
        NonOpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn inv_dat(&mut self) -> InvDatW<FsmStatSpec> {
        InvDatW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ovr_pul_cnt(&mut self) -> OvrPulCntW<FsmStatSpec> {
        OvrPulCntW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn non_op(&mut self) -> NonOpW<FsmStatSpec> {
        NonOpW::new(self, 2)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<FsmStatSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmStatSpec;
impl crate::RegisterSpec for FsmStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_stat::R`](R) reader structure"]
impl crate::Readable for FsmStatSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_stat::W`](W) writer structure"]
impl crate::Writable for FsmStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_STAT to value 0x04"]
impl crate::Resettable for FsmStatSpec {
    const RESET_VALUE: u32 = 0x04;
}
