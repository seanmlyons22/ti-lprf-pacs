#[doc = "Register `PROGDLY` reader"]
pub type R = crate::R<ProgdlySpec>;
#[doc = "Register `PROGDLY` writer"]
pub type W = crate::W<ProgdlySpec>;
#[doc = "Field `VALUE` reader - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> ValueW<ProgdlySpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "Programmable Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progdly::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progdly::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProgdlySpec;
impl crate::RegisterSpec for ProgdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`progdly::R`](R) reader structure"]
impl crate::Readable for ProgdlySpec {}
#[doc = "`write(|w| ..)` method takes [`progdly::W`](W) writer structure"]
impl crate::Writable for ProgdlySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROGDLY to value 0"]
impl crate::Resettable for ProgdlySpec {
    const RESET_VALUE: u32 = 0;
}
