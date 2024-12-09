#[doc = "Register `FSM_VSTAT` reader"]
pub type R = crate::R<FsmVstatSpec>;
#[doc = "Register `FSM_VSTAT` writer"]
pub type W = crate::W<FsmVstatSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `VSTAT_CNT` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VstatCntR = crate::FieldReader;
#[doc = "Field `VSTAT_CNT` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VstatCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vstat_cnt(&self) -> VstatCntR {
        VstatCntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vstat_cnt(&mut self) -> VstatCntW<FsmVstatSpec> {
        VstatCntW::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_vstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_vstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmVstatSpec;
impl crate::RegisterSpec for FsmVstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_vstat::R`](R) reader structure"]
impl crate::Readable for FsmVstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_vstat::W`](W) writer structure"]
impl crate::Writable for FsmVstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_VSTAT to value 0x3000"]
impl crate::Resettable for FsmVstatSpec {
    const RESET_VALUE: u32 = 0x3000;
}
