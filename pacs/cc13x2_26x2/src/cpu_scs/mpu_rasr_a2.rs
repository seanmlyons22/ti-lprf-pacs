#[doc = "Register `MPU_RASR_A2` reader"]
pub type R = crate::R<MpuRasrA2Spec>;
#[doc = "Register `MPU_RASR_A2` writer"]
pub type W = crate::W<MpuRasrA2Spec>;
#[doc = "Field `MPU_RASR_A2` reader - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA2R = crate::FieldReader<u32>;
#[doc = "Field `MPU_RASR_A2` writer - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a2(&self) -> MpuRasrA2R {
        MpuRasrA2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rasr_a2(&mut self) -> MpuRasrA2W<MpuRasrA2Spec> {
        MpuRasrA2W::new(self, 0)
    }
}
#[doc = "MPU Alias 2 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRasrA2Spec;
impl crate::RegisterSpec for MpuRasrA2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr_a2::R`](R) reader structure"]
impl crate::Readable for MpuRasrA2Spec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr_a2::W`](W) writer structure"]
impl crate::Writable for MpuRasrA2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A2 to value 0"]
impl crate::Resettable for MpuRasrA2Spec {
    const RESET_VALUE: u32 = 0;
}
