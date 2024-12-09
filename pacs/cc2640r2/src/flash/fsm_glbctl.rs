#[doc = "Register `FSM_GLBCTL` reader"]
pub type R = crate::R<FsmGlbctlSpec>;
#[doc = "Register `FSM_GLBCTL` writer"]
pub type W = crate::W<FsmGlbctlSpec>;
#[doc = "Field `CLKSEL` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ClkselR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_glbctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_glbctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmGlbctlSpec;
impl crate::RegisterSpec for FsmGlbctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_glbctl::R`](R) reader structure"]
impl crate::Readable for FsmGlbctlSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_glbctl::W`](W) writer structure"]
impl crate::Writable for FsmGlbctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_GLBCTL to value 0x01"]
impl crate::Resettable for FsmGlbctlSpec {
    const RESET_VALUE: u32 = 0x01;
}
