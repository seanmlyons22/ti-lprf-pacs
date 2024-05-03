#[doc = "Register `FEMU_ADDR` reader"]
pub type R = crate::R<FemuAddrSpec>;
#[doc = "Register `FEMU_ADDR` writer"]
pub type W = crate::W<FemuAddrSpec>;
#[doc = "Field `EMU_ADDR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type EmuAddrR = crate::FieldReader<u32>;
#[doc = "Field `EMU_ADDR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type EmuAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn emu_addr(&self) -> EmuAddrR {
        EmuAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn emu_addr(&mut self) -> EmuAddrW<FemuAddrSpec> {
        EmuAddrW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`femu_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`femu_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FemuAddrSpec;
impl crate::RegisterSpec for FemuAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`femu_addr::R`](R) reader structure"]
impl crate::Readable for FemuAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`femu_addr::W`](W) writer structure"]
impl crate::Writable for FemuAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEMU_ADDR to value 0"]
impl crate::Resettable for FemuAddrSpec {
    const RESET_VALUE: u32 = 0;
}
