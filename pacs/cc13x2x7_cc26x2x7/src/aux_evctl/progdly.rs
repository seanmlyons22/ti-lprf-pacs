#[doc = "Register `PROGDLY` reader"]
pub struct R(crate::R<PROGDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROGDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROGDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROGDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROGDLY` writer"]
pub struct W(crate::W<PROGDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROGDLY_SPEC>;
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
impl From<crate::W<PROGDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROGDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALUE` reader - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
pub type VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VALUE` writer - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
pub type VALUE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROGDLY_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PROGDLY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
VALUE decrements to 0 at a rate of 1 MHz. The event AUX_PROG_DLY_IDLE is high when VALUE is 0, otherwise it is low. Only use the programmable delay counter and the AUX_PROG_DLY_IDLE event when AUX_SYSIF:OPMODEACK.ACK equals A or LP. Decrementation of VALUE halts when either is true: - AUX_SCE:CTL.DBG_FREEZE_EN is set and system CPU is halted in debug mode. - AUX_SYSIF:TIMERHALT.PROGDLY is set."]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<0> {
        VALUE_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [progdly](index.html) module"]
pub struct PROGDLY_SPEC;
impl crate::RegisterSpec for PROGDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [progdly::R](R) reader structure"]
impl crate::Readable for PROGDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [progdly::W](W) writer structure"]
impl crate::Writable for PROGDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PROGDLY to value 0"]
impl crate::Resettable for PROGDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
