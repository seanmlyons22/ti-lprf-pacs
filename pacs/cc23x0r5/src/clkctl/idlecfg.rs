#[doc = "Register `IDLECFG` reader"]
pub type R = crate::R<IdlecfgSpec>;
#[doc = "Register `IDLECFG` writer"]
pub type W = crate::W<IdlecfgSpec>;
#[doc = "0:0\\]
Internal. Only to be used through TI provided API.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "1: Internal. Only to be used through TI provided API."]
    LdoOff = 1,
    #[doc = "0: Internal. Only to be used through TI provided API."]
    LdoOn = 0,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            true => Mode::LdoOff,
            false => Mode::LdoOn,
        }
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ldo_off(&self) -> bool {
        *self == Mode::LdoOff
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn is_ldo_on(&self) -> bool {
        *self == Mode::LdoOn
    }
}
#[doc = "Field `MODE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldo_off(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::LdoOff)
    }
    #[doc = "Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldo_on(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::LdoOn)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<IdlecfgSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idlecfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idlecfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdlecfgSpec;
impl crate::RegisterSpec for IdlecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idlecfg::R`](R) reader structure"]
impl crate::Readable for IdlecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`idlecfg::W`](W) writer structure"]
impl crate::Writable for IdlecfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDLECFG to value 0"]
impl crate::Resettable for IdlecfgSpec {
    const RESET_VALUE: u32 = 0;
}
