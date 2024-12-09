#[doc = "Register `SPECIAL_AUTH_CLR` reader"]
pub type R = crate::R<SpecialAuthClrSpec>;
#[doc = "Register `SPECIAL_AUTH_CLR` writer"]
pub type W = crate::W<SpecialAuthClrSpec>;
#[doc = "0:0\\]
This bit clears SECAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Secapen {
    #[doc = "1: Clear SECAPEN"]
    Clr = 1,
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
This bit clears SECAPEN in SPECIAL_AUTH register."]
pub type SecapenW<'a, REG> = crate::BitWriter<'a, REG, Secapen>;
impl<'a, REG> SecapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear SECAPEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Secapen::Noeff)
    }
}
#[doc = "1:1\\]
This bit clears FAKESTBYEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fakestbyen {
    #[doc = "1: Clears FAKESTBYEN"]
    Clr = 1,
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
This bit clears FAKESTBYEN in SPECIAL_AUTH register."]
pub type FakestbyenW<'a, REG> = crate::BitWriter<'a, REG, Fakestbyen>;
impl<'a, REG> FakestbyenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears FAKESTBYEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Fakestbyen::Noeff)
    }
}
#[doc = "2:2\\]
This bit clears DFTTAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dftapen {
    #[doc = "1: Clear DFTTAPEN"]
    Clr = 1,
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
This bit clears DFTTAPEN in SPECIAL_AUTH register."]
pub type DftapenW<'a, REG> = crate::BitWriter<'a, REG, Dftapen>;
impl<'a, REG> DftapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear DFTTAPEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dftapen::Clr)
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
This bit clears CFGAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgapen {
    #[doc = "1: Clear CFGAPEN"]
    Clr = 1,
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
This bit clears CFGAPEN in SPECIAL_AUTH register."]
pub type CfgapenW<'a, REG> = crate::BitWriter<'a, REG, Cfgapen>;
impl<'a, REG> CfgapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear CFGAPEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgapen::Noeff)
    }
}
#[doc = "5:5\\]
This bit clears AHBAPEN in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ahbapen {
    #[doc = "1: Clear AHBAPEN"]
    Clr = 1,
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
This bit clears AHBAPEN in SPECIAL_AUTH register."]
pub type AhbapenW<'a, REG> = crate::BitWriter<'a, REG, Ahbapen>;
impl<'a, REG> AhbapenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear AHBAPEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Ahbapen::Noeff)
    }
}
#[doc = "6:6\\]
This bit clears DBGDIS in SPECIAL_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgdis {
    #[doc = "1: Clear DBGDIS"]
    Clr = 1,
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
This bit clears DBGDIS in SPECIAL_AUTH register."]
pub type DbgdisW<'a, REG> = crate::BitWriter<'a, REG, Dbgdis>;
impl<'a, REG> DbgdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear DBGDIS"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgdis::Clr)
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
This field must be configured with 0x22 in order to access this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "34: This field must be written with 0x22 to be able to clear any of the enable bits"]
    _ToUnlockW_ = 34,
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
This field must be configured with 0x22 in order to access this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "This field must be written with 0x22 to be able to clear any of the enable bits"]
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
This bit clears SECAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn secapen(&mut self) -> SecapenW<SpecialAuthClrSpec> {
        SecapenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit clears FAKESTBYEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn fakestbyen(&mut self) -> FakestbyenW<SpecialAuthClrSpec> {
        FakestbyenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit clears DFTTAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn dftapen(&mut self) -> DftapenW<SpecialAuthClrSpec> {
        DftapenW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<SpecialAuthClrSpec> {
        Reserved3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit clears CFGAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn cfgapen(&mut self) -> CfgapenW<SpecialAuthClrSpec> {
        CfgapenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit clears AHBAPEN in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn ahbapen(&mut self) -> AhbapenW<SpecialAuthClrSpec> {
        AhbapenW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This bit clears DBGDIS in SPECIAL_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn dbgdis(&mut self) -> DbgdisW<SpecialAuthClrSpec> {
        DbgdisW::new(self, 6)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This field must be configured with 0x22 in order to access this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<SpecialAuthClrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "This register is used for clearing bits in SPECIAL_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpecialAuthClrSpec;
impl crate::RegisterSpec for SpecialAuthClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`special_auth_clr::R`](R) reader structure"]
impl crate::Readable for SpecialAuthClrSpec {}
#[doc = "`write(|w| ..)` method takes [`special_auth_clr::W`](W) writer structure"]
impl crate::Writable for SpecialAuthClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SPECIAL_AUTH_CLR to value 0"]
impl crate::Resettable for SpecialAuthClrSpec {
    const RESET_VALUE: u32 = 0;
}
