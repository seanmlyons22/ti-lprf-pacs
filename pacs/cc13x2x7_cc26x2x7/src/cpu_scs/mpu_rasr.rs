#[doc = "Register `MPU_RASR` reader"]
pub struct R(crate::R<MPU_RASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_RASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_RASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_RASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_RASR` writer"]
pub struct W(crate::W<MPU_RASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_RASR_SPEC>;
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
impl From<crate::W<MPU_RASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_RASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `SIZE` reader - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SRD` reader - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
pub type SRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRD` writer - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
pub type SRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 8, O>;
#[doc = "Field `B` reader - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
pub type B_R = crate::BitReader<bool>;
#[doc = "Field `B` writer - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
pub type B_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `C` reader - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
pub type C_R = crate::BitReader<bool>;
#[doc = "Field `C` writer - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
pub type C_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `S` reader - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
pub type S_R = crate::BitReader<bool>;
#[doc = "Field `S` writer - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
pub type S_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `TEX` reader - 21:19\\]
Type extension"]
pub type TEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEX` writer - 21:19\\]
Type extension"]
pub type TEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 2, O>;
#[doc = "Field `AP` reader - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
pub type AP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AP` writer - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
pub type AP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESERVED27` reader - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED27` writer - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED27_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `XN` reader - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
pub type XN_R = crate::BitReader<bool>;
#[doc = "Field `XN` writer - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
pub type XN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_RASR_SPEC, bool, O>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MPU_RASR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    pub fn srd(&self) -> SRD_R {
        SRD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    pub fn c(&self) -> C_R {
        C_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    pub fn tex(&self) -> TEX_R {
        TEX_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> RESERVED22_R {
        RESERVED22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> RESERVED27_R {
        RESERVED27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    pub fn xn(&self) -> XN_R {
        XN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> RESERVED29_R {
        RESERVED29_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<1> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SRD_W<8> {
        SRD_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<16> {
        B_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> C_W<17> {
        C_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> S_W<18> {
        S_W::new(self)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    #[must_use]
    pub fn tex(&mut self) -> TEX_W<19> {
        TEX_W::new(self)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> RESERVED22_W<22> {
        RESERVED22_W::new(self)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> AP_W<24> {
        AP_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> RESERVED27_W<27> {
        RESERVED27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XN_W<28> {
        XN_W::new(self)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> RESERVED29_W<29> {
        RESERVED29_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpu_rasr](index.html) module"]
pub struct MPU_RASR_SPEC;
impl crate::RegisterSpec for MPU_RASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_rasr::R](R) reader structure"]
impl crate::Readable for MPU_RASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_rasr::W](W) writer structure"]
impl crate::Writable for MPU_RASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MPU_RASR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
