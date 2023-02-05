#[doc = "Register `SPECIAL_AUTH_SET` reader"]
pub type R = crate::R<SpecialAuthSetSpec>;
#[doc = "Register `SPECIAL_AUTH_SET` writer"]
pub type W = crate::W<SpecialAuthSetSpec>;
#[doc = "0:0\\]
This bit sets SECAPEN bit in SPECIAL_AUTH register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secapen {
    #[doc = "1: Set SECAPEN"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Secapen> for bool {
    #[inline(always)]
    fn from(variant: Secapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SECAPEN` writer - 0:0\\]
This bit sets SECAPEN bit in SPECIAL_AUTH register."]
pub type SecapenW<'a, REG> = crate::BitWriter<'a, REG, Secapen>;
impl<'a, REG> SecapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set SECAPEN"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::Noeff)
    }
}
#[doc = "1:1\\]
This bit sets FAKESTBYEN in SPECIAL_AUTH register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fakestbyen {
    #[doc = "1: Set FAKESTBYEN"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Fakestbyen> for bool {
    #[inline(always)]
    fn from(variant: Fakestbyen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAKESTBYEN` writer - 1:1\\]
This bit sets FAKESTBYEN in SPECIAL_AUTH register."]
pub type FakestbyenW<'a, REG> = crate::BitWriter<'a, REG, Fakestbyen>;
impl<'a, REG> FakestbyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set FAKESTBYEN"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::Noeff)
    }
}
#[doc = "2:2\\]
This bit sets DFTAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dftapen {
    #[doc = "1: Set DFTTAPEN"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Dftapen> for bool {
    #[inline(always)]
    fn from(variant: Dftapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFTAPEN` writer - 2:2\\]
This bit sets DFTAPEN in SPECIAL_AUTH register."]
pub type DftapenW<'a, REG> = crate::BitWriter<'a, REG, Dftapen>;
impl<'a, REG> DftapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set DFTTAPEN"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dftapen::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Dftapen::Noeff)
    }
}
#[doc = "Field `RESERVED3` writer - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
This bit sets CFGAPEN in SPECIAL_AUTH register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgapen {
    #[doc = "1: Set CFGAPEN"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Cfgapen> for bool {
    #[inline(always)]
    fn from(variant: Cfgapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFGAPEN` writer - 4:4\\]
This bit sets CFGAPEN in SPECIAL_AUTH register."]
pub type CfgapenW<'a, REG> = crate::BitWriter<'a, REG, Cfgapen>;
impl<'a, REG> CfgapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set CFGAPEN"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::Noeff)
    }
}
#[doc = "5:5\\]
This bit sets AHBAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbapen {
    #[doc = "1: SET AHB-AP"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Ahbapen> for bool {
    #[inline(always)]
    fn from(variant: Ahbapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHBAPEN` writer - 5:5\\]
This bit sets AHBAPEN in SPECIAL_AUTH register."]
pub type AhbapenW<'a, REG> = crate::BitWriter<'a, REG, Ahbapen>;
impl<'a, REG> AhbapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SET AHB-AP"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::Noeff)
    }
}
#[doc = "6:6\\]
This bit sets DBGDIS in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdis {
    #[doc = "1: SET DBGDIS"]
    Set = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Dbgdis> for bool {
    #[inline(always)]
    fn from(variant: Dbgdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGDIS` writer - 6:6\\]
This bit sets DBGDIS in SPECIAL_AUTH register."]
pub type DbgdisW<'a, REG> = crate::BitWriter<'a, REG, Dbgdis>;
impl<'a, REG> DbgdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SET DBGDIS"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdis::Set)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdis::Noeff)
    }
}
#[doc = "Field `RESERVED7` reader - 23:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::FieldReader<u32>;
#[doc = "31:24\\]
This field must be configured with 0xA5 in order to access this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "165: This field must be written with 0xA5 to be able to set any of the enable bits"]
    _ToUnlockW_ = 165,
}
impl From<Key> for u8 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u8;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` writer - 31:24\\]
This field must be configured with 0xA5 in order to access this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This field must be written with 0xA5 to be able to set any of the enable bits"]
    #[inline(always)]
    pub fn _to_unlock_w_(self) -> &'a mut crate::W<REG> {
        self.variant(Key::_ToUnlockW_)
    }
}
impl R {
    #[doc = "Bits 7:23 - 23:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new((self.bits >> 7) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit sets SECAPEN bit in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn secapen(&mut self) -> SecapenW<SpecialAuthSetSpec> {
        SecapenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit sets FAKESTBYEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn fakestbyen(&mut self) -> FakestbyenW<SpecialAuthSetSpec> {
        FakestbyenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit sets DFTAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn dftapen(&mut self) -> DftapenW<SpecialAuthSetSpec> {
        DftapenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SpecialAuthSetSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit sets CFGAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn cfgapen(&mut self) -> CfgapenW<SpecialAuthSetSpec> {
        CfgapenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit sets AHBAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn ahbapen(&mut self) -> AhbapenW<SpecialAuthSetSpec> {
        AhbapenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit sets DBGDIS in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdis(&mut self) -> DbgdisW<SpecialAuthSetSpec> {
        DbgdisW::new(self, 6)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This field must be configured with 0xA5 in order to access this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<SpecialAuthSetSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "This register is used for setting bits in SPECIAL_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth_set::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth_set::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpecialAuthSetSpec;
impl crate::RegisterSpec for SpecialAuthSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`special_auth_set::R`](R) reader structure"]
impl crate::Readable for SpecialAuthSetSpec {}
#[doc = "`write(|w| ..)` method takes [`special_auth_set::W`](W) writer structure"]
impl crate::Writable for SpecialAuthSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECIAL_AUTH_SET to value 0x13"]
impl crate::Resettable for SpecialAuthSetSpec {
    const RESET_VALUE: u32 = 0x13;
}
