#[doc = "Register `ICEPICK_DEVICE_ID` reader"]
pub type R = crate::R<IcepickDeviceIdSpec>;
#[doc = "Register `ICEPICK_DEVICE_ID` writer"]
pub type W = crate::W<IcepickDeviceIdSpec>;
#[doc = "Field `MANUFACTURER_ID` reader - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
pub type ManufacturerIdR = crate::FieldReader<u16>;
#[doc = "Field `WAFER_ID` reader - 27:12\\]
Field used to identify silicon die."]
pub type WaferIdR = crate::FieldReader<u16>;
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device."]
pub type PgRevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Manufacturer code. 0x02F: Texas Instruments"]
    #[inline(always)]
    pub fn manufacturer_id(&self) -> ManufacturerIdR {
        ManufacturerIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Field used to identify silicon die."]
    #[inline(always)]
    pub fn wafer_id(&self) -> WaferIdR {
        WaferIdR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&self) -> PgRevR {
        PgRevR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "IcePick Device Identification Reading this register and the FCFG1:USER_ID register is the only supported way of identifying a device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icepick_device_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icepick_device_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcepickDeviceIdSpec;
impl crate::RegisterSpec for IcepickDeviceIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icepick_device_id::R`](R) reader structure"]
impl crate::Readable for IcepickDeviceIdSpec {}
#[doc = "`write(|w| ..)` method takes [`icepick_device_id::W`](W) writer structure"]
impl crate::Writable for IcepickDeviceIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICEPICK_DEVICE_ID to value 0x1bb7_802f"]
impl crate::Resettable for IcepickDeviceIdSpec {
    const RESET_VALUE: u32 = 0x1bb7_802f;
}
