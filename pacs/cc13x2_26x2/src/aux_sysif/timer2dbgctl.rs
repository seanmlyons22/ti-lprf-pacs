#[doc = "Register `TIMER2DBGCTL` reader"]
pub type R = crate::R<Timer2dbgctlSpec>;
#[doc = "Register `TIMER2DBGCTL` writer"]
pub type W = crate::W<Timer2dbgctlSpec>;
#[doc = "Field `DBG_FREEZE_EN` reader - 0:0\\]
Debug freeze enable. 0: AUX_TIMER2 does not halt when the system CPU halts in debug mode. 1: Halt AUX_TIMER2 when the system CPU halts in debug mode."]
pub type DbgFreezeEnR = crate::BitReader;
#[doc = "Field `DBG_FREEZE_EN` writer - 0:0\\]
Debug freeze enable. 0: AUX_TIMER2 does not halt when the system CPU halts in debug mode. 1: Halt AUX_TIMER2 when the system CPU halts in debug mode."]
pub type DbgFreezeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Debug freeze enable. 0: AUX_TIMER2 does not halt when the system CPU halts in debug mode. 1: Halt AUX_TIMER2 when the system CPU halts in debug mode."]
    #[inline(always)]
    pub fn dbg_freeze_en(&self) -> DbgFreezeEnR {
        DbgFreezeEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Debug freeze enable. 0: AUX_TIMER2 does not halt when the system CPU halts in debug mode. 1: Halt AUX_TIMER2 when the system CPU halts in debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn dbg_freeze_en(&mut self) -> DbgFreezeEnW<Timer2dbgctlSpec> {
        DbgFreezeEnW::new(self, 0)
    }
}
#[doc = "AUX_TIMER2 Debug Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2dbgctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2dbgctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer2dbgctlSpec;
impl crate::RegisterSpec for Timer2dbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2dbgctl::R`](R) reader structure"]
impl crate::Readable for Timer2dbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`timer2dbgctl::W`](W) writer structure"]
impl crate::Writable for Timer2dbgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER2DBGCTL to value 0"]
impl crate::Resettable for Timer2dbgctlSpec {
    const RESET_VALUE: u32 = 0;
}
