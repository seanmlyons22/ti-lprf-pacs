#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `WBUSY` reader - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
pub type WbusyR = crate::BitReader;
#[doc = "Field `WBUSY` writer - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
pub type WbusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
    #[inline(always)]
    pub fn wbusy(&self) -> WbusyR {
        WbusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This register will always return 0,- however it will not return the value until there are no outstanding write requests between MCU and AON Note: Writing to this register prior to reading will force a wait until next SCLK_LF edge. This is recommended for syncing read registers from AON when waking up from sleep Failure to do so may result in reading AON values from prior to going to sleep"]
    #[inline(always)]
    #[must_use]
    pub fn wbusy(&mut self) -> WbusyW<SyncSpec> {
        WbusyW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SyncSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SyncSpec {
    const RESET_VALUE: u32 = 0;
}
