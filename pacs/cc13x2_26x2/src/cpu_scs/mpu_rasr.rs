#[doc = "Register `MPU_RASR` reader"]
pub type R = crate::R<MpuRasrSpec>;
#[doc = "Register `MPU_RASR` writer"]
pub type W = crate::W<MpuRasrSpec>;
#[doc = "Field `ENABLE` reader - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIZE` reader - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `SIZE` writer - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RESERVED6` reader - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader;
#[doc = "Field `RESERVED6` writer - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SRD` reader - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
pub type SrdR = crate::FieldReader;
#[doc = "Field `SRD` writer - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
pub type SrdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `B` reader - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
pub type BR = crate::BitReader;
#[doc = "Field `B` writer - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
pub type BW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C` reader - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
pub type CR = crate::BitReader;
#[doc = "Field `C` writer - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
pub type CW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S` reader - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
pub type SR = crate::BitReader;
#[doc = "Field `S` writer - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
pub type SW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEX` reader - 21:19\\]
Type extension"]
pub type TexR = crate::FieldReader;
#[doc = "Field `TEX` writer - 21:19\\]
Type extension"]
pub type TexW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED22` reader - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22R = crate::FieldReader;
#[doc = "Field `RESERVED22` writer - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved22W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AP` reader - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
pub type ApR = crate::FieldReader;
#[doc = "Field `AP` writer - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
pub type ApW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED27` reader - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27R = crate::BitReader;
#[doc = "Field `RESERVED27` writer - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XN` reader - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
pub type XnR = crate::BitReader;
#[doc = "Field `XN` writer - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
pub type XnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED29` reader - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29R = crate::FieldReader;
#[doc = "Field `RESERVED29` writer - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved29W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    pub fn srd(&self) -> SrdR {
        SrdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    pub fn tex(&self) -> TexR {
        TexR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved27(&self) -> Reserved27R {
        Reserved27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    pub fn xn(&self) -> XnR {
        XnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved29(&self) -> Reserved29R {
        Reserved29R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Region enable bit: 0: Disable region 1: Enable region"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<MpuRasrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:5 - 5:1\\]
MPU Protection Region Size Field: 0x04: 32B 0x05: 64B 0x06: 128B 0x07: 256B 0x08: 512B 0x09: 1KB 0x0A: 2KB 0x0B: 4KB 0x0C: 8KB 0x0D: 16KB 0x0E: 32KB 0x0F: 64KB 0x10: 128KB 0x11: 256KB 0x12: 512KB 0x13: 1MB 0x14: 2MB 0x15: 4MB 0x16: 8MB 0x17: 16MB 0x18: 32MB 0x19: 64MB 0x1A: 128MB 0x1B: 256MB 0x1C: 512MB 0x1D: 1GB 0x1E: 2GB 0x1F: 4GB"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SizeW<MpuRasrSpec> {
        SizeW::new(self, 1)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<MpuRasrSpec> {
        Reserved6W::new(self, 6)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Sub-Region Disable field: Setting a bit in this field disables the corresponding sub-region. Regions are split into eight equal-sized sub-regions. Sub-regions are not supported for region sizes of 128 bytes and less."]
    #[inline(always)]
    #[must_use]
    pub fn srd(&mut self) -> SrdW<MpuRasrSpec> {
        SrdW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Bufferable bit: 0: Not bufferable 1: Bufferable"]
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> BW<MpuRasrSpec> {
        BW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Cacheable bit: 0: Not cacheable 1: Cacheable"]
    #[inline(always)]
    #[must_use]
    pub fn c(&mut self) -> CW<MpuRasrSpec> {
        CW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Shareable bit: 0: Not shareable 1: Shareable"]
    #[inline(always)]
    #[must_use]
    pub fn s(&mut self) -> SW<MpuRasrSpec> {
        SW::new(self, 18)
    }
    #[doc = "Bits 19:21 - 21:19\\]
Type extension"]
    #[inline(always)]
    #[must_use]
    pub fn tex(&mut self) -> TexW<MpuRasrSpec> {
        TexW::new(self, 19)
    }
    #[doc = "Bits 22:23 - 23:22\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved22(&mut self) -> Reserved22W<MpuRasrSpec> {
        Reserved22W::new(self, 22)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Data access permission: 0x0: Priviliged permissions: No access. User permissions: No access. 0x1: Priviliged permissions: Read-write. User permissions: No access. 0x2: Priviliged permissions: Read-write. User permissions: Read-only. 0x3: Priviliged permissions: Read-write. User permissions: Read-write. 0x4: Reserved 0x5: Priviliged permissions: Read-only. User permissions: No access. 0x6: Priviliged permissions: Read-only. User permissions: Read-only. 0x7: Priviliged permissions: Read-only. User permissions: Read-only."]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<MpuRasrSpec> {
        ApW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved27(&mut self) -> Reserved27W<MpuRasrSpec> {
        Reserved27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Instruction access disable: 0: Enable instruction fetches 1: Disable instruction fetches"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XnW<MpuRasrSpec> {
        XnW::new(self, 28)
    }
    #[doc = "Bits 29:31 - 31:29\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved29(&mut self) -> Reserved29W<MpuRasrSpec> {
        Reserved29W::new(self, 29)
    }
}
#[doc = "MPU Region Attribute and Size This register controls the MPU access permissions. The register is made up of two part registers, each of halfword size. These can be accessed using the halfword size, or they can both be simultaneously accessed using a word operation. The sub-region disable bits are not supported for region sizes of 32 bytes, 64 bytes, and 128 bytes. When these region sizes are used, the subregion disable bits must be programmed as 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRasrSpec;
impl crate::RegisterSpec for MpuRasrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr::R`](R) reader structure"]
impl crate::Readable for MpuRasrSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr::W`](W) writer structure"]
impl crate::Writable for MpuRasrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR to value 0"]
impl crate::Resettable for MpuRasrSpec {
    const RESET_VALUE: u32 = 0;
}
