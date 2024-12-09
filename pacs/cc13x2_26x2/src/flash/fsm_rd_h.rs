#[doc = "Register `FSM_RD_H` reader"]
pub type R = crate::R<FsmRdHSpec>;
#[doc = "Register `FSM_RD_H` writer"]
pub type W = crate::W<FsmRdHSpec>;
#[doc = "Field `RD_H` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RdHR = crate::FieldReader;
#[doc = "Field `RD_H` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RdHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rd_h(&self) -> RdHR {
        RdHR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rd_h(&mut self) -> RdHW<FsmRdHSpec> {
        RdHW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_rd_h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_rd_h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmRdHSpec;
impl crate::RegisterSpec for FsmRdHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_rd_h::R`](R) reader structure"]
impl crate::Readable for FsmRdHSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_rd_h::W`](W) writer structure"]
impl crate::Writable for FsmRdHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_RD_H to value 0x5a"]
impl crate::Resettable for FsmRdHSpec {
    const RESET_VALUE: u32 = 0x5a;
}
