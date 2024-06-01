#[doc = "Register `DMACH0CTL` reader"]
pub type R = crate::R<Dmach0ctlSpec>;
#[doc = "Register `DMACH0CTL` writer"]
pub type W = crate::W<Dmach0ctlSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - 1:1\\]
Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PrioR = crate::BitReader;
#[doc = "Field `PRIO` writer - 1:1\\]
Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
pub type PrioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
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
Channel enable 0: Disabled 1: Enable Note: Disabling an active channel interrupts the DMA operation. The ongoing block transfer completes, but no new transfers are requested."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Dmach0ctlSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Channel priority 0: Low 1: High If both channels have the same priority, access of the channels to the external port is arbitrated using the round robin scheme. If one channel has a high priority and another one low, the channel with the high priority is served first, in case of simultaneous access requests."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Dmach0ctlSpec> {
        PrioW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<Dmach0ctlSpec> {
        Reserved2W::new(self, 2)
    }
}
#[doc = "Channel 0 Control This register is used for channel enabling and priority selection. When a channel is disabled, it becomes inactive only when all ongoing requests are finished.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmach0ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmach0ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmach0ctlSpec;
impl crate::RegisterSpec for Dmach0ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmach0ctl::R`](R) reader structure"]
impl crate::Readable for Dmach0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmach0ctl::W`](W) writer structure"]
impl crate::Writable for Dmach0ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACH0CTL to value 0"]
impl crate::Resettable for Dmach0ctlSpec {
    const RESET_VALUE: u32 = 0;
}
