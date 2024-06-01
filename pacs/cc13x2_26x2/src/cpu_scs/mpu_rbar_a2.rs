#[doc = "Register `MPU_RBAR_A2` reader"]
pub type R = crate::R<MpuRbarA2Spec>;
#[doc = "Register `MPU_RBAR_A2` writer"]
pub type W = crate::W<MpuRbarA2Spec>;
#[doc = "Field `MPU_RBAR_A2` reader - 31:0\\]
Alias for MPU_RBAR"]
pub type MpuRbarA2R = crate::FieldReader<u32>;
#[doc = "Field `MPU_RBAR_A2` writer - 31:0\\]
Alias for MPU_RBAR"]
pub type MpuRbarA2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    pub fn mpu_rbar_a2(&self) -> MpuRbarA2R {
        MpuRbarA2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Alias for MPU_RBAR"]
    #[inline(always)]
    #[must_use]
    pub fn mpu_rbar_a2(&mut self) -> MpuRbarA2W<MpuRbarA2Spec> {
        MpuRbarA2W::new(self, 0)
    }
}
#[doc = "MPU Alias 2 Region Base Address Alias for MPU_RBAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rbar_a2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rbar_a2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRbarA2Spec;
impl crate::RegisterSpec for MpuRbarA2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rbar_a2::R`](R) reader structure"]
impl crate::Readable for MpuRbarA2Spec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rbar_a2::W`](W) writer structure"]
impl crate::Writable for MpuRbarA2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RBAR_A2 to value 0"]
impl crate::Resettable for MpuRbarA2Spec {
    const RESET_VALUE: u32 = 0;
}
