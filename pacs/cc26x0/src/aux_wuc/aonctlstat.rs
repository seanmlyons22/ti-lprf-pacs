#[doc = "Register `AONCTLSTAT` reader"]
pub type R = crate::R<AonctlstatSpec>;
#[doc = "Register `AONCTLSTAT` writer"]
pub type W = crate::W<AonctlstatSpec>;
#[doc = "Field `SCE_RUN_EN` reader - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub type SceRunEnR = crate::BitReader;
#[doc = "Field `SCE_RUN_EN` writer - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
pub type SceRunEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUX_FORCE_ON` reader - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub type AuxForceOnR = crate::BitReader;
#[doc = "Field `AUX_FORCE_ON` writer - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
pub type AuxForceOnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    pub fn sce_run_en(&self) -> SceRunEnR {
        SceRunEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    pub fn aux_force_on(&self) -> AuxForceOnR {
        AuxForceOnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of AON_WUC:AUX_CTL.SCE_RUN_EN."]
    #[inline(always)]
    #[must_use]
    pub fn sce_run_en(&mut self) -> SceRunEnW<AonctlstatSpec> {
        SceRunEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Status of AON_WUC:AUX_CTL.AUX_FORCE_ON."]
    #[inline(always)]
    #[must_use]
    pub fn aux_force_on(&mut self) -> AuxForceOnW<AonctlstatSpec> {
        AuxForceOnW::new(self, 1)
    }
}
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonctlstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonctlstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonctlstatSpec;
impl crate::RegisterSpec for AonctlstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aonctlstat::R`](R) reader structure"]
impl crate::Readable for AonctlstatSpec {}
#[doc = "`write(|w| ..)` method takes [`aonctlstat::W`](W) writer structure"]
impl crate::Writable for AonctlstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AONCTLSTAT to value 0"]
impl crate::Resettable for AonctlstatSpec {
    const RESET_VALUE: u32 = 0;
}
