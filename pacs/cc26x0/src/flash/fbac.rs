#[doc = "Register `FBAC` reader"]
pub type R = crate::R<FbacSpec>;
#[doc = "Register `FBAC` writer"]
pub type W = crate::W<FbacSpec>;
#[doc = "Field `VREADS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VreadsR = crate::FieldReader;
#[doc = "Field `VREADS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type VreadsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BAGP` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type BagpR = crate::FieldReader;
#[doc = "Field `BAGP` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type BagpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OTPPROTDIS` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type OtpprotdisR = crate::BitReader;
#[doc = "Field `OTPPROTDIS` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type OtpprotdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type Reserved17R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED17` writer - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type Reserved17W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vreads(&self) -> VreadsR {
        VreadsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bagp(&self) -> BagpR {
        BagpR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otpprotdis(&self) -> OtpprotdisR {
        OtpprotdisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vreads(&mut self) -> VreadsW<FbacSpec> {
        VreadsW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bagp(&mut self) -> BagpW<FbacSpec> {
        BagpW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn otpprotdis(&mut self) -> OtpprotdisW<FbacSpec> {
        OtpprotdisW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved17(&mut self) -> Reserved17W<FbacSpec> {
        Reserved17W::new(self, 17)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbacSpec;
impl crate::RegisterSpec for FbacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbac::R`](R) reader structure"]
impl crate::Readable for FbacSpec {}
#[doc = "`write(|w| ..)` method takes [`fbac::W`](W) writer structure"]
impl crate::Writable for FbacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBAC to value 0x0f"]
impl crate::Resettable for FbacSpec {
    const RESET_VALUE: u32 = 0x0f;
}
