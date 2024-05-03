#[doc = "Register `DMACH1CTL` reader"]
pub type R = crate::R<Dmach1ctlSpec>;
#[doc = "Register `DMACH1CTL` writer"]
pub type W = crate::W<Dmach1ctlSpec>;
#[doc = "0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "1: Channel enabled"]
    En = 1,
    #[doc = "0: Channel disabled"]
    Dis = 0,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            true => En::En,
            false => En::Dis,
        }
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == En::En
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == En::Dis
    }
}
#[doc = "Field `EN` writer - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(En::En)
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(En::Dis)
    }
}
#[doc = "1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prio {
    #[doc = "1: Priority high"]
    High = 1,
    #[doc = "0: Priority low"]
    Low = 0,
}
impl From<Prio> for bool {
    #[inline(always)]
    fn from(variant: Prio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIO` reader - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub type PrioR = crate::BitReader<Prio>;
impl PrioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prio {
        match self.bits {
            true => Prio::High,
            false => Prio::Low,
        }
    }
    #[doc = "Priority high"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Prio::High
    }
    #[doc = "Priority low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Prio::Low
    }
}
#[doc = "Field `PRIO` writer - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
pub type PrioW<'a, REG> = crate::BitWriter<'a, REG, Prio>;
impl<'a, REG> PrioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Priority high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Prio::High)
    }
    #[doc = "Priority low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Prio::Low)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel enable: Note: Disabling an active channel will interrupt the DMA operation. The ongoing block transfer will be completed, but no new transfers will be requested."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Dmach1ctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority: A channel with high priority will be served before a channel with low priority in cases with simultaneous access requests. If both channels have the same priority access of the channels to the external port is arbitrated using a Round Robin scheme."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Dmach1ctlSpec> {
        PrioW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Dmach1ctlSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "DMA Channel 1 Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach1ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach1ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach1ctlSpec;
impl crate::RegisterSpec for Dmach1ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach1ctl::R`](R) reader structure"]
impl crate::Readable for Dmach1ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach1ctl::W`](W) writer structure"]
impl crate::Writable for Dmach1ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH1CTL to value 0"]
impl crate::Resettable for Dmach1ctlSpec {
    const RESET_VALUE: u32 = 0;
}
