#[doc = "Register `STCVR` reader"]
pub type R = crate::R<StcvrSpec>;
#[doc = "Register `STCVR` writer"]
pub type W = crate::W<StcvrSpec>;
#[doc = "Field `CURRENT` reader - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
pub type CurrentR = crate::FieldReader<u32>;
#[doc = "Field `CURRENT` writer - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
pub type CurrentW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    pub fn current(&self) -> CurrentR {
        CurrentR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Current value at the time the register is accessed. No read-modify-write protection is provided, so change with care. Writing to it with any value clears the register to 0. Clearing this register also clears STCSR.COUNTFLAG."]
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CurrentW<StcvrSpec> {
        CurrentW::new(self, 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<StcvrSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "SysTick Current Value Read from this register returns the current value of SysTick counter. Writing to this register resets the SysTick counter (as well as STCSR.COUNTFLAG).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StcvrSpec;
impl crate::RegisterSpec for StcvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcvr::R`](R) reader structure"]
impl crate::Readable for StcvrSpec {}
#[doc = "`write(|w| ..)` method takes [`stcvr::W`](W) writer structure"]
impl crate::Writable for StcvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STCVR to value 0"]
impl crate::Resettable for StcvrSpec {
    const RESET_VALUE: u32 = 0;
}
