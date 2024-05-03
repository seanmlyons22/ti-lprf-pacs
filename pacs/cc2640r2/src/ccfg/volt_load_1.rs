#[doc = "Register `VOLT_LOAD_1` reader"]
pub type R = crate::R<VoltLoad1Spec>;
#[doc = "Register `VOLT_LOAD_1` writer"]
pub type W = crate::W<VoltLoad1Spec>;
#[doc = "Field `VDDR_EXT_TP65` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp65R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP65` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp65W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP85` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp85R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP85` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp85W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP105` reader - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp105R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP105` writer - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp105W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP125` reader - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp125R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP125` writer - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp125W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp65(&self) -> VddrExtTp65R {
        VddrExtTp65R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp85(&self) -> VddrExtTp85R {
        VddrExtTp85R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp105(&self) -> VddrExtTp105R {
        VddrExtTp105R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp125(&self) -> VddrExtTp125R {
        VddrExtTp125R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp65(&mut self) -> VddrExtTp65W<VoltLoad1Spec> {
        VddrExtTp65W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp85(&mut self) -> VddrExtTp85W<VoltLoad1Spec> {
        VddrExtTp85W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp105(&mut self) -> VddrExtTp105W<VoltLoad1Spec> {
        VddrExtTp105W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp125(&mut self) -> VddrExtTp125W<VoltLoad1Spec> {
        VddrExtTp125W::new(self, 24)
    }
}
#[doc = "Voltage Load 1 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_load_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_load_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VoltLoad1Spec;
impl crate::RegisterSpec for VoltLoad1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`volt_load_1::R`](R) reader structure"]
impl crate::Readable for VoltLoad1Spec {}
#[doc = "`write(|w| ..)` method takes [`volt_load_1::W`](W) writer structure"]
impl crate::Writable for VoltLoad1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VOLT_LOAD_1 to value 0xffff_ffff"]
impl crate::Resettable for VoltLoad1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
