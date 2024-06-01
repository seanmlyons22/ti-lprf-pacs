#[doc = "Register `RFCMODEHWOPT` reader"]
pub type R = crate::R<RfcmodehwoptSpec>;
#[doc = "Register `RFCMODEHWOPT` writer"]
pub type W = crate::W<RfcmodehwoptSpec>;
#[doc = "7:0\\]
Permitted RFC modes. More than one mode can be permitted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avail {
    #[doc = "128: Mode 7 permitted"]
    Mode7 = 128,
    #[doc = "64: Mode 6 permitted"]
    Mode6 = 64,
    #[doc = "32: Mode 5 permitted"]
    Mode5 = 32,
    #[doc = "16: Mode 4 permitted"]
    Mode4 = 16,
    #[doc = "8: Mode 3 permitted"]
    Mode3 = 8,
    #[doc = "4: Mode 2 permitted"]
    Mode2 = 4,
    #[doc = "2: Mode 1 permitted"]
    Mode1 = 2,
    #[doc = "1: Mode 0 permitted"]
    Mode0 = 1,
}
impl From<Avail> for u8 {
    #[inline(always)]
    fn from(variant: Avail) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avail {
    type Ux = u8;
}
impl crate::IsEnum for Avail {}
#[doc = "Field `AVAIL` reader - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
pub type AvailR = crate::FieldReader<Avail>;
impl AvailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Avail> {
        match self.bits {
            128 => Some(Avail::Mode7),
            64 => Some(Avail::Mode6),
            32 => Some(Avail::Mode5),
            16 => Some(Avail::Mode4),
            8 => Some(Avail::Mode3),
            4 => Some(Avail::Mode2),
            2 => Some(Avail::Mode1),
            1 => Some(Avail::Mode0),
            _ => None,
        }
    }
    #[doc = "Mode 7 permitted"]
    #[inline(always)]
    pub fn is_mode7(&self) -> bool {
        *self == Avail::Mode7
    }
    #[doc = "Mode 6 permitted"]
    #[inline(always)]
    pub fn is_mode6(&self) -> bool {
        *self == Avail::Mode6
    }
    #[doc = "Mode 5 permitted"]
    #[inline(always)]
    pub fn is_mode5(&self) -> bool {
        *self == Avail::Mode5
    }
    #[doc = "Mode 4 permitted"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Avail::Mode4
    }
    #[doc = "Mode 3 permitted"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Avail::Mode3
    }
    #[doc = "Mode 2 permitted"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Avail::Mode2
    }
    #[doc = "Mode 1 permitted"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Avail::Mode1
    }
    #[doc = "Mode 0 permitted"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == Avail::Mode0
    }
}
#[doc = "Field `AVAIL` writer - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
pub type AvailW<'a, REG> = crate::FieldWriter<'a, REG, 8, Avail>;
impl<'a, REG> AvailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 7 permitted"]
    #[inline(always)]
    pub fn mode7(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode7)
    }
    #[doc = "Mode 6 permitted"]
    #[inline(always)]
    pub fn mode6(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode6)
    }
    #[doc = "Mode 5 permitted"]
    #[inline(always)]
    pub fn mode5(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode5)
    }
    #[doc = "Mode 4 permitted"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode4)
    }
    #[doc = "Mode 3 permitted"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode3)
    }
    #[doc = "Mode 2 permitted"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode2)
    }
    #[doc = "Mode 1 permitted"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode1)
    }
    #[doc = "Mode 0 permitted"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut crate::W<REG> {
        self.variant(Avail::Mode0)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    pub fn avail(&self) -> AvailR {
        AvailR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Permitted RFC modes. More than one mode can be permitted."]
    #[inline(always)]
    #[must_use]
    pub fn avail(&mut self) -> AvailW<RfcmodehwoptSpec> {
        AvailW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<RfcmodehwoptSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Allowed RFC Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcmodehwopt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcmodehwopt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfcmodehwoptSpec;
impl crate::RegisterSpec for RfcmodehwoptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfcmodehwopt::R`](R) reader structure"]
impl crate::Readable for RfcmodehwoptSpec {}
#[doc = "`write(|w| ..)` method takes [`rfcmodehwopt::W`](W) writer structure"]
impl crate::Writable for RfcmodehwoptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFCMODEHWOPT to value 0"]
impl crate::Resettable for RfcmodehwoptSpec {
    const RESET_VALUE: u32 = 0;
}
