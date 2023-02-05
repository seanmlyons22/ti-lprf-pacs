#[doc = "Register `SYNC` reader"]
pub struct R(crate::R<SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC` writer"]
pub struct W(crate::W<SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_SPEC>;
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
impl From<crate::W<SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WBUSY` reader - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_MF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
pub type WBUSY_R = crate::BitReader<bool>;
#[doc = "Field `WBUSY` writer - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_MF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
pub type WBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYNC_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_MF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
    #[inline(always)]
    pub fn wbusy(&self) -> WBUSY_R {
        WBUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_MF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
    #[inline(always)]
    #[must_use]
    pub fn wbusy(&mut self) -> WBUSY_W<0> {
        WBUSY_W::new(self)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](index.html) module"]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync::R](R) reader structure"]
impl crate::Readable for SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync::W](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
