#[doc = "Register `RBAR_A2` reader"]
pub type R = crate::R<RbarA2Spec>;
#[doc = "Register `RBAR_A2` writer"]
pub type W = crate::W<RbarA2Spec>;
#[doc = "Field `XN` reader - 0:0\\]
Defines whether code can be executed from this region"]
pub type XnR = crate::BitReader;
#[doc = "Field `XN` writer - 0:0\\]
Defines whether code can be executed from this region"]
pub type XnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AP` reader - 2:1\\]
Defines the access permissions for this region"]
pub type ApR = crate::FieldReader;
#[doc = "Field `AP` writer - 2:1\\]
Defines the access permissions for this region"]
pub type ApW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SH` reader - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
pub type ShR = crate::FieldReader;
#[doc = "Field `SH` writer - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
pub type ShW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BASE` reader - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BaseR = crate::FieldReader<u32>;
#[doc = "Field `BASE` writer - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Defines whether code can be executed from this region"]
    #[inline(always)]
    pub fn xn(&self) -> XnR {
        XnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the access permissions for this region"]
    #[inline(always)]
    pub fn ap(&self) -> ApR {
        ApR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    pub fn sh(&self) -> ShR {
        ShR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Defines whether code can be executed from this region"]
    #[inline(always)]
    #[must_use]
    pub fn xn(&mut self) -> XnW<RbarA2Spec> {
        XnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the access permissions for this region"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<RbarA2Spec> {
        ApW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> ShW<RbarA2Spec> {
        ShW::new(self, 3)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<RbarA2Spec> {
        BaseW::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the base address of the MPU region selected by MPU_RNR\\[7:2\\]:(2\\[1:0\\])\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar_a2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar_a2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbarA2Spec;
impl crate::RegisterSpec for RbarA2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbar_a2::R`](R) reader structure"]
impl crate::Readable for RbarA2Spec {}
#[doc = "`write(|w| ..)` method takes [`rbar_a2::W`](W) writer structure"]
impl crate::Writable for RbarA2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBAR_A2 to value 0"]
impl crate::Resettable for RbarA2Spec {
    const RESET_VALUE: u32 = 0;
}
