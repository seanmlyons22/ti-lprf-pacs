#[doc = "Register `FEMU_ECC` reader"]
pub type R = crate::R<FemuEccSpec>;
#[doc = "Register `FEMU_ECC` writer"]
pub type W = crate::W<FemuEccSpec>;
#[doc = "Field `EMU_ECC` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type EmuEccR = crate::FieldReader<u32>;
#[doc = "Field `EMU_ECC` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type EmuEccW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_ecc(&self) -> EmuEccR {
        EmuEccR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn emu_ecc(&mut self) -> EmuEccW<FemuEccSpec> {
        EmuEccW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FemuEccSpec;
impl crate::RegisterSpec for FemuEccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`femu_ecc::R`](R) reader structure"]
impl crate::Readable for FemuEccSpec {}
#[doc = "`write(|w| ..)` method takes [`femu_ecc::W`](W) writer structure"]
impl crate::Writable for FemuEccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEMU_ECC to value 0"]
impl crate::Resettable for FemuEccSpec {
    const RESET_VALUE: u32 = 0;
}
