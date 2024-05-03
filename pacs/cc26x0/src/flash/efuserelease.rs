#[doc = "Register `EFUSERELEASE` reader"]
pub type R = crate::R<EfusereleaseSpec>;
#[doc = "Register `EFUSERELEASE` writer"]
pub type W = crate::W<EfusereleaseSpec>;
#[doc = "Field `EFUSEDAY` reader - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type EfusedayR = crate::FieldReader;
#[doc = "Field `EFUSEDAY` writer - 4:0\\]
Internal. Only to be used through TI provided API."]
pub type EfusedayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFUSEMONTH` reader - 8:5\\]
Internal. Only to be used through TI provided API."]
pub type EfusemonthR = crate::FieldReader;
#[doc = "Field `EFUSEMONTH` writer - 8:5\\]
Internal. Only to be used through TI provided API."]
pub type EfusemonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EFUSEYEAR` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type EfuseyearR = crate::FieldReader;
#[doc = "Field `EFUSEYEAR` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type EfuseyearW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ODPDAY` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type OdpdayR = crate::FieldReader;
#[doc = "Field `ODPDAY` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type OdpdayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ODPMONTH` reader - 24:21\\]
Internal. Only to be used through TI provided API."]
pub type OdpmonthR = crate::FieldReader;
#[doc = "Field `ODPMONTH` writer - 24:21\\]
Internal. Only to be used through TI provided API."]
pub type OdpmonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODPYEAR` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type OdpyearR = crate::FieldReader;
#[doc = "Field `ODPYEAR` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type OdpyearW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseday(&self) -> EfusedayR {
        EfusedayR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efusemonth(&self) -> EfusemonthR {
        EfusemonthR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn efuseyear(&self) -> EfuseyearR {
        EfuseyearR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpday(&self) -> OdpdayR {
        OdpdayR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpmonth(&self) -> OdpmonthR {
        OdpmonthR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn odpyear(&self) -> OdpyearR {
        OdpyearR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuseday(&mut self) -> EfusedayW<EfusereleaseSpec> {
        EfusedayW::new(self, 0)
    }
    #[doc = "Bits 5:8 - 8:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efusemonth(&mut self) -> EfusemonthW<EfusereleaseSpec> {
        EfusemonthW::new(self, 5)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn efuseyear(&mut self) -> EfuseyearW<EfusereleaseSpec> {
        EfuseyearW::new(self, 9)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpday(&mut self) -> OdpdayW<EfusereleaseSpec> {
        OdpdayW::new(self, 16)
    }
    #[doc = "Bits 21:24 - 24:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpmonth(&mut self) -> OdpmonthW<EfusereleaseSpec> {
        OdpmonthW::new(self, 21)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn odpyear(&mut self) -> OdpyearW<EfusereleaseSpec> {
        OdpyearW::new(self, 25)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`efuserelease::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`efuserelease::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfusereleaseSpec;
impl crate::RegisterSpec for EfusereleaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuserelease::R`](R) reader structure"]
impl crate::Readable for EfusereleaseSpec {}
#[doc = "`write(|w| ..)` method takes [`efuserelease::W`](W) writer structure"]
impl crate::Writable for EfusereleaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EFUSERELEASE to value 0"]
impl crate::Resettable for EfusereleaseSpec {
    const RESET_VALUE: u32 = 0;
}
