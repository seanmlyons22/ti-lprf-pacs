#[doc = "Register `APP_AUTH_CLR` reader"]
pub type R = crate::R<AppAuthClrSpec>;
#[doc = "Register `APP_AUTH_CLR` writer"]
pub type W = crate::W<AppAuthClrSpec>;
#[doc = "0:0\\]
Clears DBGEN bit in APP_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbgen {
    #[doc = "1: Clears DBGEN"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Dbgen> for bool {
    #[inline(always)]
    fn from(variant: Dbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGEN` writer - 0:0\\]
Clears DBGEN bit in APP_AUTH register."]
pub type DbgenW<'a, REG> = crate::BitWriter<'a, REG, Dbgen>;
impl<'a, REG> DbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears DBGEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgen::Noeff)
    }
}
#[doc = "1:1\\]
Clears NIDEN bit in APP_AUTH register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Niden {
    #[doc = "1: Clears NIDEN"]
    Clr = 1,
    #[doc = "0: Writing 0 has no effect"]
    Noeff = 0,
}
impl From<Niden> for bool {
    #[inline(always)]
    fn from(variant: Niden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NIDEN` writer - 1:1\\]
Clears NIDEN bit in APP_AUTH register."]
pub type NidenW<'a, REG> = crate::BitWriter<'a, REG, Niden>;
impl<'a, REG> NidenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears NIDEN"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Clr)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Niden::Noeff)
    }
}
#[doc = "Field `RESERVED2` reader - 23:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED2` writer - 23:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "31:24\\]
This field must be configured with 0x7D in order to access this register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    #[doc = "125: Write this value 0x7D to unlock writing to the APP_AUTH_CLR register"]
    _ToUnlockW_ = 125,
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
This field must be configured with 0x7D in order to access this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Write this value 0x7D to unlock writing to the APP_AUTH_CLR register"]
    #[inline(always)]
    pub fn _to_unlock_w_(self) -> &'a mut crate::W<REG> {
        self.variant(Key::_ToUnlockW_)
    }
}
impl R {
    #[doc = "Bits 2:23 - 23:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears DBGEN bit in APP_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DbgenW<AppAuthClrSpec> {
        DbgenW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears NIDEN bit in APP_AUTH register."]
    #[inline(always)]
    #[must_use]
    pub fn niden(&mut self) -> NidenW<AppAuthClrSpec> {
        NidenW::new(self, 1)
    }
    #[doc = "Bits 2:23 - 23:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<AppAuthClrSpec> {
        Reserved2W::new(self, 2)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This field must be configured with 0x7D in order to access this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<AppAuthClrSpec> {
        KeyW::new(self, 24)
    }
}
#[doc = "This register is used for clearing bits in APP_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_auth_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppAuthClrSpec;
impl crate::RegisterSpec for AppAuthClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_auth_clr::R`](R) reader structure"]
impl crate::Readable for AppAuthClrSpec {}
#[doc = "`write(|w| ..)` method takes [`app_auth_clr::W`](W) writer structure"]
impl crate::Writable for AppAuthClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APP_AUTH_CLR to value 0"]
impl crate::Resettable for AppAuthClrSpec {
    const RESET_VALUE: u32 = 0;
}
