#[doc = "Register `MPU_TYPE` reader"]
pub type R = crate::R<MpuTypeSpec>;
#[doc = "Register `MPU_TYPE` writer"]
pub type W = crate::W<MpuTypeSpec>;
#[doc = "Field `SEPARATE` reader - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
pub type SeparateR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Reads 0."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DREGION` reader - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
pub type DregionR = crate::FieldReader;
#[doc = "Field `IREGION` reader - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
pub type IregionR = crate::FieldReader;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Reads 0."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The processor core uses only a unified MPU, thus this field is always 0."]
    #[inline(always)]
    pub fn separate(&self) -> SeparateR {
        SeparateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of supported MPU regions field. This field reads 0x08 indicating eight MPU regions."]
    #[inline(always)]
    pub fn dregion(&self) -> DregionR {
        DregionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
The processor core uses only a unified MPU, this field always reads 0x0."]
    #[inline(always)]
    pub fn iregion(&self) -> IregionR {
        IregionR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reads 0."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "MPU Type This register indicates many regions the MPU supports.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuTypeSpec;
impl crate::RegisterSpec for MpuTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_type::R`](R) reader structure"]
impl crate::Readable for MpuTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_type::W`](W) writer structure"]
impl crate::Writable for MpuTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_TYPE to value 0x0800"]
impl crate::Resettable for MpuTypeSpec {
    const RESET_VALUE: u32 = 0x0800;
}
