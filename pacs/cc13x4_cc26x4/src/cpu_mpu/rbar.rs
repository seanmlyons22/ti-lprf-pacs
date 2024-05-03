#[doc = "Register `RBAR` reader"]
pub type R = crate::R<RbarSpec>;
#[doc = "Register `RBAR` writer"]
pub type W = crate::W<RbarSpec>;
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
    pub fn xn(&mut self) -> XnW<RbarSpec> {
        XnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - 2:1\\]
Defines the access permissions for this region"]
    #[inline(always)]
    #[must_use]
    pub fn ap(&mut self) -> ApW<RbarSpec> {
        ApW::new(self, 1)
    }
    #[doc = "Bits 3:4 - 4:3\\]
Defines the Shareability domain of this region for Normal memory"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> ShW<RbarSpec> {
        ShW::new(self, 3)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Contains bits \\[31:5\\]
of the lower inclusive limit of the selected MPU memory region. This value is zero extended to provide the base address to be checked against"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BaseW<RbarSpec> {
        BaseW::new(self, 5)
    }
}
#[doc = "Provides indirect read and write access to the base address of the currently selected MPU region\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rbar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RbarSpec;
impl crate::RegisterSpec for RbarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rbar::R`](R) reader structure"]
impl crate::Readable for RbarSpec {}
#[doc = "`write(|w| ..)` method takes [`rbar::W`](W) writer structure"]
impl crate::Writable for RbarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RBAR to value 0"]
impl crate::Resettable for RbarSpec {
    const RESET_VALUE: u32 = 0;
}
