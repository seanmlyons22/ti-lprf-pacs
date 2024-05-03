#[doc = "Register `VOLT_LOAD_0` reader"]
pub type R = crate::R<VoltLoad0Spec>;
#[doc = "Register `VOLT_LOAD_0` writer"]
pub type W = crate::W<VoltLoad0Spec>;
#[doc = "Field `VDDR_EXT_TM15` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTm15R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TM15` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTm15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP5` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp5R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP5` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP25` reader - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp25R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP25` writer - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `VDDR_EXT_TP45` reader - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp45R = crate::FieldReader;
#[doc = "Field `VDDR_EXT_TP45` writer - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type VddrExtTp45W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tm15(&self) -> VddrExtTm15R {
        VddrExtTm15R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp5(&self) -> VddrExtTp5R {
        VddrExtTp5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp25(&self) -> VddrExtTp25R {
        VddrExtTp25R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn vddr_ext_tp45(&self) -> VddrExtTp45R {
        VddrExtTp45R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tm15(&mut self) -> VddrExtTm15W<VoltLoad0Spec> {
        VddrExtTm15W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp5(&mut self) -> VddrExtTp5W<VoltLoad0Spec> {
        VddrExtTp5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp25(&mut self) -> VddrExtTp25W<VoltLoad0Spec> {
        VddrExtTp25W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn vddr_ext_tp45(&mut self) -> VddrExtTp45W<VoltLoad0Spec> {
        VddrExtTp45W::new(self, 24)
    }
}
#[doc = "Voltage Load 0 Enabled by MODE_CONF.VDDR_EXT_LOAD.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`volt_load_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`volt_load_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VoltLoad0Spec;
impl crate::RegisterSpec for VoltLoad0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`volt_load_0::R`](R) reader structure"]
impl crate::Readable for VoltLoad0Spec {}
#[doc = "`write(|w| ..)` method takes [`volt_load_0::W`](W) writer structure"]
impl crate::Writable for VoltLoad0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VOLT_LOAD_0 to value 0xffff_ffff"]
impl crate::Resettable for VoltLoad0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
