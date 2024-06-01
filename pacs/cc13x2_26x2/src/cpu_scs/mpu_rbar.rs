#[doc = "Register `MPU_RBAR` reader"]
pub type R = crate::R<MpuRbarSpec>;
#[doc = "Register `MPU_RBAR` writer"]
pub type W = crate::W<MpuRbarSpec>;
#[doc = "Field `REGION` reader - 3:0\\]
MPU region override field"]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - 3:0\\]
MPU region override field"]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VALID` reader - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR` reader - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
pub type AddrR = crate::FieldReader<u32>;
#[doc = "Field `ADDR` writer - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
MPU region override field"]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
MPU region override field"]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> RegionW<MpuRbarSpec> {
        RegionW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
MPU region number valid: 0: MPU_RNR remains unchanged and is interpreted. 1: MPU_RNR is overwritten by REGION."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<MpuRbarSpec> {
        ValidW::new(self, 4)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Region base address field. The position of the LSB depends on the region size, so that the base address is aligned according to an even multiple of size. The power of 2 size specified by the SZENABLE field of the MPU Region Attribute and Size Register defines how many bits of base address are used."]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<MpuRbarSpec> {
        AddrW::new(self, 5)
    }
}
#[doc = "MPU Region Base Address This register writes the base address of a region. It also contains a REGION field that can be used to override MPU_RNR.REGION, if the VALID bit is set. This register sets the base for the region. It is aligned by the size. So, a 64-KB sized region must be aligned on a multiple of 64KB, for example, 0x00010000 or 0x00020000. The region always reads back as the current MPU region number. VALID always reads back as 0. Writing VALID = 1 and REGION = n changes the region number to n. This is a short-hand way to write the MPU_RNR. This register is unpredictable if accessed other than as a word.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRbarSpec;
impl crate::RegisterSpec for MpuRbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar::R`](R) reader structure"]
impl crate::Readable for MpuRbarSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar::W`](W) writer structure"]
impl crate::Writable for MpuRbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR to value 0"]
impl crate::Resettable for MpuRbarSpec {
    const RESET_VALUE: u32 = 0;
}
