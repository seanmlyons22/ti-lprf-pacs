#[doc = "Register `RATCH4VAL` reader"]
pub type R = crate::R<Ratch4valSpec>;
#[doc = "Register `RATCH4VAL` writer"]
pub type W = crate::W<Ratch4valSpec>;
#[doc = "Field `VAL` reader - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Capture/compare value. The system CPU can safely read this register, but it is recommended to use the CPE API commands to configure it for compare mode."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<Ratch4valSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Timer Channel 4 Capture/Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ratch4val::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ratch4val::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ratch4valSpec;
impl crate::RegisterSpec for Ratch4valSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ratch4val::R`](R) reader structure"]
impl crate::Readable for Ratch4valSpec {}
#[doc = "`write(|w| ..)` method takes [`ratch4val::W`](W) writer structure"]
impl crate::Writable for Ratch4valSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RATCH4VAL to value 0"]
impl crate::Resettable for Ratch4valSpec {
    const RESET_VALUE: u32 = 0;
}
