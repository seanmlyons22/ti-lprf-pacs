#[doc = "Register `RDPRNMN` reader"]
pub type R = crate::R<RdprnmnSpec>;
#[doc = "Register `RDPRNMN` writer"]
pub type W = crate::W<RdprnmnSpec>;
#[doc = "Field `VAL` reader - 5:0\\]
Flash read protection configuration value."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - 5:0\\]
Flash read protection configuration value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Flash read protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<RdprnmnSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<RdprnmnSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Flash non main region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdprnmn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdprnmn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdprnmnSpec;
impl crate::RegisterSpec for RdprnmnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdprnmn::R`](R) reader structure"]
impl crate::Readable for RdprnmnSpec {}
#[doc = "`write(|w| ..)` method takes [`rdprnmn::W`](W) writer structure"]
impl crate::Writable for RdprnmnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RDPRNMN to value 0x3f"]
impl crate::Resettable for RdprnmnSpec {
    const RESET_VALUE: u32 = 0x3f;
}
