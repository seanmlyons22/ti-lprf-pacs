#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision of the IP"]
pub type MinrevR = crate::FieldReader;
#[doc = "Field `MINREV` writer - 3:0\\]
Minor revision of the IP"]
pub type MinrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision of the IP"]
pub type MajrevR = crate::FieldReader;
#[doc = "Field `MAJREV` writer - 7:4\\]
Major revision of the IP"]
pub type MajrevW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED8` reader - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FEATUREVER` reader - 15:12\\]
Feature set version for this module instance."]
pub type FeatureverR = crate::FieldReader;
#[doc = "Field `FEATUREVER` writer - 15:12\\]
Feature set version for this module instance."]
pub type FeatureverW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MODULEID` reader - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type ModuleidR = crate::FieldReader<u16>;
#[doc = "Field `MODULEID` writer - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type ModuleidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of the IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of the IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set version for this module instance."]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn minrev(&mut self) -> MinrevW<DescSpec> {
        MinrevW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision of the IP"]
    #[inline(always)]
    #[must_use]
    pub fn majrev(&mut self) -> MajrevW<DescSpec> {
        MajrevW::new(self, 4)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<DescSpec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set version for this module instance."]
    #[inline(always)]
    #[must_use]
    pub fn featurever(&mut self) -> FeatureverW<DescSpec> {
        FeatureverW::new(self, 12)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    #[must_use]
    pub fn moduleid(&mut self) -> ModuleidW<DescSpec> {
        ModuleidW::new(self, 16)
    }
}
#[doc = "This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`write(|w| ..)` method takes [`desc::W`](W) writer structure"]
impl crate::Writable for DescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESC to value 0x1411_0010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x1411_0010;
}
