#[doc = "Register `USER_ID` reader"]
pub struct R(crate::R<USER_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USER_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USER_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USER_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USER_ID` writer"]
pub struct W(crate::W<USER_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USER_ID_SPEC>;
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
impl From<crate::W<USER_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USER_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED0` writer - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u16, u16, 12, O>;
#[doc = "Field `PROTOCOL` reader - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
pub type PROTOCOL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROTOCOL` writer - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
pub type PROTOCOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u8, u8, 4, O>;
#[doc = "Field `PKG` reader - 18:16\\]
Package type. 0x2: 7x7mm QFN (RGZ) package 0x7: 8x8mm QFN (RSK) package Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
pub type PKG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PKG` writer - 18:16\\]
Package type. 0x2: 7x7mm QFN (RGZ) package 0x7: 8x8mm QFN (RSK) package Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
pub type PKG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u8, u8, 3, O>;
#[doc = "Field `SEQUENCE` reader - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
pub type SEQUENCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEQUENCE` writer - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
pub type SEQUENCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u8, u8, 4, O>;
#[doc = "Field `CC13` reader - 23:23\\]
0: CC26x4x10 device type 1: CC13x4x10 device type"]
pub type CC13_R = crate::BitReader<bool>;
#[doc = "Field `CC13` writer - 23:23\\]
0: CC26x4x10 device type 1: CC13x4x10 device type"]
pub type CC13_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_ID_SPEC, bool, O>;
#[doc = "Field `RESERVED24` reader - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED24` writer - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_ID_SPEC, bool, O>;
#[doc = "Field `PA` reader - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
pub type PA_R = crate::BitReader<bool>;
#[doc = "Field `PA` writer - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
pub type PA_W<'a, const O: u8> = crate::BitWriter<'a, u32, USER_ID_SPEC, bool, O>;
#[doc = "Field `VER` reader - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
pub type VER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VER` writer - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
pub type VER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u8, u8, 2, O>;
#[doc = "Field `PG_REV` reader - 31:28\\]
Field used to distinguish revisions of the device"]
pub type PG_REV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PG_REV` writer - 31:28\\]
Field used to distinguish revisions of the device"]
pub type PG_REV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USER_ID_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    pub fn protocol(&self) -> PROTOCOL_R {
        PROTOCOL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x2: 7x7mm QFN (RGZ) package 0x7: 8x8mm QFN (RSK) package Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    pub fn pkg(&self) -> PKG_R {
        PKG_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    pub fn sequence(&self) -> SEQUENCE_R {
        SEQUENCE_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
0: CC26x4x10 device type 1: CC13x4x10 device type"]
    #[inline(always)]
    pub fn cc13(&self) -> CC13_R {
        CC13_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device"]
    #[inline(always)]
    pub fn pg_rev(&self) -> PG_REV_R {
        PG_REV_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Protocols supported. 0x1: BLE 0x2: RF4CE 0x4: Zigbee/6lowpan 0x8: Proprietary More than one protocol can be supported on same device - values above are then combined."]
    #[inline(always)]
    #[must_use]
    pub fn protocol(&mut self) -> PROTOCOL_W<12> {
        PROTOCOL_W::new(self)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Package type. 0x2: 7x7mm QFN (RGZ) package 0x7: 8x8mm QFN (RSK) package Other values are reserved for future use. Packages available for a specific device are shown in the device datasheet."]
    #[inline(always)]
    #[must_use]
    pub fn pkg(&mut self) -> PKG_W<16> {
        PKG_W::new(self)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Sequence. Used to differentiate between marketing/orderable product where other fields of this register are the same (temp range, flash size, voltage range etc)"]
    #[inline(always)]
    #[must_use]
    pub fn sequence(&mut self) -> SEQUENCE_W<19> {
        SEQUENCE_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
0: CC26x4x10 device type 1: CC13x4x10 device type"]
    #[inline(always)]
    #[must_use]
    pub fn cc13(&mut self) -> CC13_W<23> {
        CC13_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Does not support 20dBm PA 1: Supports 20dBM PA"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<25> {
        PA_W::new(self)
    }
    #[doc = "Bits 26:27 - 27:26\\]
Version number. 0x0: Bits \\[25:12\\]
of this register has the stated meaning. Any other setting indicate a different encoding of these bits."]
    #[inline(always)]
    #[must_use]
    pub fn ver(&mut self) -> VER_W<26> {
        VER_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Field used to distinguish revisions of the device"]
    #[inline(always)]
    #[must_use]
    pub fn pg_rev(&mut self) -> PG_REV_W<28> {
        PG_REV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "User Identification. Reading this register and the FCFG1:ICEPICK_DEVICE_ID register is the only supported way of identifying a device. The value of this register will be written to AON_PMCTL:JTAGUSERCODE by boot FW while in safezone.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [user_id](index.html) module"]
pub struct USER_ID_SPEC;
impl crate::RegisterSpec for USER_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [user_id::R](R) reader structure"]
impl crate::Readable for USER_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [user_id::W](W) writer structure"]
impl crate::Writable for USER_ID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USER_ID to value 0x1000_0000"]
impl crate::Resettable for USER_ID_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
