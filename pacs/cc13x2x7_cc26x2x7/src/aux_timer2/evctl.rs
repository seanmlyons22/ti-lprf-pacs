#[doc = "Register `EVCTL` reader"]
pub struct R(crate::R<EVCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVCTL` writer"]
pub struct W(crate::W<EVCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVCTL_SPEC>;
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
impl From<crate::W<EVCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EV0_CLR` reader - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
pub type EV0_CLR_R = crate::BitReader<bool>;
#[doc = "Field `EV0_CLR` writer - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
pub type EV0_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV0_SET` reader - 1:1\\]
Set event 0. Write 1 to set event 0."]
pub type EV0_SET_R = crate::BitReader<bool>;
#[doc = "Field `EV0_SET` writer - 1:1\\]
Set event 0. Write 1 to set event 0."]
pub type EV0_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV1_CLR` reader - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
pub type EV1_CLR_R = crate::BitReader<bool>;
#[doc = "Field `EV1_CLR` writer - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
pub type EV1_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV1_SET` reader - 3:3\\]
Set event 1. Write 1 to set event 1."]
pub type EV1_SET_R = crate::BitReader<bool>;
#[doc = "Field `EV1_SET` writer - 3:3\\]
Set event 1. Write 1 to set event 1."]
pub type EV1_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV2_CLR` reader - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
pub type EV2_CLR_R = crate::BitReader<bool>;
#[doc = "Field `EV2_CLR` writer - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
pub type EV2_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV2_SET` reader - 5:5\\]
Set event 2. Write 1 to set event 2."]
pub type EV2_SET_R = crate::BitReader<bool>;
#[doc = "Field `EV2_SET` writer - 5:5\\]
Set event 2. Write 1 to set event 2."]
pub type EV2_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV3_CLR` reader - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
pub type EV3_CLR_R = crate::BitReader<bool>;
#[doc = "Field `EV3_CLR` writer - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
pub type EV3_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `EV3_SET` reader - 7:7\\]
Set event 3. Write 1 to set event 3."]
pub type EV3_SET_R = crate::BitReader<bool>;
#[doc = "Field `EV3_SET` writer - 7:7\\]
Set event 3. Write 1 to set event 3."]
pub type EV3_SET_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVCTL_SPEC, bool, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVCTL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
    #[inline(always)]
    pub fn ev0_clr(&self) -> EV0_CLR_R {
        EV0_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set event 0. Write 1 to set event 0."]
    #[inline(always)]
    pub fn ev0_set(&self) -> EV0_SET_R {
        EV0_SET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
    #[inline(always)]
    pub fn ev1_clr(&self) -> EV1_CLR_R {
        EV1_CLR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set event 1. Write 1 to set event 1."]
    #[inline(always)]
    pub fn ev1_set(&self) -> EV1_SET_R {
        EV1_SET_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
    #[inline(always)]
    pub fn ev2_clr(&self) -> EV2_CLR_R {
        EV2_CLR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set event 2. Write 1 to set event 2."]
    #[inline(always)]
    pub fn ev2_set(&self) -> EV2_SET_R {
        EV2_SET_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
    #[inline(always)]
    pub fn ev3_clr(&self) -> EV3_CLR_R {
        EV3_CLR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set event 3. Write 1 to set event 3."]
    #[inline(always)]
    pub fn ev3_set(&self) -> EV3_SET_R {
        EV3_SET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear event 0. Write 1 to clear event 0."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_clr(&mut self) -> EV0_CLR_W<0> {
        EV0_CLR_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Set event 0. Write 1 to set event 0."]
    #[inline(always)]
    #[must_use]
    pub fn ev0_set(&mut self) -> EV0_SET_W<1> {
        EV0_SET_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear event 1. Write 1 to clear event 1."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_clr(&mut self) -> EV1_CLR_W<2> {
        EV1_CLR_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Set event 1. Write 1 to set event 1."]
    #[inline(always)]
    #[must_use]
    pub fn ev1_set(&mut self) -> EV1_SET_W<3> {
        EV1_SET_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear event 2. Write 1 to clear event 2."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_clr(&mut self) -> EV2_CLR_W<4> {
        EV2_CLR_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Set event 2. Write 1 to set event 2."]
    #[inline(always)]
    #[must_use]
    pub fn ev2_set(&mut self) -> EV2_SET_W<5> {
        EV2_SET_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear event 3. Write 1 to clear event 3."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_clr(&mut self) -> EV3_CLR_W<6> {
        EV3_CLR_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Set event 3. Write 1 to set event 3."]
    #[inline(always)]
    #[must_use]
    pub fn ev3_set(&mut self) -> EV3_SET_W<7> {
        EV3_SET_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evctl](index.html) module"]
pub struct EVCTL_SPEC;
impl crate::RegisterSpec for EVCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evctl::R](R) reader structure"]
impl crate::Readable for EVCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evctl::W](W) writer structure"]
impl crate::Writable for EVCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVCTL to value 0"]
impl crate::Resettable for EVCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
