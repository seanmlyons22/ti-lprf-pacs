#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Register `TYPE` writer"]
pub type W = crate::W<TypeSpec>;
#[doc = "Field `SREGION` reader - 7:0\\]
The number of implemented SAU regions"]
pub type SregionR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
The number of implemented SAU regions"]
    #[inline(always)]
    pub fn sregion(&self) -> SregionR {
        SregionR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Indicates the number of regions implemented by the Security Attribution Unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TypeSpec;
impl crate::RegisterSpec for TypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`type_::R`](R) reader structure"]
impl crate::Readable for TypeSpec {}
#[doc = "`write(|w| ..)` method takes [`type_::W`](W) writer structure"]
impl crate::Writable for TypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TYPE to value 0"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0;
}
