#[doc = "Register `SPECIAL_AUTH` reader"]
pub type R = crate::R<SpecialAuthSpec>;
#[doc = "Register `SPECIAL_AUTH` writer"]
pub type W = crate::W<SpecialAuthSpec>;
#[doc = "0:0\\]
Indicates status of SECAP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secapen {
    #[doc = "1: Enable SEC-AP"]
    En = 1,
    #[doc = "0: Disable SEC-AP"]
    Dis = 0,
}
impl From<Secapen> for bool {
    #[inline(always)]
    fn from(variant: Secapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECAPEN` reader - 0:0\\]
Indicates status of SECAP"]
pub type SecapenR = crate::BitReader<Secapen>;
impl SecapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Secapen {
        match self.bits {
            true => Secapen::En,
            false => Secapen::Dis,
        }
    }
    #[doc = "Enable SEC-AP"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Secapen::En
    }
    #[doc = "Disable SEC-AP"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Secapen::Dis
    }
}
#[doc = "Field `SECAPEN` writer - 0:0\\]
Indicates status of SECAP"]
pub type SecapenW<'a, REG> = crate::BitWriter<'a, REG, Secapen>;
impl<'a, REG> SecapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable SEC-AP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::En)
    }
    #[doc = "Disable SEC-AP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::Dis)
    }
}
#[doc = "1:1\\]
Indicates status of FAKESTBYEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fakestbyen {
    #[doc = "1: FAKESTBYEN is Enabled"]
    En = 1,
    #[doc = "0: FAKESTBYEN is Disabled"]
    Dis = 0,
}
impl From<Fakestbyen> for bool {
    #[inline(always)]
    fn from(variant: Fakestbyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAKESTBYEN` reader - 1:1\\]
Indicates status of FAKESTBYEN"]
pub type FakestbyenR = crate::BitReader<Fakestbyen>;
impl FakestbyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fakestbyen {
        match self.bits {
            true => Fakestbyen::En,
            false => Fakestbyen::Dis,
        }
    }
    #[doc = "FAKESTBYEN is Enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Fakestbyen::En
    }
    #[doc = "FAKESTBYEN is Disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Fakestbyen::Dis
    }
}
#[doc = "Field `FAKESTBYEN` writer - 1:1\\]
Indicates status of FAKESTBYEN"]
pub type FakestbyenW<'a, REG> = crate::BitWriter<'a, REG, Fakestbyen>;
impl<'a, REG> FakestbyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FAKESTBYEN is Enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::En)
    }
    #[doc = "FAKESTBYEN is Disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::Dis)
    }
}
#[doc = "2:2\\]
Indicates status of DFTAPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dftapen {
    #[doc = "1: Enable DFT-TAP"]
    En = 1,
    #[doc = "0: Disable DFT-TAP"]
    Dis = 0,
}
impl From<Dftapen> for bool {
    #[inline(always)]
    fn from(variant: Dftapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFTAPEN` reader - 2:2\\]
Indicates status of DFTAPEN"]
pub type DftapenR = crate::BitReader<Dftapen>;
impl DftapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dftapen {
        match self.bits {
            true => Dftapen::En,
            false => Dftapen::Dis,
        }
    }
    #[doc = "Enable DFT-TAP"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dftapen::En
    }
    #[doc = "Disable DFT-TAP"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dftapen::Dis
    }
}
#[doc = "Field `DFTAPEN` writer - 2:2\\]
Indicates status of DFTAPEN"]
pub type DftapenW<'a, REG> = crate::BitWriter<'a, REG, Dftapen>;
impl<'a, REG> DftapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable DFT-TAP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dftapen::En)
    }
    #[doc = "Disable DFT-TAP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dftapen::Dis)
    }
}
#[doc = "Field `RESERVED3` reader - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
Indicates status of CFGAPEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgapen {
    #[doc = "1: Enable CFG-AP"]
    En = 1,
    #[doc = "0: Disable CFG-AP"]
    Dis = 0,
}
impl From<Cfgapen> for bool {
    #[inline(always)]
    fn from(variant: Cfgapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGAPEN` reader - 4:4\\]
Indicates status of CFGAPEN"]
pub type CfgapenR = crate::BitReader<Cfgapen>;
impl CfgapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgapen {
        match self.bits {
            true => Cfgapen::En,
            false => Cfgapen::Dis,
        }
    }
    #[doc = "Enable CFG-AP"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Cfgapen::En
    }
    #[doc = "Disable CFG-AP"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Cfgapen::Dis
    }
}
#[doc = "Field `CFGAPEN` writer - 4:4\\]
Indicates status of CFGAPEN"]
pub type CfgapenW<'a, REG> = crate::BitWriter<'a, REG, Cfgapen>;
impl<'a, REG> CfgapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable CFG-AP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::En)
    }
    #[doc = "Disable CFG-AP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::Dis)
    }
}
#[doc = "5:5\\]
Indicates status of AHBAPEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbapen {
    #[doc = "1: Enable AHB-AP"]
    En = 1,
    #[doc = "0: Disable AHB-AP"]
    Dis = 0,
}
impl From<Ahbapen> for bool {
    #[inline(always)]
    fn from(variant: Ahbapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBAPEN` reader - 5:5\\]
Indicates status of AHBAPEN"]
pub type AhbapenR = crate::BitReader<Ahbapen>;
impl AhbapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ahbapen {
        match self.bits {
            true => Ahbapen::En,
            false => Ahbapen::Dis,
        }
    }
    #[doc = "Enable AHB-AP"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ahbapen::En
    }
    #[doc = "Disable AHB-AP"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ahbapen::Dis
    }
}
#[doc = "Field `AHBAPEN` writer - 5:5\\]
Indicates status of AHBAPEN"]
pub type AhbapenW<'a, REG> = crate::BitWriter<'a, REG, Ahbapen>;
impl<'a, REG> AhbapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable AHB-AP"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::En)
    }
    #[doc = "Disable AHB-AP"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::Dis)
    }
}
#[doc = "6:6\\]
Indicates status of DBGDIS.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdis {
    #[doc = "1: Disables debugging capability"]
    Dis = 1,
    #[doc = "0: Enables debugging capability."]
    En = 0,
}
impl From<Dbgdis> for bool {
    #[inline(always)]
    fn from(variant: Dbgdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDIS` reader - 6:6\\]
Indicates status of DBGDIS."]
pub type DbgdisR = crate::BitReader<Dbgdis>;
impl DbgdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbgdis {
        match self.bits {
            true => Dbgdis::Dis,
            false => Dbgdis::En,
        }
    }
    #[doc = "Disables debugging capability"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Dbgdis::Dis
    }
    #[doc = "Enables debugging capability."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dbgdis::En
    }
}
#[doc = "Field `DBGDIS` writer - 6:6\\]
Indicates status of DBGDIS."]
pub type DbgdisW<'a, REG> = crate::BitWriter<'a, REG, Dbgdis>;
impl<'a, REG> DbgdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables debugging capability"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdis::Dis)
    }
    #[doc = "Enables debugging capability."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdis::En)
    }
}
#[doc = "Field `RESERVED7` reader - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED7` writer - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates status of SECAP"]
    #[inline(always)]
    pub fn secapen(&self) -> SecapenR {
        SecapenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates status of FAKESTBYEN"]
    #[inline(always)]
    pub fn fakestbyen(&self) -> FakestbyenR {
        FakestbyenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates status of DFTAPEN"]
    #[inline(always)]
    pub fn dftapen(&self) -> DftapenR {
        DftapenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates status of CFGAPEN"]
    #[inline(always)]
    pub fn cfgapen(&self) -> CfgapenR {
        CfgapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates status of AHBAPEN"]
    #[inline(always)]
    pub fn ahbapen(&self) -> AhbapenR {
        AhbapenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates status of DBGDIS."]
    #[inline(always)]
    pub fn dbgdis(&self) -> DbgdisR {
        DbgdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates status of SECAP"]
    #[inline(always)]
    #[must_use]
    pub fn secapen(&mut self) -> SecapenW<SpecialAuthSpec> {
        SecapenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Indicates status of FAKESTBYEN"]
    #[inline(always)]
    #[must_use]
    pub fn fakestbyen(&mut self) -> FakestbyenW<SpecialAuthSpec> {
        FakestbyenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Indicates status of DFTAPEN"]
    #[inline(always)]
    #[must_use]
    pub fn dftapen(&mut self) -> DftapenW<SpecialAuthSpec> {
        DftapenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SpecialAuthSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates status of CFGAPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cfgapen(&mut self) -> CfgapenW<SpecialAuthSpec> {
        CfgapenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Indicates status of AHBAPEN"]
    #[inline(always)]
    #[must_use]
    pub fn ahbapen(&mut self) -> AhbapenW<SpecialAuthSpec> {
        AhbapenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Indicates status of DBGDIS."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdis(&mut self) -> DbgdisW<SpecialAuthSpec> {
        DbgdisW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<SpecialAuthSpec> {
        Reserved7W::new(self, 7)
    }
}
#[doc = "This register indicates the status of different AP firewalls and fakestandby enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpecialAuthSpec;
impl crate::RegisterSpec for SpecialAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`special_auth::R`](R) reader structure"]
impl crate::Readable for SpecialAuthSpec {}
#[doc = "`write(|w| ..)` method takes [`special_auth::W`](W) writer structure"]
impl crate::Writable for SpecialAuthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECIAL_AUTH to value 0x13"]
impl crate::Resettable for SpecialAuthSpec {
    const RESET_VALUE: u32 = 0x13;
}
