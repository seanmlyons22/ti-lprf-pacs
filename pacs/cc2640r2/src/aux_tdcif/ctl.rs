#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "1:0\\]
TDC commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    #[doc = "3: Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    Abort = 3,
    #[doc = "2: Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    Run = 2,
    #[doc = "1: Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RunSyncStart = 1,
    #[doc = "0: Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    ClrResult = 0,
}
impl From<Cmd> for u8 {
    #[inline(always)]
    fn from(variant: Cmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd {
    type Ux = u8;
}
impl crate::IsEnum for Cmd {}
#[doc = "Field `CMD` writer - 1:0\\]
TDC commands."]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd, crate::Safe>;
impl<'a, REG> CmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    #[inline(always)]
    pub fn abort(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Abort)
    }
    #[doc = "Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::Run)
    }
    #[doc = "Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    #[inline(always)]
    pub fn run_sync_start(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::RunSyncStart)
    }
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    #[inline(always)]
    pub fn clr_result(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd::ClrResult)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
TDC commands."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CmdW<CtlSpec> {
        CmdW::new(self, 0)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
