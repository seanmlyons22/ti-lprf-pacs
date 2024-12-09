#[doc = "Register `STIR` reader"]
pub type R = crate::R<StirSpec>;
#[doc = "Register `STIR` writer"]
pub type W = crate::W<StirSpec>;
#[doc = "Field `INTID` writer - 8:0\\]
Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
pub type IntidW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
Indicates the interrupt to be pended. The value written is (ExceptionNumber - 16)"]
    #[inline(always)]
    #[must_use]
    pub fn intid(&mut self) -> IntidW<StirSpec> {
        IntidW::new(self, 0)
    }
}
#[doc = "Provides a mechanism for software to generate an interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StirSpec;
impl crate::RegisterSpec for StirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stir::R`](R) reader structure"]
impl crate::Readable for StirSpec {}
#[doc = "`write(|w| ..)` method takes [`stir::W`](W) writer structure"]
impl crate::Writable for StirSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STIR to value 0"]
impl crate::Resettable for StirSpec {
    const RESET_VALUE: u32 = 0;
}
