#[doc = "Register `FSM_CMP_VSU` reader"]
pub type R = crate::R<FsmCmpVsuSpec>;
#[doc = "Register `FSM_CMP_VSU` writer"]
pub type W = crate::W<FsmCmpVsuSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `ADD_EXZ` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type AddExzR = crate::FieldReader;
#[doc = "Field `ADD_EXZ` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type AddExzW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
    pub fn add_exz(&self) -> AddExzR {
        AddExzR::new(((self.bits >> 12) & 0x0f) as u8)
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
    pub fn add_exz(&mut self) -> AddExzW<FsmCmpVsuSpec> {
        AddExzW::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_cmp_vsu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_cmp_vsu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmCmpVsuSpec;
impl crate::RegisterSpec for FsmCmpVsuSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_cmp_vsu::R`](R) reader structure"]
impl crate::Readable for FsmCmpVsuSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_cmp_vsu::W`](W) writer structure"]
impl crate::Writable for FsmCmpVsuSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_CMP_VSU to value 0"]
impl crate::Resettable for FsmCmpVsuSpec {
    const RESET_VALUE: u32 = 0;
}
