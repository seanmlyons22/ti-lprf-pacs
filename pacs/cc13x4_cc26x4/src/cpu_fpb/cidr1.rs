#[doc = "Register `CIDR1` reader"]
pub type R = crate::R<Cidr1Spec>;
#[doc = "Register `CIDR1` writer"]
pub type W = crate::W<Cidr1Spec>;
#[doc = "Field `PRMBL_1` reader - 3:0\\]
See CoreSight Architecture Specification"]
pub type Prmbl1R = crate::FieldReader;
#[doc = "Field `CLASS` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type ClassR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> Prmbl1R {
        Prmbl1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn class(&self) -> ClassR {
        ClassR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cidr1Spec;
impl crate::RegisterSpec for Cidr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cidr1::R`](R) reader structure"]
impl crate::Readable for Cidr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cidr1::W`](W) writer structure"]
impl crate::Writable for Cidr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIDR1 to value 0"]
impl crate::Resettable for Cidr1Spec {
    const RESET_VALUE: u32 = 0;
}
