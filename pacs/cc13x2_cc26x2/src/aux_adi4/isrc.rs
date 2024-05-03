#[doc = "Register `ISRC` reader"]
pub type R = crate::R<IsrcSpec>;
#[doc = "Register `ISRC` writer"]
pub type W = crate::W<IsrcSpec>;
#[doc = "Field `EN` reader - 0:0\\]
Current source enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 0:0\\]
Current source enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trim {
    #[doc = "32: 11.75 uA"]
    _11p75u = 32,
    #[doc = "16: 4.5 uA"]
    _4p5u = 16,
    #[doc = "8: 2.0 uA"]
    _2p0u = 8,
    #[doc = "4: 1.0 uA"]
    _1p0u = 4,
    #[doc = "2: 0.5 uA"]
    _0p5u = 2,
    #[doc = "1: 0.25 uA"]
    _0p25u = 1,
    #[doc = "0: No current connected"]
    Nc = 0,
}
impl From<Trim> for u8 {
    #[inline(always)]
    fn from(variant: Trim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trim {
    type Ux = u8;
}
impl crate::IsEnum for Trim {}
#[doc = "Field `TRIM` reader - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
pub type TrimR = crate::FieldReader<Trim>;
impl TrimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trim> {
        match self.bits {
            32 => Some(Trim::_11p75u),
            16 => Some(Trim::_4p5u),
            8 => Some(Trim::_2p0u),
            4 => Some(Trim::_1p0u),
            2 => Some(Trim::_0p5u),
            1 => Some(Trim::_0p25u),
            0 => Some(Trim::Nc),
            _ => None,
        }
    }
    #[doc = "11.75 uA"]
    #[inline(always)]
    pub fn is_11p75u(&self) -> bool {
        *self == Trim::_11p75u
    }
    #[doc = "4.5 uA"]
    #[inline(always)]
    pub fn is_4p5u(&self) -> bool {
        *self == Trim::_4p5u
    }
    #[doc = "2.0 uA"]
    #[inline(always)]
    pub fn is_2p0u(&self) -> bool {
        *self == Trim::_2p0u
    }
    #[doc = "1.0 uA"]
    #[inline(always)]
    pub fn is_1p0u(&self) -> bool {
        *self == Trim::_1p0u
    }
    #[doc = "0.5 uA"]
    #[inline(always)]
    pub fn is_0p5u(&self) -> bool {
        *self == Trim::_0p5u
    }
    #[doc = "0.25 uA"]
    #[inline(always)]
    pub fn is_0p25u(&self) -> bool {
        *self == Trim::_0p25u
    }
    #[doc = "No current connected"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Trim::Nc
    }
}
#[doc = "Field `TRIM` writer - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
pub type TrimW<'a, REG> = crate::FieldWriter<'a, REG, 6, Trim>;
impl<'a, REG> TrimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "11.75 uA"]
    #[inline(always)]
    pub fn _11p75u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_11p75u)
    }
    #[doc = "4.5 uA"]
    #[inline(always)]
    pub fn _4p5u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_4p5u)
    }
    #[doc = "2.0 uA"]
    #[inline(always)]
    pub fn _2p0u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_2p0u)
    }
    #[doc = "1.0 uA"]
    #[inline(always)]
    pub fn _1p0u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_1p0u)
    }
    #[doc = "0.5 uA"]
    #[inline(always)]
    pub fn _0p5u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_0p5u)
    }
    #[doc = "0.25 uA"]
    #[inline(always)]
    pub fn _0p25u(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::_0p25u)
    }
    #[doc = "No current connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(Trim::Nc)
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    pub fn trim(&self) -> TrimR {
        TrimR::new((self.bits >> 2) & 0x3f)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Current source enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<IsrcSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IsrcSpec> {
        Reserved1W::new(self, 1)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Adjust current from current source. Output currents may be combined to get desired total current."]
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TrimW<IsrcSpec> {
        TrimW::new(self, 2)
    }
}
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isrc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isrc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrcSpec;
impl crate::RegisterSpec for IsrcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`isrc::R`](R) reader structure"]
impl crate::Readable for IsrcSpec {}
#[doc = "`write(|w| ..)` method takes [`isrc::W`](W) writer structure"]
impl crate::Writable for IsrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ISRC to value 0"]
impl crate::Resettable for IsrcSpec {
    const RESET_VALUE: u8 = 0;
}
