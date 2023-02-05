#[doc = "Register `UDMACH10BSEL` reader"]
pub struct R(crate::R<UDMACH10BSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDMACH10BSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDMACH10BSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDMACH10BSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDMACH10BSEL` writer"]
pub struct W(crate::W<UDMACH10BSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDMACH10BSEL_SPEC>;
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
impl From<crate::W<UDMACH10BSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDMACH10BSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV` reader - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_R = crate::FieldReader<u8, EV_A>;
#[doc = "7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 78"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EV_A {
    #[doc = "121: Always asserted"]
    ALWAYS_ACTIVE = 121,
    #[doc = "84: GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3B_DMABREQ = 84,
    #[doc = "83: GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    GPT3A_DMABREQ = 83,
    #[doc = "82: GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2B_DMABREQ = 82,
    #[doc = "81: GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    GPT2A_DMABREQ = 81,
    #[doc = "80: GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1B_DMABREQ = 80,
    #[doc = "79: GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    GPT1A_DMABREQ = 79,
    #[doc = "78: GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0B_DMABREQ = 78,
    #[doc = "77: GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    GPT0A_DMABREQ = 77,
    #[doc = "0: Always inactive"]
    NONE = 0,
}
impl From<EV_A> for u8 {
    #[inline(always)]
    fn from(variant: EV_A) -> Self {
        variant as _
    }
}
impl EV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EV_A> {
        match self.bits {
            121 => Some(EV_A::ALWAYS_ACTIVE),
            84 => Some(EV_A::GPT3B_DMABREQ),
            83 => Some(EV_A::GPT3A_DMABREQ),
            82 => Some(EV_A::GPT2B_DMABREQ),
            81 => Some(EV_A::GPT2A_DMABREQ),
            80 => Some(EV_A::GPT1B_DMABREQ),
            79 => Some(EV_A::GPT1A_DMABREQ),
            78 => Some(EV_A::GPT0B_DMABREQ),
            77 => Some(EV_A::GPT0A_DMABREQ),
            0 => Some(EV_A::NONE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALWAYS_ACTIVE`"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == EV_A::ALWAYS_ACTIVE
    }
    #[doc = "Checks if the value of the field is `GPT3B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == EV_A::GPT3B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT3A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == EV_A::GPT3A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == EV_A::GPT2B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT2A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == EV_A::GPT2A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == EV_A::GPT1B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT1A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == EV_A::GPT1A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0B_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == EV_A::GPT0B_DMABREQ
    }
    #[doc = "Checks if the value of the field is `GPT0A_DMABREQ`"]
    #[inline(always)]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == EV_A::GPT0A_DMABREQ
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == EV_A::NONE
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDMACH10BSEL_SPEC, u8, EV_A, 8, O>;
impl<'a, const O: u8> EV_W<'a, O> {
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut W {
        self.variant(EV_A::ALWAYS_ACTIVE)
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT3B_DMABREQ)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT3A_DMABREQ)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT2B_DMABREQ)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT2A_DMABREQ)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT1B_DMABREQ)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT1A_DMABREQ)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0b_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT0B_DMABREQ)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0a_dmabreq(self) -> &'a mut W {
        self.variant(EV_A::GPT0A_DMABREQ)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(EV_A::NONE)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EV_R {
        EV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EV_W<0> {
        EV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udmach10bsel](index.html) module"]
pub struct UDMACH10BSEL_SPEC;
impl crate::RegisterSpec for UDMACH10BSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udmach10bsel::R](R) reader structure"]
impl crate::Readable for UDMACH10BSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udmach10bsel::W](W) writer structure"]
impl crate::Writable for UDMACH10BSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDMACH10BSEL to value 0x4e"]
impl crate::Resettable for UDMACH10BSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4e;
}
