#[doc = "Register `UDMACH11BSEL` reader"]
pub type R = crate::R<Udmach11bselSpec>;
#[doc = "Register `UDMACH11BSEL` writer"]
pub type W = crate::W<Udmach11bselSpec>;
#[doc = "7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 79"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "84: GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    Gpt3bDmabreq = 84,
    #[doc = "83: GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    Gpt3aDmabreq = 83,
    #[doc = "82: GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    Gpt2bDmabreq = 82,
    #[doc = "81: GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    Gpt2aDmabreq = 81,
    #[doc = "80: GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    Gpt1bDmabreq = 80,
    #[doc = "79: GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    Gpt1aDmabreq = 79,
    #[doc = "78: GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    Gpt0bDmabreq = 78,
    #[doc = "77: GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    Gpt0aDmabreq = 77,
    #[doc = "0: Always inactive"]
    None = 0,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            121 => Some(Ev::AlwaysActive),
            84 => Some(Ev::Gpt3bDmabreq),
            83 => Some(Ev::Gpt3aDmabreq),
            82 => Some(Ev::Gpt2bDmabreq),
            81 => Some(Ev::Gpt2aDmabreq),
            80 => Some(Ev::Gpt1bDmabreq),
            79 => Some(Ev::Gpt1aDmabreq),
            78 => Some(Ev::Gpt0bDmabreq),
            77 => Some(Ev::Gpt0aDmabreq),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == Ev::AlwaysActive
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn is_gpt3b_dmabreq(&self) -> bool {
        *self == Ev::Gpt3bDmabreq
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn is_gpt3a_dmabreq(&self) -> bool {
        *self == Ev::Gpt3aDmabreq
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn is_gpt2b_dmabreq(&self) -> bool {
        *self == Ev::Gpt2bDmabreq
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn is_gpt2a_dmabreq(&self) -> bool {
        *self == Ev::Gpt2aDmabreq
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn is_gpt1b_dmabreq(&self) -> bool {
        *self == Ev::Gpt1bDmabreq
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn is_gpt1a_dmabreq(&self) -> bool {
        *self == Ev::Gpt1aDmabreq
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn is_gpt0b_dmabreq(&self) -> bool {
        *self == Ev::Gpt0bDmabreq
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn is_gpt0a_dmabreq(&self) -> bool {
        *self == Ev::Gpt0aDmabreq
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ev::None
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn always_active(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AlwaysActive)
    }
    #[doc = "GPT3B DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3bDmabreq)
    }
    #[doc = "GPT3A DMA trigger event. Configured by GPT3:DMAEV"]
    #[inline(always)]
    pub fn gpt3a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3aDmabreq)
    }
    #[doc = "GPT2B DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2bDmabreq)
    }
    #[doc = "GPT2A DMA trigger event. Configured by GPT2:DMAEV"]
    #[inline(always)]
    pub fn gpt2a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2aDmabreq)
    }
    #[doc = "GPT1B DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1bDmabreq)
    }
    #[doc = "GPT1A DMA trigger event. Configured by GPT1:DMAEV"]
    #[inline(always)]
    pub fn gpt1a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1aDmabreq)
    }
    #[doc = "GPT0B DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0b_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0bDmabreq)
    }
    #[doc = "GPT0A DMA trigger event. Configured by GPT0:DMAEV"]
    #[inline(always)]
    pub fn gpt0a_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0aDmabreq)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::None)
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Udmach11bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach11bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach11bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach11bselSpec;
impl crate::RegisterSpec for Udmach11bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach11bsel::R`](R) reader structure"]
impl crate::Readable for Udmach11bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach11bsel::W`](W) writer structure"]
impl crate::Writable for Udmach11bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH11BSEL to value 0x4f"]
impl crate::Resettable for Udmach11bselSpec {
    const RESET_VALUE: u32 = 0x4f;
}
