#[doc = "Register `USER_ID` reader"]
pub type R = crate::R<UserIdSpec>;
#[doc = "Register `USER_ID` writer"]
pub type W = crate::W<UserIdSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PROTOCOL` reader - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
pub type ProtocolR = crate::FieldReader;
#[doc = "Field `PROTOCOL` writer - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
pub type ProtocolW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PKG` reader - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
pub type PkgR = crate::FieldReader;
#[doc = "Field `PKG` writer - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
pub type PkgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SEQUENCE` reader - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)"]
pub type SequenceR = crate::FieldReader;
#[doc = "Field `SEQUENCE` writer - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)"]
pub type SequenceW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED23` reader - 25:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23R = crate::FieldReader;
#[doc = "Field `RESERVED23` writer - 25:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VER` reader - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
pub type VerR = crate::FieldReader;
#[doc = "Field `VER` writer - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
pub type VerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device."]
pub type PgRevR = crate::FieldReader;
#[doc = "Field `PG_REV` writer - 31:28\\]
Field used to distinguish revisions of the device."]
pub type PgRevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    pub fn protocol(&self) -> ProtocolR {
        ProtocolR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub fn pkg(&self) -> PkgR {
        PkgR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    pub fn sequence(&self) -> SequenceR {
        SequenceR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    pub fn pg_rev(&self) -> PgRevR {
        PgRevR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<UserIdSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    #[must_use]
    pub fn protocol(&mut self) -> ProtocolW<UserIdSpec> {
        ProtocolW::new(self, 12)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: 2.7x2.7mm WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn pkg(&mut self) -> PkgW<UserIdSpec> {
        PkgW::new(self, 16)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of USER_ID is the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    #[must_use]
    pub fn sequence(&mut self) -> SequenceW<UserIdSpec> {
        SequenceW::new(self, 19)
    }
    #[doc = "Bits 23:25 - 25:23\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved23(&mut self) -> Reserved23W<UserIdSpec> {
        Reserved23W::new(self, 23)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    #[must_use]
    pub fn ver(&mut self) -> VerW<UserIdSpec> {
        VerW::new(self, 26)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device."]
    #[inline(always)]
    #[must_use]
    pub fn pg_rev(&mut self) -> PgRevW<UserIdSpec> {
        PgRevW::new(self, 28)
    }
}
#[doc = "User Identification. Reading this register and the ICEPICK_DEVICE_ID register is the only support way of identifying a device. The value of this register will be written to AON_WUC:JTAGUSERCODE by boot FW while in safezone.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UserIdSpec;
impl crate::RegisterSpec for UserIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`user_id::R`](R) reader structure"]
impl crate::Readable for UserIdSpec {}
#[doc = "`write(|w| ..)` method takes [`user_id::W`](W) writer structure"]
impl crate::Writable for UserIdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USER_ID to value 0"]
impl crate::Resettable for UserIdSpec {
    const RESET_VALUE: u32 = 0;
}
