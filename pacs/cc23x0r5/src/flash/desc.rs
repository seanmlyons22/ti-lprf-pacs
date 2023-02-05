#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "3:0\\]
Minor Revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Minrev {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Minrev> for u8 {
    #[inline(always)]
    fn from(variant: Minrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Minrev {
    type Ux = u8;
}
impl crate::IsEnum for Minrev {}
#[doc = "Field `MINREV` reader - 3:0\\]
Minor Revision"]
pub type MinrevR = crate::FieldReader<Minrev>;
impl MinrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Minrev> {
        match self.bits {
            15 => Some(Minrev::Maximum),
            0 => Some(Minrev::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Minrev::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Minrev::Minimum
    }
}
#[doc = "7:4\\]
Major Revision\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Majrev {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Majrev> for u8 {
    #[inline(always)]
    fn from(variant: Majrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Majrev {
    type Ux = u8;
}
impl crate::IsEnum for Majrev {}
#[doc = "Field `MAJREV` reader - 7:4\\]
Major Revision"]
pub type MajrevR = crate::FieldReader<Majrev>;
impl MajrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Majrev> {
        match self.bits {
            15 => Some(Majrev::Maximum),
            0 => Some(Majrev::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Majrev::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Majrev::Minimum
    }
}
#[doc = "11:8\\]
Instance number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instnum {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Instnum> for u8 {
    #[inline(always)]
    fn from(variant: Instnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Instnum {
    type Ux = u8;
}
impl crate::IsEnum for Instnum {}
#[doc = "Field `INSTNUM` reader - 11:8\\]
Instance number"]
pub type InstnumR = crate::FieldReader<Instnum>;
impl InstnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Instnum> {
        match self.bits {
            15 => Some(Instnum::Maximum),
            0 => Some(Instnum::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Instnum::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Instnum::Minimum
    }
}
#[doc = "15:12\\]
Feature set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Featurever {
    #[doc = "15: Maximum Value"]
    Maximum = 15,
    #[doc = "0: Minimum Value"]
    Minimum = 0,
}
impl From<Featurever> for u8 {
    #[inline(always)]
    fn from(variant: Featurever) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Featurever {
    type Ux = u8;
}
impl crate::IsEnum for Featurever {}
#[doc = "Field `FEATUREVER` reader - 15:12\\]
Feature set"]
pub type FeatureverR = crate::FieldReader<Featurever>;
impl FeatureverR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Featurever> {
        match self.bits {
            15 => Some(Featurever::Maximum),
            0 => Some(Featurever::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum Value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Featurever::Maximum
    }
    #[doc = "Minimum Value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Featurever::Minimum
    }
}
#[doc = "31:16\\]
Module ID\n\nValue on reset: 2880"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Moduleid {
    #[doc = "65535: Highest possible value"]
    Maximum = 65535,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Moduleid> for u16 {
    #[inline(always)]
    fn from(variant: Moduleid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Moduleid {
    type Ux = u16;
}
impl crate::IsEnum for Moduleid {}
#[doc = "Field `MODULEID` reader - 31:16\\]
Module ID"]
pub type ModuleidR = crate::FieldReader<Moduleid>;
impl ModuleidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Moduleid> {
        match self.bits {
            65535 => Some(Moduleid::Maximum),
            0 => Some(Moduleid::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Moduleid::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Moduleid::Minimum
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor Revision"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major Revision"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Instance number"]
    #[inline(always)]
    pub fn instnum(&self) -> InstnumR {
        InstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Feature set"]
    #[inline(always)]
    pub fn featurever(&self) -> FeatureverR {
        FeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module ID"]
    #[inline(always)]
    pub fn moduleid(&self) -> ModuleidR {
        ModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Hardware Version Description Register: This register identifies the flash wrapper hardware version and feature set used.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets DESC to value 0x0b40_1010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x0b40_1010;
}
