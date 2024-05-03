#[doc = "Register `FPAR_OVR` reader"]
pub type R = crate::R<FparOvrSpec>;
#[doc = "Register `FPAR_OVR` writer"]
pub type W = crate::W<FparOvrSpec>;
#[doc = "Field `DAT_INV_PAR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DatInvParR = crate::FieldReader<u32>;
#[doc = "Field `DAT_INV_PAR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type DatInvParW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dat_inv_par(&self) -> DatInvParR {
        DatInvParR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dat_inv_par(&mut self) -> DatInvParW<FparOvrSpec> {
        DatInvParW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpar_ovr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpar_ovr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FparOvrSpec;
impl crate::RegisterSpec for FparOvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpar_ovr::R`](R) reader structure"]
impl crate::Readable for FparOvrSpec {}
#[doc = "`write(|w| ..)` method takes [`fpar_ovr::W`](W) writer structure"]
impl crate::Writable for FparOvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FPAR_OVR to value 0"]
impl crate::Resettable for FparOvrSpec {
    const RESET_VALUE: u32 = 0;
}
