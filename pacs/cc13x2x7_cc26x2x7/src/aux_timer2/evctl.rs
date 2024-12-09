#[doc = "Register `EVCTL` reader"]
pub type R = crate::R<EvctlSpec>;
#[doc = "Register `EVCTL` writer"]
pub type W = crate::W<EvctlSpec>;
#[doc = "Field `EV0_CLR` writer - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
pub type Ev0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV0_SET` writer - 1:1\\]
Set event 0. Write 1 to set event 0."]
pub type Ev0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV1_CLR` writer - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
pub type Ev1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV1_SET` writer - 3:3\\]
Set event 1. Write 1 to set event 1."]
pub type Ev1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV2_CLR` writer - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
pub type Ev2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV2_SET` writer - 5:5\\]
Set event 2. Write 1 to set event 2."]
pub type Ev2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV3_CLR` writer - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
pub type Ev3ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EV3_SET` writer - 7:7\\]
Set event 3. Write 1 to set event 3."]
pub type Ev3SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_clr(&mut self) -> Ev0ClrW<EvctlSpec> {
        Ev0ClrW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set event 0. Write 1 to set event 0."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_set(&mut self) -> Ev0SetW<EvctlSpec> {
        Ev0SetW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_clr(&mut self) -> Ev1ClrW<EvctlSpec> {
        Ev1ClrW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Set event 1. Write 1 to set event 1."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_set(&mut self) -> Ev1SetW<EvctlSpec> {
        Ev1SetW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_clr(&mut self) -> Ev2ClrW<EvctlSpec> {
        Ev2ClrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set event 2. Write 1 to set event 2."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_set(&mut self) -> Ev2SetW<EvctlSpec> {
        Ev2SetW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_clr(&mut self) -> Ev3ClrW<EvctlSpec> {
        Ev3ClrW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Set event 3. Write 1 to set event 3."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_set(&mut self) -> Ev3SetW<EvctlSpec> {
        Ev3SetW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<EvctlSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctlSpec;
impl crate::RegisterSpec for EvctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctl::R`](R) reader structure"]
impl crate::Readable for EvctlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctl::W`](W) writer structure"]
impl crate::Writable for EvctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVCTL to value 0"]
impl crate::Resettable for EvctlSpec {
    const RESET_VALUE: u32 = 0;
}
