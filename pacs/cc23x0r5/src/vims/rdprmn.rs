#[doc = "Register `RDPRMN` reader"]
pub type R = crate::R<RdprmnSpec>;
#[doc = "Register `RDPRMN` writer"]
pub type W = crate::W<RdprmnSpec>;
#[doc = "Field `VAL` reader - 2:0\\]
Flash read protection configuration value."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - 2:0\\]
Flash read protection configuration value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<RdprmnSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash main region read protection register upto first 8KB. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdprmn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdprmn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdprmnSpec;
impl crate::RegisterSpec for RdprmnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdprmn::R`](R) reader structure"]
impl crate::Readable for RdprmnSpec {}
#[doc = "`write(|w| ..)` method takes [`rdprmn::W`](W) writer structure"]
impl crate::Writable for RdprmnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDPRMN to value 0x07"]
impl crate::Resettable for RdprmnSpec {
    const RESET_VALUE: u32 = 0x07;
}
