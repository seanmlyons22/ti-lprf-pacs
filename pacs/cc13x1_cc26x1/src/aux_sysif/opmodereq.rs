#[doc = "Register `OPMODEREQ` reader"]
pub struct R(crate::R<OPMODEREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPMODEREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPMODEREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPMODEREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPMODEREQ` writer"]
pub struct W(crate::W<OPMODEREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPMODEREQ_SPEC>;
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
impl From<crate::W<OPMODEREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPMODEREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REQ` reader - 1:0\\]
AUX operational mode request."]
pub type REQ_R = crate::FieldReader<u8, REQ_A>;
#[doc = "1:0\\]
AUX operational mode request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REQ_A {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    PDLP = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    PDA = 2,
    #[doc = "1: Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    LP = 1,
    #[doc = "0: Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    A = 0,
}
impl From<REQ_A> for u8 {
    #[inline(always)]
    fn from(variant: REQ_A) -> Self {
        variant as _
    }
}
impl REQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REQ_A {
        match self.bits {
            3 => REQ_A::PDLP,
            2 => REQ_A::PDA,
            1 => REQ_A::LP,
            0 => REQ_A::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PDLP`"]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == REQ_A::PDLP
    }
    #[doc = "Checks if the value of the field is `PDA`"]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == REQ_A::PDA
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == REQ_A::LP
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == REQ_A::A
    }
}
#[doc = "Field `REQ` writer - 1:0\\]
AUX operational mode request."]
pub type REQ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OPMODEREQ_SPEC, u8, REQ_A, 2, O>;
impl<'a, const O: u8> REQ_W<'a, O> {
    #[doc = "Powerdown operational mode with wakeup to lowpower mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to lowpower (LP) as long as the flag is set."]
    #[inline(always)]
    pub fn pdlp(self) -> &'a mut W {
        self.variant(REQ_A::PDLP)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode, characterized by: - Powerdown system power supply state (uLDO) request. - AON_PMCTL:AUXSCECLK.PD_SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag overrides the operational mode externally to active (A) as long as the flag is set."]
    #[inline(always)]
    pub fn pda(self) -> &'a mut W {
        self.variant(REQ_A::PDA)
    }
    #[doc = "Lowpower operational mode, characterized by: - Powerdown system power supply state (uLDO) request. - SCE clock frequency (SCE_RATE) equals SCLK_MF. - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(REQ_A::LP)
    }
    #[doc = "Active operational mode, characterized by: - Active system power supply state (GLDO or DCDC) request. - AON_PMCTL:AUXSCECLK.SRC sets the SCE clock frequency (SCE_RATE). - An active wakeup flag does not change operational mode."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(REQ_A::A)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPMODEREQ_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AUX operational mode request."]
    #[inline(always)]
    pub fn req(&self) -> REQ_R {
        REQ_R::new((self.bits & 3) as u8)
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
AUX operational mode request."]
    #[inline(always)]
    #[must_use]
    pub fn req(&mut self) -> REQ_W<0> {
        REQ_W::new(self)
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
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opmodereq](index.html) module"]
pub struct OPMODEREQ_SPEC;
impl crate::RegisterSpec for OPMODEREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opmodereq::R](R) reader structure"]
impl crate::Readable for OPMODEREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opmodereq::W](W) writer structure"]
impl crate::Writable for OPMODEREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPMODEREQ to value 0"]
impl crate::Resettable for OPMODEREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
