#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - 1:0\\]
TDC commands."]
pub type CMD_R = crate::FieldReader<u8, CMD_A>;
#[doc = "1:0\\]
TDC commands.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "3: Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    ABORT = 3,
    #[doc = "2: Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    RUN = 2,
    #[doc = "1: Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    RUN_SYNC_START = 1,
    #[doc = "0: Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    CLR_RESULT = 0,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
impl CMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_A {
        match self.bits {
            3 => CMD_A::ABORT,
            2 => CMD_A::RUN,
            1 => CMD_A::RUN_SYNC_START,
            0 => CMD_A::CLR_RESULT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == CMD_A::ABORT
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == CMD_A::RUN
    }
    #[doc = "Checks if the value of the field is `RUN_SYNC_START`"]
    #[inline(always)]
    pub fn is_run_sync_start(&self) -> bool {
        *self == CMD_A::RUN_SYNC_START
    }
    #[doc = "Checks if the value of the field is `CLR_RESULT`"]
    #[inline(always)]
    pub fn is_clr_result(&self) -> bool {
        *self == CMD_A::CLR_RESULT
    }
}
#[doc = "Field `CMD` writer - 1:0\\]
TDC commands."]
pub type CMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTL_SPEC, u8, CMD_A, 2, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Force TDC state machine back to IDLE state. Never write this command while AUX_TDC:STAT.STATE equals CLR_CNT or WAIT_CLR_CNT_DONE."]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMD_A::ABORT)
    }
    #[doc = "Asynchronous counter start. The counter starts to count when the start event is high. To achieve precise edge-to-edge measurements you must ensure that the start event is low for at least 420 ns after you write this command."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CMD_A::RUN)
    }
    #[doc = "Synchronous counter start. The counter looks for the opposite edge of the selected start event before it starts to count when the selected edge occurs. This guarantees an edge-triggered start and is recommended for frequency measurements."]
    #[inline(always)]
    pub fn run_sync_start(self) -> &'a mut W {
        self.variant(CMD_A::RUN_SYNC_START)
    }
    #[doc = "Clear STAT.SAT, STAT.DONE, and RESULT.VALUE. This is not needed as prerequisite for a measurement. Reliable clear is only guaranteed from IDLE state."]
    #[inline(always)]
    pub fn clr_result(self) -> &'a mut W {
        self.variant(CMD_A::CLR_RESULT)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
TDC commands."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
TDC commands."]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
