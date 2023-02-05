#[doc = "Register `SPPR` reader"]
pub struct R(crate::R<SPPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPPR` writer"]
pub struct W(crate::W<SPPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPPR_SPEC>;
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
impl From<crate::W<SPPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROTOCOL` reader - 1:0\\]
Trace output protocol"]
pub type PROTOCOL_R = crate::FieldReader<u8, PROTOCOL_A>;
#[doc = "1:0\\]
Trace output protocol\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROTOCOL_A {
    #[doc = "2: SerialWire Output (NRZ)"]
    SWO_NRZ = 2,
    #[doc = "1: SerialWire Output (Manchester). This is the reset value."]
    SWO_MANCHESTER = 1,
    #[doc = "0: TracePort mode"]
    TRACEPORT = 0,
}
impl From<PROTOCOL_A> for u8 {
    #[inline(always)]
    fn from(variant: PROTOCOL_A) -> Self {
        variant as _
    }
}
impl PROTOCOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROTOCOL_A> {
        match self.bits {
            2 => Some(PROTOCOL_A::SWO_NRZ),
            1 => Some(PROTOCOL_A::SWO_MANCHESTER),
            0 => Some(PROTOCOL_A::TRACEPORT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SWO_NRZ`"]
    #[inline(always)]
    pub fn is_swo_nrz(&self) -> bool {
        *self == PROTOCOL_A::SWO_NRZ
    }
    #[doc = "Checks if the value of the field is `SWO_MANCHESTER`"]
    #[inline(always)]
    pub fn is_swo_manchester(&self) -> bool {
        *self == PROTOCOL_A::SWO_MANCHESTER
    }
    #[doc = "Checks if the value of the field is `TRACEPORT`"]
    #[inline(always)]
    pub fn is_traceport(&self) -> bool {
        *self == PROTOCOL_A::TRACEPORT
    }
}
#[doc = "Field `PROTOCOL` writer - 1:0\\]
Trace output protocol"]
pub type PROTOCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPPR_SPEC, u8, PROTOCOL_A, 2, O>;
impl<'a, const O: u8> PROTOCOL_W<'a, O> {
    #[doc = "SerialWire Output (NRZ)"]
    #[inline(always)]
    pub fn swo_nrz(self) -> &'a mut W {
        self.variant(PROTOCOL_A::SWO_NRZ)
    }
    #[doc = "SerialWire Output (Manchester). This is the reset value."]
    #[inline(always)]
    pub fn swo_manchester(self) -> &'a mut W {
        self.variant(PROTOCOL_A::SWO_MANCHESTER)
    }
    #[doc = "TracePort mode"]
    #[inline(always)]
    pub fn traceport(self) -> &'a mut W {
        self.variant(PROTOCOL_A::TRACEPORT)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPPR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Trace output protocol"]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new((self.bits & 3) as u8)
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
Trace output protocol"]
    #[inline(always)]
    #[must_use]
    pub fn protocol(&mut self) -> PROTOCOL_W<0> {
        PROTOCOL_W::new(self)
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
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sppr](index.html) module"]
pub struct SPPR_SPEC;
impl crate::RegisterSpec for SPPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sppr::R](R) reader structure"]
impl crate::Readable for SPPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sppr::W](W) writer structure"]
impl crate::Writable for SPPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPPR to value 0x01"]
impl crate::Resettable for SPPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
