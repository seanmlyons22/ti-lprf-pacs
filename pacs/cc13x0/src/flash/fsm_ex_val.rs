#[doc = "Register `FSM_EX_VAL` reader"]
pub type R = crate::R<FsmExValSpec>;
#[doc = "Register `FSM_EX_VAL` writer"]
pub type W = crate::W<FsmExValSpec>;
#[doc = "Field `EXE_VALD` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ExeValdR = crate::FieldReader;
#[doc = "Field `EXE_VALD` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type ExeValdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REP_VSU` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RepVsuR = crate::FieldReader;
#[doc = "Field `REP_VSU` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type RepVsuW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exe_vald(&self) -> ExeValdR {
        ExeValdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rep_vsu(&self) -> RepVsuR {
        RepVsuR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exe_vald(&mut self) -> ExeValdW<FsmExValSpec> {
        ExeValdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rep_vsu(&mut self) -> RepVsuW<FsmExValSpec> {
        RepVsuW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_ex_val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_ex_val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmExValSpec;
impl crate::RegisterSpec for FsmExValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_ex_val::R`](R) reader structure"]
impl crate::Readable for FsmExValSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_ex_val::W`](W) writer structure"]
impl crate::Writable for FsmExValSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_EX_VAL to value 0x0301"]
impl crate::Resettable for FsmExValSpec {
    const RESET_VALUE: u32 = 0x0301;
}
