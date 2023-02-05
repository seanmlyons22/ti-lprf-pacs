#[doc = "Register `FSM_EN_PRL_BNK_RD` reader"]
pub struct R(crate::R<FSM_EN_PRL_BNK_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_EN_PRL_BNK_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_EN_PRL_BNK_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_EN_PRL_BNK_RD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_EN_PRL_BNK_RD` writer"]
pub struct W(crate::W<FSM_EN_PRL_BNK_RD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_EN_PRL_BNK_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSM_EN_PRL_BNK_RD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_EN_PRL_BNK_RD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_PRL_BNK_RD` reader - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EN_PRL_BNK_RD_R = crate::BitReader<bool>;
#[doc = "Field `EN_PRL_BNK_RD` writer - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EN_PRL_BNK_RD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, FSM_EN_PRL_BNK_RD_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_prl_bnk_rd(&self) -> EN_PRL_BNK_RD_R {
        EN_PRL_BNK_RD_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enables parallel reads of multiple banks The primary use case for this mode is manufacturing test for test time reduction. 0: Read of one bank only. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    #[must_use]
    pub fn en_prl_bnk_rd(&mut self) -> EN_PRL_BNK_RD_W<0> {
        EN_PRL_BNK_RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC FSM Enable Parallell Reads for Multibanks\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_en_prl_bnk_rd](index.html) module"]
pub struct FSM_EN_PRL_BNK_RD_SPEC;
impl crate::RegisterSpec for FSM_EN_PRL_BNK_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_en_prl_bnk_rd::R](R) reader structure"]
impl crate::Readable for FSM_EN_PRL_BNK_RD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_en_prl_bnk_rd::W](W) writer structure"]
impl crate::Writable for FSM_EN_PRL_BNK_RD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_EN_PRL_BNK_RD to value 0"]
impl crate::Resettable for FSM_EN_PRL_BNK_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
