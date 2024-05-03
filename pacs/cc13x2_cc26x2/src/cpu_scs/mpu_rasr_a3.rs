#[doc = "Register `MPU_RASR_A3` reader"]
pub type R = crate::R<MpuRasrA3Spec>;
#[doc = "Register `MPU_RASR_A3` writer"]
pub type W = crate::W<MpuRasrA3Spec>;
#[doc = "Field `MPU_RASR_A3` reader - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA3R = crate::FieldReader<u32>;
#[doc = "Field `MPU_RASR_A3` writer - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a3(&self) -> MpuRasrA3R {
        MpuRasrA3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rasr_a3(&mut self) -> MpuRasrA3W<MpuRasrA3Spec> {
        MpuRasrA3W::new(self, 0)
    }
}
#[doc = "MPU Alias 3 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRasrA3Spec;
impl crate::RegisterSpec for MpuRasrA3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr_a3::R`](R) reader structure"]
impl crate::Readable for MpuRasrA3Spec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr_a3::W`](W) writer structure"]
impl crate::Writable for MpuRasrA3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A3 to value 0"]
impl crate::Resettable for MpuRasrA3Spec {
    const RESET_VALUE: u32 = 0;
}
