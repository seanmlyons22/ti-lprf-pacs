#[doc = "Register `FSM_PGM128` reader"]
pub struct R(crate::R<FSM_PGM128_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSM_PGM128_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSM_PGM128_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSM_PGM128_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSM_PGM128` writer"]
pub struct W(crate::W<FSM_PGM128_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSM_PGM128_SPEC>;
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
impl From<crate::W<FSM_PGM128_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSM_PGM128_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN_PGM128` reader - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EN_PGM128_R = crate::BitReader<bool>;
#[doc = "Field `EN_PGM128` writer - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EN_PGM128_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSM_PGM128_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_pgm128(&self) -> EN_PGM128_R {
        EN_PGM128_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    #[must_use]
    pub fn en_pgm128(&mut self) -> EN_PGM128_W<0> {
        EN_PGM128_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC FSM Enable 128-bit Wide Programming\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsm_pgm128](index.html) module"]
pub struct FSM_PGM128_SPEC;
impl crate::RegisterSpec for FSM_PGM128_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsm_pgm128::R](R) reader structure"]
impl crate::Readable for FSM_PGM128_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsm_pgm128::W](W) writer structure"]
impl crate::Writable for FSM_PGM128_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSM_PGM128 to value 0"]
impl crate::Resettable for FSM_PGM128_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
