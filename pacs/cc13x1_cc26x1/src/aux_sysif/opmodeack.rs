#[doc = "Register `OPMODEACK` reader"]
pub struct R(crate::R<OPMODEACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPMODEACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPMODEACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPMODEACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPMODEACK` writer"]
pub struct W(crate::W<OPMODEACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPMODEACK_SPEC>;
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
impl From<crate::W<OPMODEACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPMODEACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK` reader - 1:0\\]
AUX operational mode acknowledgement."]
pub type ACK_R = crate::FieldReader<u8, ACK_A>;
#[doc = "1:0\\]
AUX operational mode acknowledgement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACK_A {
    #[doc = "3: Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    PDLP = 3,
    #[doc = "2: Powerdown operational mode with wakeup to active mode is acknowledged."]
    PDA = 2,
    #[doc = "1: Lowpower operational mode is acknowledged."]
    LP = 1,
    #[doc = "0: Active operational mode is acknowledged."]
    A = 0,
}
impl From<ACK_A> for u8 {
    #[inline(always)]
    fn from(variant: ACK_A) -> Self {
        variant as _
    }
}
impl ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_A {
        match self.bits {
            3 => ACK_A::PDLP,
            2 => ACK_A::PDA,
            1 => ACK_A::LP,
            0 => ACK_A::A,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PDLP`"]
    #[inline(always)]
    pub fn is_pdlp(&self) -> bool {
        *self == ACK_A::PDLP
    }
    #[doc = "Checks if the value of the field is `PDA`"]
    #[inline(always)]
    pub fn is_pda(&self) -> bool {
        *self == ACK_A::PDA
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == ACK_A::LP
    }
    #[doc = "Checks if the value of the field is `A`"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == ACK_A::A
    }
}
#[doc = "Field `ACK` writer - 1:0\\]
AUX operational mode acknowledgement."]
pub type ACK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, OPMODEACK_SPEC, u8, ACK_A, 2, O>;
impl<'a, const O: u8> ACK_W<'a, O> {
    #[doc = "Powerdown operational mode with wakeup to lowpower mode is acknowledged."]
    #[inline(always)]
    pub fn pdlp(self) -> &'a mut W {
        self.variant(ACK_A::PDLP)
    }
    #[doc = "Powerdown operational mode with wakeup to active mode is acknowledged."]
    #[inline(always)]
    pub fn pda(self) -> &'a mut W {
        self.variant(ACK_A::PDA)
    }
    #[doc = "Lowpower operational mode is acknowledged."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(ACK_A::LP)
    }
    #[doc = "Active operational mode is acknowledged."]
    #[inline(always)]
    pub fn a(self) -> &'a mut W {
        self.variant(ACK_A::A)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPMODEACK_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
AUX operational mode acknowledgement."]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new((self.bits & 3) as u8)
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
AUX operational mode acknowledgement."]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<0> {
        ACK_W::new(self)
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
#[doc = "Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opmodeack](index.html) module"]
pub struct OPMODEACK_SPEC;
impl crate::RegisterSpec for OPMODEACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opmodeack::R](R) reader structure"]
impl crate::Readable for OPMODEACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opmodeack::W](W) writer structure"]
impl crate::Writable for OPMODEACK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPMODEACK to value 0"]
impl crate::Resettable for OPMODEACK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
