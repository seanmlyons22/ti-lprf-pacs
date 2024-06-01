#[doc = "Register `ROM_TEST` reader"]
pub type R = crate::R<RomTestSpec>;
#[doc = "Register `ROM_TEST` writer"]
pub type W = crate::W<RomTestSpec>;
#[doc = "Field `ROM_KEY` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RomKeyR = crate::FieldReader<u32>;
#[doc = "Field `ROM_KEY` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type RomKeyW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rom_key(&self) -> RomKeyR {
        RomKeyR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rom_key(&mut self) -> RomKeyW<RomTestSpec> {
        RomKeyW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RomTestSpec;
impl crate::RegisterSpec for RomTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_test::R`](R) reader structure"]
impl crate::Readable for RomTestSpec {}
#[doc = "`write(|w| ..)` method takes [`rom_test::W`](W) writer structure"]
impl crate::Writable for RomTestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_TEST to value 0"]
impl crate::Resettable for RomTestSpec {
    const RESET_VALUE: u32 = 0;
}
