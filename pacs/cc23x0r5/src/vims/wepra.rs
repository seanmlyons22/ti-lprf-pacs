#[doc = "Register `WEPRA` reader"]
pub type R = crate::R<WepraSpec>;
#[doc = "Register `WEPRA` writer"]
pub type W = crate::W<WepraSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Flash write/erase protection configuration value."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Flash write/erase protection configuration value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Flash write/erase protection configuration value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Flash write/erase protection configuration value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<WepraSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash main region write/erase protection for first 32 sectors. Nth bit corresponds to the Nth sector. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wepra::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wepra::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WepraSpec;
impl crate::RegisterSpec for WepraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wepra::R`](R) reader structure"]
impl crate::Readable for WepraSpec {}
#[doc = "`write(|w| ..)` method takes [`wepra::W`](W) writer structure"]
impl crate::Writable for WepraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEPRA to value 0xffff_ffff"]
impl crate::Resettable for WepraSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
