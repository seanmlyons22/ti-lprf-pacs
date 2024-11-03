#[doc = "Register `RDPREGR` reader"]
pub type R = crate::R<RdpregrSpec>;
#[doc = "Register `RDPREGR` writer"]
pub type W = crate::W<RdpregrSpec>;
#[doc = "Field `VAL` reader - 0:0\\]
Flash read protection configuration value."]
pub type ValR = crate::BitReader;
#[doc = "Field `VAL` writer - 0:0\\]
Flash read protection configuration value."]
pub type ValW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<RdpregrSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<RdpregrSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Flash engr region read protection register. This register is sticky when written with value 0. This register is retained\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdpregr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdpregr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdpregrSpec;
impl crate::RegisterSpec for RdpregrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdpregr::R`](R) reader structure"]
impl crate::Readable for RdpregrSpec {}
#[doc = "`write(|w| ..)` method takes [`rdpregr::W`](W) writer structure"]
impl crate::Writable for RdpregrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDPREGR to value 0x01"]
impl crate::Resettable for RdpregrSpec {
    const RESET_VALUE: u32 = 0x01;
}
