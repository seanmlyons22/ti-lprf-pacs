#[doc = "Register `FEDACCTL1` reader"]
pub type R = crate::R<Fedacctl1Spec>;
#[doc = "Register `FEDACCTL1` writer"]
pub type W = crate::W<Fedacctl1Spec>;
#[doc = "Field `EDACEN` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type EdacenR = crate::FieldReader<u32>;
#[doc = "Field `EDACEN` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type EdacenW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SUSP_IGNR` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type SuspIgnrR = crate::BitReader;
#[doc = "Field `SUSP_IGNR` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type SuspIgnrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn edacen(&self) -> EdacenR {
        EdacenR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn susp_ignr(&self) -> SuspIgnrR {
        SuspIgnrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn edacen(&mut self) -> EdacenW<Fedacctl1Spec> {
        EdacenW::new(self, 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn susp_ignr(&mut self) -> SuspIgnrW<Fedacctl1Spec> {
        SuspIgnrW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Fedacctl1Spec> {
        Reserved25W::new(self, 25)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fedacctl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fedacctl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fedacctl1Spec;
impl crate::RegisterSpec for Fedacctl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fedacctl1::R`](R) reader structure"]
impl crate::Readable for Fedacctl1Spec {}
#[doc = "`write(|w| ..)` method takes [`fedacctl1::W`](W) writer structure"]
impl crate::Writable for Fedacctl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEDACCTL1 to value 0"]
impl crate::Resettable for Fedacctl1Spec {
    const RESET_VALUE: u32 = 0;
}
