#[doc = "Register `MPU_RNR` reader"]
pub type R = crate::R<MpuRnrSpec>;
#[doc = "Register `MPU_RNR` writer"]
pub type W = crate::W<MpuRnrSpec>;
#[doc = "Field `REGION` reader - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
pub type RegionR = crate::FieldReader;
#[doc = "Field `REGION` writer - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
pub type RegionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    pub fn region(&self) -> RegionR {
        RegionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Region select field. This field selects the region to operate on when using the MPU_RASR and MPU_RBAR. It must be written first except when the address MPU_RBAR.VALID and MPU_RBAR.REGION fields are written, which overwrites this."]
    #[inline(always)]
    #[must_use]
    pub fn region(&mut self) -> RegionW<MpuRnrSpec> {
        RegionW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<MpuRnrSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "MPU Region Number This register is used to select which protection region is accessed. The following write to MPU_RASR or MPU_RBAR configures the characteristics of the protection region that is selected by this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mpu_rnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mpu_rnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MpuRnrSpec;
impl crate::RegisterSpec for MpuRnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mpu_rnr::R`](R) reader structure"]
impl crate::Readable for MpuRnrSpec {}
#[doc = "`write(|w| ..)` method takes [`mpu_rnr::W`](W) writer structure"]
impl crate::Writable for MpuRnrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MPU_RNR to value 0"]
impl crate::Resettable for MpuRnrSpec {
    const RESET_VALUE: u32 = 0;
}
