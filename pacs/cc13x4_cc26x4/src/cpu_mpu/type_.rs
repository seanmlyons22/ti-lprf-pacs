#[doc = "Register `TYPE` reader"]
pub type R = crate::R<TypeSpec>;
#[doc = "Register `TYPE` writer"]
pub type W = crate::W<TypeSpec>;
#[doc = "Field `SEPARATE` reader - 0:0\\]
Indicates support for separate instructions and data address regions"]
pub type SeparateR = crate::BitReader;
#[doc = "Field `SEPARATE` writer - 0:0\\]
Indicates support for separate instructions and data address regions"]
pub type SeparateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RESERVED1` writer - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DREGION` reader - 15:8\\]
Number of regions supported by the MPU"]
pub type DregionR = crate::FieldReader;
#[doc = "Field `DREGION` writer - 15:8\\]
Number of regions supported by the MPU"]
pub type DregionW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates support for separate instructions and data address regions"]
    #[inline(always)]
    pub fn separate(&self) -> SeparateR {
        SeparateR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of regions supported by the MPU"]
    #[inline(always)]
    pub fn dregion(&self) -> DregionR {
        DregionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates support for separate instructions and data address regions"]
    #[inline(always)]
    #[must_use]
    pub fn separate(&mut self) -> SeparateW<TypeSpec> {
        SeparateW::new(self, 0)
    }
    #[doc = "Bits 1:7 - 7:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<TypeSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Number of regions supported by the MPU"]
    #[inline(always)]
    #[must_use]
    pub fn dregion(&mut self) -> DregionW<TypeSpec> {
        DregionW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<TypeSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "The MPU Type Register indicates how many regions the MPU supports\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`type_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`type_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets TYPE to value 0x0800"]
impl crate::Resettable for TypeSpec {
    const RESET_VALUE: u32 = 0x0800;
}
