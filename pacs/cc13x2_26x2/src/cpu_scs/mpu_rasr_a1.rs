#[doc = "Register `MPU_RASR_A1` reader"]
pub type R = crate::R<MpuRasrA1Spec>;
#[doc = "Register `MPU_RASR_A1` writer"]
pub type W = crate::W<MpuRasrA1Spec>;
#[doc = "Field `MPU_RASR_A1` reader - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA1R = crate::FieldReader<u32>;
#[doc = "Field `MPU_RASR_A1` writer - 31:0\\]
Alias for MPU_RASR"]
pub type MpuRasrA1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    pub fn mpu_rasr_a1(&self) -> MpuRasrA1R {
        MpuRasrA1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RASR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rasr_a1(&mut self) -> MpuRasrA1W<MpuRasrA1Spec> {
        MpuRasrA1W::new(self, 0)
    }
}
#[doc = "MPU Alias 1 Region Attribute and Size Alias for MPU_RASR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rasr_a1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rasr_a1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRasrA1Spec;
impl crate::RegisterSpec for MpuRasrA1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rasr_a1::R`](R) reader structure"]
impl crate::Readable for MpuRasrA1Spec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rasr_a1::W`](W) writer structure"]
impl crate::Writable for MpuRasrA1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RASR_A1 to value 0"]
impl crate::Resettable for MpuRasrA1Spec {
    const RESET_VALUE: u32 = 0;
}
