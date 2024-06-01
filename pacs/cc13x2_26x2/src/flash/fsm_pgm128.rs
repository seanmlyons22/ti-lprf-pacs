#[doc = "Register `FSM_PGM128` reader"]
pub type R = crate::R<FsmPgm128Spec>;
#[doc = "Register `FSM_PGM128` writer"]
pub type W = crate::W<FsmPgm128Spec>;
#[doc = "Field `EN_PGM128` reader - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EnPgm128R = crate::BitReader;
#[doc = "Field `EN_PGM128` writer - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
pub type EnPgm128W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    pub fn en_pgm128(&self) -> EnPgm128R {
        EnPgm128R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Enables 128-bit wide programming. This mode requires programming supply voltage to be greater than 2.5v at the Flash Pump. The primary use case for this mode is manufacturing test for test time reduction. 0: 64-bit wide programming. Valid at any programming voltage. A 128-bit word is divided into two 64-bit words for programming. \\[default\\]
This register is write protected with the FSM_WR_ENA register."]
    #[inline(always)]
    #[must_use]
    pub fn en_pgm128(&mut self) -> EnPgm128W<FsmPgm128Spec> {
        EnPgm128W::new(self, 0)
    }
}
#[doc = "FMC FSM Enable 128-bit Wide Programming\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm_pgm128::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_pgm128::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsmPgm128Spec;
impl crate::RegisterSpec for FsmPgm128Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm_pgm128::R`](R) reader structure"]
impl crate::Readable for FsmPgm128Spec {}
#[doc = "`write(|w| ..)` method takes [`fsm_pgm128::W`](W) writer structure"]
impl crate::Writable for FsmPgm128Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM_PGM128 to value 0"]
impl crate::Resettable for FsmPgm128Spec {
    const RESET_VALUE: u32 = 0;
}
