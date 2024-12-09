#[doc = "Register `SWRESET` reader"]
pub type R = crate::R<SwresetSpec>;
#[doc = "Register `SWRESET` writer"]
pub type W = crate::W<SwresetSpec>;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type McuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<SwresetSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mcu(&mut self) -> McuW<SwresetSpec> {
        McuW::new(self, 2)
    }
}
#[doc = "SW Initiated Resets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwresetSpec;
impl crate::RegisterSpec for SwresetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreset::R`](R) reader structure"]
impl crate::Readable for SwresetSpec {}
#[doc = "`write(|w| ..)` method takes [`swreset::W`](W) writer structure"]
impl crate::Writable for SwresetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWRESET to value 0"]
impl crate::Resettable for SwresetSpec {
    const RESET_VALUE: u32 = 0;
}
