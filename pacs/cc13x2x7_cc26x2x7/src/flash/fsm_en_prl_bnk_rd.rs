#[doc = "Register `FSM_EN_PRL_BNK_RD` reader"]
pub type R = crate::R<FsmEnPrlBnkRdSpec>;
#[doc = "Register `FSM_EN_PRL_BNK_RD` writer"]
pub type W = crate::W<FsmEnPrlBnkRdSpec>;
#[doc = "Field `EN_PRL_BNK_RD` reader - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EnPrlBnkRdR = crate::BitReader;
#[doc = "Field `EN_PRL_BNK_RD` writer - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EnPrlBnkRdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_prl_bnk_rd(&self) -> EnPrlBnkRdR {
        EnPrlBnkRdR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    #[must_use]
    pub fn en_prl_bnk_rd(&mut self) -> EnPrlBnkRdW<FsmEnPrlBnkRdSpec> {
        EnPrlBnkRdW::new(self, 0)
    }
}
#[doc = "FMC FSM Enable Parallell Reads for Multibanks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_en_prl_bnk_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_en_prl_bnk_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmEnPrlBnkRdSpec;
impl crate::RegisterSpec for FsmEnPrlBnkRdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_en_prl_bnk_rd::R`](R) reader structure"]
impl crate::Readable for FsmEnPrlBnkRdSpec {}
#[doc = "`write(|w| ..)` method takes [`fsm_en_prl_bnk_rd::W`](W) writer structure"]
impl crate::Writable for FsmEnPrlBnkRdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_EN_PRL_BNK_RD to value 0"]
impl crate::Resettable for FsmEnPrlBnkRdSpec {
    const RESET_VALUE: u32 = 0;
}
