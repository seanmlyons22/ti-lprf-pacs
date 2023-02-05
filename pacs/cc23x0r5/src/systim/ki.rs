#[doc = "Register `KI` reader"]
pub type R = crate::R<KiSpec>;
#[doc = "Register `KI` writer"]
pub type W = crate::W<KiSpec>;
#[doc = "Field `VAL` reader - 3:0\\]
Accumulated Error is left shifted by this value."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - 3:0\\]
Accumulated Error is left shifted by this value."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved4R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Accumulated Error is left shifted by this value."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Accumulated Error is left shifted by this value."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<KiSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "PI filter's Accumulator's Gain Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ki::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ki::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KiSpec;
impl crate::RegisterSpec for KiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ki::R`](R) reader structure"]
impl crate::Readable for KiSpec {}
#[doc = "`write(|w| ..)` method takes [`ki::W`](W) writer structure"]
impl crate::Writable for KiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KI to value 0"]
impl crate::Resettable for KiSpec {
    const RESET_VALUE: u32 = 0;
}
