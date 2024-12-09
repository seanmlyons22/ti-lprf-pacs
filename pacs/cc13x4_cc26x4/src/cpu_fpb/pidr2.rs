#[doc = "Register `PIDR2` reader"]
pub type R = crate::R<Pidr2Spec>;
#[doc = "Register `PIDR2` writer"]
pub type W = crate::W<Pidr2Spec>;
#[doc = "Field `DES_1` reader - 2:0\\]
See CoreSight Architecture Specification"]
pub type Des1R = crate::FieldReader;
#[doc = "Field `JEDEC` reader - 3:3\\]
See CoreSight Architecture Specification"]
pub type JedecR = crate::BitReader;
#[doc = "Field `REVISION` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_1(&self) -> Des1R {
        Des1R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn jedec(&self) -> JedecR {
        JedecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pidr2Spec;
impl crate::RegisterSpec for Pidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pidr2::R`](R) reader structure"]
impl crate::Readable for Pidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`pidr2::W`](W) writer structure"]
impl crate::Writable for Pidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIDR2 to value 0"]
impl crate::Resettable for Pidr2Spec {
    const RESET_VALUE: u32 = 0;
}
