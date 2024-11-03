#[doc = "Register `WEPRB` reader"]
pub type R = crate::R<WeprbSpec>;
#[doc = "Register `WEPRB` writer"]
pub type W = crate::W<WeprbSpec>;
#[doc = "Field `VAL` reader - 27:0\\]
Flash write/erase protection configuration value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 27:0\\]
Flash write/erase protection configuration value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:27 - 27:0\\]
Flash write/erase protection configuration value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - 27:0\\]
Flash write/erase protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<WeprbSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<WeprbSpec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "Flash main region write/erase protection for remaining sectors. Each bit corresponds to 8 sectors. Bit 0 corresponds to sector 32-39, bit 1 corresponds to sector 40-47 and so on. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeprbSpec;
impl crate::RegisterSpec for WeprbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weprb::R`](R) reader structure"]
impl crate::Readable for WeprbSpec {}
#[doc = "`write(|w| ..)` method takes [`weprb::W`](W) writer structure"]
impl crate::Writable for WeprbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEPRB to value 0x0fff_ffff"]
impl crate::Resettable for WeprbSpec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
