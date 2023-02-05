#[doc = "Register `DMACH1CTL` reader"]
pub struct R(crate::R<DMACH1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMACH1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMACH1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMACH1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMACH1CTL` writer"]
pub struct W(crate::W<DMACH1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMACH1CTL_SPEC>;
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
impl From<crate::W<DMACH1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMACH1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "1: Channel enabled"]
    EN = 1,
    #[doc = "0: Channel disabled"]
    DIS = 0,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            true => EN_A::EN,
            false => EN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == EN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == EN_A::DIS
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACH1CTL_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(EN_A::EN)
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(EN_A::DIS)
    }
}
#[doc = "Field `PRIO` reader - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub type PRIO_R = crate::BitReader<PRIO_A>;
#[doc = "1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIO_A {
    #[doc = "1: Priority high"]
    HIGH = 1,
    #[doc = "0: Priority low"]
    LOW = 0,
}
impl From<PRIO_A> for bool {
    #[inline(always)]
    fn from(variant: PRIO_A) -> Self {
        variant as u8 != 0
    }
}
impl PRIO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRIO_A {
        match self.bits {
            true => PRIO_A::HIGH,
            false => PRIO_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PRIO_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PRIO_A::LOW
    }
}
#[doc = "Field `PRIO` writer - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub type PRIO_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMACH1CTL_SPEC, PRIO_A, O>;
impl<'a, const O: u8> PRIO_W<'a, O> {
    #[doc = "Priority high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PRIO_A::HIGH)
    }
    #[doc = "Priority low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PRIO_A::LOW)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DMACH1CTL_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<1> {
        PRIO_W::new(self)
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
#[doc = "DMA Channel 1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmach1ctl](index.html) module"]
pub struct DMACH1CTL_SPEC;
impl crate::RegisterSpec for DMACH1CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmach1ctl::R](R) reader structure"]
impl crate::Readable for DMACH1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmach1ctl::W](W) writer structure"]
impl crate::Writable for DMACH1CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMACH1CTL to value 0"]
impl crate::Resettable for DMACH1CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
