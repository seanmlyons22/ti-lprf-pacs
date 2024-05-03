#[doc = "Register `AUXSEL0` reader"]
pub type R = crate::R<Auxsel0Spec>;
#[doc = "Register `AUXSEL0` writer"]
pub type W = crate::W<Auxsel0Spec>;
#[doc = "6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior.\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "121: Always asserted"]
    AlwaysActive = 121,
    #[doc = "68: GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    Gpt3bCmp = 68,
    #[doc = "67: GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    Gpt3aCmp = 67,
    #[doc = "66: GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    Gpt2bCmp = 66,
    #[doc = "65: GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    Gpt2aCmp = 65,
    #[doc = "64: GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    Gpt1bCmp = 64,
    #[doc = "63: GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    Gpt1aCmp = 63,
    #[doc = "62: GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    Gpt0bCmp = 62,
    #[doc = "61: GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    Gpt0aCmp = 61,
    #[doc = "19: GPT1B interrupt event, controlled by GPT1:TBMR"]
    Gpt1b = 19,
    #[doc = "18: GPT1A interrupt event, controlled by GPT1:TAMR"]
    Gpt1a = 18,
    #[doc = "17: GPT0B interrupt event, controlled by GPT0:TBMR"]
    Gpt0b = 17,
    #[doc = "16: GPT0A interrupt event, controlled by GPT0:TAMR"]
    Gpt0a = 16,
    #[doc = "15: GPT3B interrupt event, controlled by GPT3:TBMR"]
    Gpt3b = 15,
    #[doc = "14: GPT3A interrupt event, controlled by GPT3:TAMR"]
    Gpt3a = 14,
    #[doc = "13: GPT2B interrupt event, controlled by GPT2:TBMR"]
    Gpt2b = 13,
    #[doc = "12: GPT2A interrupt event, controlled by GPT2:TAMR"]
    Gpt2a = 12,
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
#[doc = "Field `EV` reader - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            121 => Some(Ev::AlwaysActive),
            68 => Some(Ev::Gpt3bCmp),
            67 => Some(Ev::Gpt3aCmp),
            66 => Some(Ev::Gpt2bCmp),
            65 => Some(Ev::Gpt2aCmp),
            64 => Some(Ev::Gpt1bCmp),
            63 => Some(Ev::Gpt1aCmp),
            62 => Some(Ev::Gpt0bCmp),
            61 => Some(Ev::Gpt0aCmp),
            19 => Some(Ev::Gpt1b),
            18 => Some(Ev::Gpt1a),
            17 => Some(Ev::Gpt0b),
            16 => Some(Ev::Gpt0a),
            15 => Some(Ev::Gpt3b),
            14 => Some(Ev::Gpt3a),
            13 => Some(Ev::Gpt2b),
            12 => Some(Ev::Gpt2a),
            0 => Some(Ev::None),
            _ => None,
        }
    }
    #[doc = "Always asserted"]
    #[inline(always)]
    pub fn is_always_active(&self) -> bool {
        *self == Ev::AlwaysActive
    }
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3b_cmp(&self) -> bool {
        *self == Ev::Gpt3bCmp
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt3a_cmp(&self) -> bool {
        *self == Ev::Gpt3aCmp
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2b_cmp(&self) -> bool {
        *self == Ev::Gpt2bCmp
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt2a_cmp(&self) -> bool {
        *self == Ev::Gpt2aCmp
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt1b_cmp(&self) -> bool {
        *self == Ev::Gpt1bCmp
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt1a_cmp(&self) -> bool {
        *self == Ev::Gpt1aCmp
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0b_cmp(&self) -> bool {
        *self == Ev::Gpt0bCmp
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn is_gpt0a_cmp(&self) -> bool {
        *self == Ev::Gpt0aCmp
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn is_gpt1b(&self) -> bool {
        *self == Ev::Gpt1b
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn is_gpt1a(&self) -> bool {
        *self == Ev::Gpt1a
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn is_gpt0b(&self) -> bool {
        *self == Ev::Gpt0b
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn is_gpt0a(&self) -> bool {
        *self == Ev::Gpt0a
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn is_gpt3b(&self) -> bool {
        *self == Ev::Gpt3b
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn is_gpt3a(&self) -> bool {
        *self == Ev::Gpt3a
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn is_gpt2b(&self) -> bool {
        *self == Ev::Gpt2b
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn is_gpt2a(&self) -> bool {
        *self == Ev::Gpt2a
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ev::None
    }
}
#[doc = "Field `EV` writer - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 7, Ev>;
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
    #[doc = "GPT3B compare event. Configured by GPT3:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt3b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3bCmp)
    }
    #[doc = "GPT3A compare event. Configured by GPT3:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt3a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3aCmp)
    }
    #[doc = "GPT2B compare event. Configured by GPT2:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt2b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2bCmp)
    }
    #[doc = "GPT2A compare event. Configured by GPT2:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt2a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2aCmp)
    }
    #[doc = "GPT1B compare event. Configured by GPT1:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt1b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1bCmp)
    }
    #[doc = "GPT1A compare event. Configured by GPT1:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt1a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1aCmp)
    }
    #[doc = "GPT0B compare event. Configured by GPT0:TBMR.TCACT"]
    #[inline(always)]
    pub fn gpt0b_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0bCmp)
    }
    #[doc = "GPT0A compare event. Configured by GPT0:TAMR.TCACT"]
    #[inline(always)]
    pub fn gpt0a_cmp(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0aCmp)
    }
    #[doc = "GPT1B interrupt event, controlled by GPT1:TBMR"]
    #[inline(always)]
    pub fn gpt1b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1b)
    }
    #[doc = "GPT1A interrupt event, controlled by GPT1:TAMR"]
    #[inline(always)]
    pub fn gpt1a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt1a)
    }
    #[doc = "GPT0B interrupt event, controlled by GPT0:TBMR"]
    #[inline(always)]
    pub fn gpt0b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0b)
    }
    #[doc = "GPT0A interrupt event, controlled by GPT0:TAMR"]
    #[inline(always)]
    pub fn gpt0a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt0a)
    }
    #[doc = "GPT3B interrupt event, controlled by GPT3:TBMR"]
    #[inline(always)]
    pub fn gpt3b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3b)
    }
    #[doc = "GPT3A interrupt event, controlled by GPT3:TAMR"]
    #[inline(always)]
    pub fn gpt3a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt3a)
    }
    #[doc = "GPT2B interrupt event, controlled by GPT2:TBMR"]
    #[inline(always)]
    pub fn gpt2b(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2b)
    }
    #[doc = "GPT2A interrupt event, controlled by GPT2:TAMR"]
    #[inline(always)]
    pub fn gpt2a(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Gpt2a)
    }
    #[doc = "Always inactive"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::None)
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Read/write selection value Writing any other value than values defined by a ENUM may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Auxsel0Spec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for AUX Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxsel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxsel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Auxsel0Spec;
impl crate::RegisterSpec for Auxsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`auxsel0::R`](R) reader structure"]
impl crate::Readable for Auxsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`auxsel0::W`](W) writer structure"]
impl crate::Writable for Auxsel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUXSEL0 to value 0x10"]
impl crate::Resettable for Auxsel0Spec {
    const RESET_VALUE: u32 = 0x10;
}
