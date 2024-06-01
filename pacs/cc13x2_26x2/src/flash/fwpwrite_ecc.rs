#[doc = "Register `FWPWRITE_ECC` reader"]
pub type R = crate::R<FwpwriteEccSpec>;
#[doc = "Register `FWPWRITE_ECC` writer"]
pub type W = crate::W<FwpwriteEccSpec>;
#[doc = "Field `ECCBYTES31_24` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes31_24R = crate::FieldReader;
#[doc = "Field `ECCBYTES31_24` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ECCBYTES23_16` reader - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes23_16R = crate::FieldReader;
#[doc = "Field `ECCBYTES23_16` writer - 15:8\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes23_16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ECCBYTES15_08` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes15_08R = crate::FieldReader;
#[doc = "Field `ECCBYTES15_08` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes15_08W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ECCBYTES07_00` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes07_00R = crate::FieldReader;
#[doc = "Field `ECCBYTES07_00` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Eccbytes07_00W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes31_24(&self) -> Eccbytes31_24R {
        Eccbytes31_24R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes23_16(&self) -> Eccbytes23_16R {
        Eccbytes23_16R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes15_08(&self) -> Eccbytes15_08R {
        Eccbytes15_08R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn eccbytes07_00(&self) -> Eccbytes07_00R {
        Eccbytes07_00R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes31_24(&mut self) -> Eccbytes31_24W<FwpwriteEccSpec> {
        Eccbytes31_24W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes23_16(&mut self) -> Eccbytes23_16W<FwpwriteEccSpec> {
        Eccbytes23_16W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes15_08(&mut self) -> Eccbytes15_08W<FwpwriteEccSpec> {
        Eccbytes15_08W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn eccbytes07_00(&mut self) -> Eccbytes07_00W<FwpwriteEccSpec> {
        Eccbytes07_00W::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fwpwrite_ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fwpwrite_ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwpwriteEccSpec;
impl crate::RegisterSpec for FwpwriteEccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwpwrite_ecc::R`](R) reader structure"]
impl crate::Readable for FwpwriteEccSpec {}
#[doc = "`write(|w| ..)` method takes [`fwpwrite_ecc::W`](W) writer structure"]
impl crate::Writable for FwpwriteEccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWPWRITE_ECC to value 0xffff_ffff"]
impl crate::Resettable for FwpwriteEccSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
