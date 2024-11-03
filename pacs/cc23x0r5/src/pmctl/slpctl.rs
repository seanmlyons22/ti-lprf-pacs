#[doc = "Register `SLPCTL` reader"]
pub type R = crate::R<SlpctlSpec>;
#[doc = "Register `SLPCTL` writer"]
pub type W = crate::W<SlpctlSpec>;
#[doc = "0:0\\]
The boot code will set this bit field and disable sleep mode, automatically unless waking up from a SHUTDOWN RSTSTA.SDDET is set. Application software must reconfigure the state for all IO's before setting this bit field upon waking up from a SHUTDOWN to avoid glitches on pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slpn {
    #[doc = "1: I/O pad sleep mode is disabled"]
    Dis = 1,
    #[doc = "0: I/O pad sleep mode is enabled"]
    En = 0,
}
impl From<Slpn> for bool {
    #[inline(always)]
    fn from(variant: Slpn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLPN` reader - 0:0\\]
The boot code will set this bit field and disable sleep mode, automatically unless waking up from a SHUTDOWN RSTSTA.SDDET is set. Application software must reconfigure the state for all IO's before setting this bit field upon waking up from a SHUTDOWN to avoid glitches on pins."]
pub type SlpnR = crate::BitReader<Slpn>;
impl SlpnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slpn {
        match self.bits {
            true => Slpn::Dis,
            false => Slpn::En,
        }
    }
    #[doc = "I/O pad sleep mode is disabled"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Slpn::Dis
    }
    #[doc = "I/O pad sleep mode is enabled"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Slpn::En
    }
}
#[doc = "Field `SLPN` writer - 0:0\\]
The boot code will set this bit field and disable sleep mode, automatically unless waking up from a SHUTDOWN RSTSTA.SDDET is set. Application software must reconfigure the state for all IO's before setting this bit field upon waking up from a SHUTDOWN to avoid glitches on pins."]
pub type SlpnW<'a, REG> = crate::BitWriter<'a, REG, Slpn>;
impl<'a, REG> SlpnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I/O pad sleep mode is disabled"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Slpn::Dis)
    }
    #[doc = "I/O pad sleep mode is enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Slpn::En)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
The boot code will set this bit field and disable sleep mode, automatically unless waking up from a SHUTDOWN RSTSTA.SDDET is set. Application software must reconfigure the state for all IO's before setting this bit field upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    pub fn slpn(&self) -> SlpnR {
        SlpnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
The boot code will set this bit field and disable sleep mode, automatically unless waking up from a SHUTDOWN RSTSTA.SDDET is set. Application software must reconfigure the state for all IO's before setting this bit field upon waking up from a SHUTDOWN to avoid glitches on pins."]
    #[inline(always)]
    #[must_use]
    pub fn slpn(&mut self) -> SlpnW<SlpctlSpec> {
        SlpnW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SlpctlSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Sleep Control Register. This register controls I/O pad sleep mode. When I/O pad sleep mode is enabled all I/O pad outputs and I/O pad configurations are latched. Inputs are transparent if I/O pad is configured as input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlpctlSpec;
impl crate::RegisterSpec for SlpctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slpctl::R`](R) reader structure"]
impl crate::Readable for SlpctlSpec {}
#[doc = "`write(|w| ..)` method takes [`slpctl::W`](W) writer structure"]
impl crate::Writable for SlpctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLPCTL to value 0"]
impl crate::Resettable for SlpctlSpec {
    const RESET_VALUE: u32 = 0;
}
