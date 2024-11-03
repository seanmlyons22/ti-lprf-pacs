#[doc = "Register `SHTDWN` reader"]
pub type R = crate::R<ShtdwnSpec>;
#[doc = "Register `SHTDWN` writer"]
pub type W = crate::W<ShtdwnSpec>;
#[doc = "15:0\\]
Setting a valid key will trigger the device to enter SHUTDOWN mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Key {
    #[doc = "42405: This is the only valid key value that will trigger SHUTDOWN mode. All other values are invalid and will have no effect."]
    Valid = 42405,
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(variant: Key) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Key {
    type Ux = u16;
}
impl crate::IsEnum for Key {}
#[doc = "Field `KEY` reader - 15:0\\]
Setting a valid key will trigger the device to enter SHUTDOWN mode."]
pub type KeyR = crate::FieldReader<Key>;
impl KeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Key> {
        match self.bits {
            42405 => Some(Key::Valid),
            _ => None,
        }
    }
    #[doc = "This is the only valid key value that will trigger SHUTDOWN mode. All other values are invalid and will have no effect."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Key::Valid
    }
}
#[doc = "Field `KEY` writer - 15:0\\]
Setting a valid key will trigger the device to enter SHUTDOWN mode."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Key>;
impl<'a, REG> KeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "This is the only valid key value that will trigger SHUTDOWN mode. All other values are invalid and will have no effect."]
    #[inline(always)]
    pub fn valid(self) -> &'a mut crate::W<REG> {
        self.variant(Key::Valid)
    }
}
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Setting a valid key will trigger the device to enter SHUTDOWN mode."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Setting a valid key will trigger the device to enter SHUTDOWN mode."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<ShtdwnSpec> {
        KeyW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<ShtdwnSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "Shutdown Register. This register controls SHUTDOWN mode entry.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shtdwn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shtdwn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShtdwnSpec;
impl crate::RegisterSpec for ShtdwnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shtdwn::R`](R) reader structure"]
impl crate::Readable for ShtdwnSpec {}
#[doc = "`write(|w| ..)` method takes [`shtdwn::W`](W) writer structure"]
impl crate::Writable for ShtdwnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHTDWN to value 0"]
impl crate::Resettable for ShtdwnSpec {
    const RESET_VALUE: u32 = 0;
}
