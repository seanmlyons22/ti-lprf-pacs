#[doc = "Register `DEBUG3` reader"]
pub type R = crate::R<Debug3Spec>;
#[doc = "Register `DEBUG3` writer"]
pub type W = crate::W<Debug3Spec>;
#[doc = "Field `BOOST_ENZ` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BoostEnzR = crate::BitReader;
#[doc = "Field `BOOST_ENZ` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BoostEnzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 3:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `DEC0_DIS` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type Dec0DisR = crate::BitReader;
#[doc = "Field `DEC0_DIS` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type Dec0DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEC1_DIS` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type Dec1DisR = crate::BitReader;
#[doc = "Field `DEC1_DIS` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type Dec1DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn boost_enz(&self) -> BoostEnzR {
        BoostEnzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dec0_dis(&self) -> Dec0DisR {
        Dec0DisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dec1_dis(&self) -> Dec1DisR {
        Dec1DisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn boost_enz(&mut self) -> BoostEnzW<Debug3Spec> {
        BoostEnzW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dec0_dis(&mut self) -> Dec0DisW<Debug3Spec> {
        Dec0DisW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dec1_dis(&mut self) -> Dec1DisW<Debug3Spec> {
        Dec1DisW::new(self, 5)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug3Spec;
impl crate::RegisterSpec for Debug3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug3::R`](R) reader structure"]
impl crate::Readable for Debug3Spec {}
#[doc = "`write(|w| ..)` method takes [`debug3::W`](W) writer structure"]
impl crate::Writable for Debug3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG3 to value 0"]
impl crate::Resettable for Debug3Spec {
    const RESET_VALUE: u32 = 0;
}
