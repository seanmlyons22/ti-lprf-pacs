#[doc = "Register `USER_ID` reader"]
pub type R = crate::R<UserIdSpec>;
#[doc = "Register `USER_ID` writer"]
pub type W = crate::W<UserIdSpec>;
#[doc = "Field `RESERVED0` reader - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `PROTOCOL` reader - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
pub type ProtocolR = crate::FieldReader;
#[doc = "Field `PKG` reader - 18:16\\]
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
pub type PkgR = crate::FieldReader;
#[doc = "Field `SEQUENCE` reader - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
pub type SequenceR = crate::FieldReader;
#[doc = "Field `CC13` reader - 23:23\\]
0: CC26xx device type 1: CC13xx device type"]
pub type Cc13R = crate::BitReader;
#[doc = "Field `RESERVED24` reader - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::BitReader;
#[doc = "Field `PA` reader - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
pub type PaR = crate::BitReader;
#[doc = "Field `VER` reader - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
pub type VerR = crate::FieldReader;
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device"]
pub type PgRevR = crate::FieldReader;
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
Package type. 0x0: 4x4mm QFN (RHB) package 0x1: 5x5mm QFN (RSM) package 0x2: 7x7mm QFN (RGZ) package 0x3: Wafer sale package (naked die) 0x4: WCSP (YFV) 0x5: 7x7mm QFN package with Wettable Flanks Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub fn pkg(&self) -> PkgR {
        PkgR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    pub fn sequence(&self) -> SequenceR {
        SequenceR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
0: CC26xx device type 1: CC13xx device type"]
    #[inline(always)]
    pub fn cc13(&self) -> Cc13R {
        Cc13R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub fn ver(&self) -> VerR {
        VerR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device"]
    #[inline(always)]
    pub fn pg_rev(&self) -> PgRevR {
        PgRevR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`user_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`user_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets USER_ID to value 0x1000_0000"]
impl crate::Resettable for UserIdSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
