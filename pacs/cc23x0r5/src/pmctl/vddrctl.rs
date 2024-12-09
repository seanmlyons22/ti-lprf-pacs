#[doc = "Register `VDDRCTL` reader"]
pub type R = crate::R<VddrctlSpec>;
#[doc = "Register `VDDRCTL` writer"]
pub type W = crate::W<VddrctlSpec>;
#[doc = "0:0\\]
Select between GLDO and DCDC as VDDR regulator (in ACTIVE, IDLE and STANDBY mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Select {
    #[doc = "1: DCDC enabled for regulation of VDDR voltage"]
    Dcdc = 1,
    #[doc = "0: GLDO enabled for regulation of VDDR voltage"]
    Gldo = 0,
}
impl From<Select> for bool {
    #[inline(always)]
    fn from(variant: Select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SELECT` reader - 0:0\\]
Select between GLDO and DCDC as VDDR regulator (in ACTIVE, IDLE and STANDBY mode)."]
pub type SelectR = crate::BitReader<Select>;
impl SelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Select {
        match self.bits {
            true => Select::Dcdc,
            false => Select::Gldo,
        }
    }
    #[doc = "DCDC enabled for regulation of VDDR voltage"]
    #[inline(always)]
    pub fn is_dcdc(&self) -> bool {
        *self == Select::Dcdc
    }
    #[doc = "GLDO enabled for regulation of VDDR voltage"]
    #[inline(always)]
    pub fn is_gldo(&self) -> bool {
        *self == Select::Gldo
    }
}
#[doc = "Field `SELECT` writer - 0:0\\]
Select between GLDO and DCDC as VDDR regulator (in ACTIVE, IDLE and STANDBY mode)."]
pub type SelectW<'a, REG> = crate::BitWriter<'a, REG, Select>;
impl<'a, REG> SelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DCDC enabled for regulation of VDDR voltage"]
    #[inline(always)]
    pub fn dcdc(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Dcdc)
    }
    #[doc = "GLDO enabled for regulation of VDDR voltage"]
    #[inline(always)]
    pub fn gldo(self) -> &'a mut crate::W<REG> {
        self.variant(Select::Gldo)
    }
}
#[doc = "1:1\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stby {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    Psuedo = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    Normal = 0,
}
impl From<Stby> for bool {
    #[inline(always)]
    fn from(variant: Stby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STBY` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type StbyR = crate::BitReader<Stby>;
impl StbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stby {
        match self.bits {
            true => Stby::Psuedo,
            false => Stby::Normal,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_psuedo(&self) -> bool {
        *self == Stby::Psuedo
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Stby::Normal
    }
}
#[doc = "Field `STBY` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type StbyW<'a, REG> = crate::BitWriter<'a, REG, Stby>;
impl<'a, REG> StbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psuedo(self) -> &'a mut crate::W<REG> {
        self.variant(Stby::Psuedo)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Stby::Normal)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Select between GLDO and DCDC as VDDR regulator (in ACTIVE, IDLE and STANDBY mode)."]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn stby(&self) -> StbyR {
        StbyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Select between GLDO and DCDC as VDDR regulator (in ACTIVE, IDLE and STANDBY mode)."]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<VddrctlSpec> {
        SelectW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn stby(&mut self) -> StbyW<VddrctlSpec> {
        StbyW::new(self, 1)
    }
}
#[doc = "VDDR Control Register. This register contains VDDR regulator settings for the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddrctlSpec;
impl crate::RegisterSpec for VddrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vddrctl::R`](R) reader structure"]
impl crate::Readable for VddrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`vddrctl::W`](W) writer structure"]
impl crate::Writable for VddrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDDRCTL to value 0"]
impl crate::Resettable for VddrctlSpec {
    const RESET_VALUE: u32 = 0;
}
