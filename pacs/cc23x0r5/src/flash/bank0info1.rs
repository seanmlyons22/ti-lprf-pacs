#[doc = "Register `BANK0INFO1` reader"]
pub type R = crate::R<Bank0info1Spec>;
#[doc = "Register `BANK0INFO1` writer"]
pub type W = crate::W<Bank0info1Spec>;
#[doc = "7:0\\]
Non-main region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Nonmainsize {
    #[doc = "32: Maximum value of NONMAINSIZE"]
    Maxsectors = 32,
    #[doc = "0: Minimum value of NONMAINSIZE"]
    Minsectors = 0,
}
impl From<Nonmainsize> for u8 {
    #[inline(always)]
    fn from(variant: Nonmainsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Nonmainsize {
    type Ux = u8;
}
impl crate::IsEnum for Nonmainsize {}
#[doc = "Field `NONMAINSIZE` reader - 7:0\\]
Non-main region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type NonmainsizeR = crate::FieldReader<Nonmainsize>;
impl NonmainsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Nonmainsize> {
        match self.bits {
            32 => Some(Nonmainsize::Maxsectors),
            0 => Some(Nonmainsize::Minsectors),
            _ => None,
        }
    }
    #[doc = "Maximum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Nonmainsize::Maxsectors
    }
    #[doc = "Minimum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Nonmainsize::Minsectors
    }
}
#[doc = "Field `NONMAINSIZE` writer - 7:0\\]
Non-main region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type NonmainsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Nonmainsize>;
impl<'a, REG> NonmainsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Nonmainsize::Maxsectors)
    }
    #[doc = "Minimum value of NONMAINSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Nonmainsize::Minsectors)
    }
}
#[doc = "15:8\\]
Trim region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trimsize {
    #[doc = "32: Maximum value of TRIMSIZE"]
    Maxsectors = 32,
    #[doc = "0: Minimum value of TRIMSIZE"]
    Minsectors = 0,
}
impl From<Trimsize> for u8 {
    #[inline(always)]
    fn from(variant: Trimsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trimsize {
    type Ux = u8;
}
impl crate::IsEnum for Trimsize {}
#[doc = "Field `TRIMSIZE` reader - 15:8\\]
Trim region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type TrimsizeR = crate::FieldReader<Trimsize>;
impl TrimsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trimsize> {
        match self.bits {
            32 => Some(Trimsize::Maxsectors),
            0 => Some(Trimsize::Minsectors),
            _ => None,
        }
    }
    #[doc = "Maximum value of TRIMSIZE"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Trimsize::Maxsectors
    }
    #[doc = "Minimum value of TRIMSIZE"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Trimsize::Minsectors
    }
}
#[doc = "Field `TRIMSIZE` writer - 15:8\\]
Trim region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type TrimsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Trimsize>;
impl<'a, REG> TrimsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value of TRIMSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Trimsize::Maxsectors)
    }
    #[doc = "Minimum value of TRIMSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Trimsize::Minsectors)
    }
}
#[doc = "23:16\\]
Engr region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Engrsize {
    #[doc = "32: Maximum value of ENGRSIZE"]
    Maxsectors = 32,
    #[doc = "0: Minimum value of ENGRSIZE"]
    Minsectors = 0,
}
impl From<Engrsize> for u8 {
    #[inline(always)]
    fn from(variant: Engrsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Engrsize {
    type Ux = u8;
}
impl crate::IsEnum for Engrsize {}
#[doc = "Field `ENGRSIZE` reader - 23:16\\]
Engr region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type EngrsizeR = crate::FieldReader<Engrsize>;
impl EngrsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Engrsize> {
        match self.bits {
            32 => Some(Engrsize::Maxsectors),
            0 => Some(Engrsize::Minsectors),
            _ => None,
        }
    }
    #[doc = "Maximum value of ENGRSIZE"]
    #[inline(always)]
    pub fn is_maxsectors(&self) -> bool {
        *self == Engrsize::Maxsectors
    }
    #[doc = "Minimum value of ENGRSIZE"]
    #[inline(always)]
    pub fn is_minsectors(&self) -> bool {
        *self == Engrsize::Minsectors
    }
}
#[doc = "Field `ENGRSIZE` writer - 23:16\\]
Engr region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
pub type EngrsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Engrsize>;
impl<'a, REG> EngrsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Maximum value of ENGRSIZE"]
    #[inline(always)]
    pub fn maxsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Engrsize::Maxsectors)
    }
    #[doc = "Minimum value of ENGRSIZE"]
    #[inline(always)]
    pub fn minsectors(self) -> &'a mut crate::W<REG> {
        self.variant(Engrsize::Minsectors)
    }
}
#[doc = "Field `RESERVED_31_24` reader - 31:24\\]
Reserved"]
pub type Reserved31_24R = crate::FieldReader;
#[doc = "Field `RESERVED_31_24` writer - 31:24\\]
Reserved"]
pub type Reserved31_24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Non-main region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    pub fn nonmainsize(&self) -> NonmainsizeR {
        NonmainsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trim region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    pub fn trimsize(&self) -> TrimsizeR {
        TrimsizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Engr region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    pub fn engrsize(&self) -> EngrsizeR {
        EngrsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_24(&self) -> Reserved31_24R {
        Reserved31_24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Non-main region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn nonmainsize(&mut self) -> NonmainsizeW<Bank0info1Spec> {
        NonmainsizeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trim region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn trimsize(&mut self) -> TrimsizeW<Bank0info1Spec> {
        TrimsizeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Engr region size in sectors Minimum: 0x0 (0) Maximum: 0x10 (16)"]
    #[inline(always)]
    #[must_use]
    pub fn engrsize(&mut self) -> EngrsizeW<Bank0info1Spec> {
        EngrsizeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_24(&mut self) -> Reserved31_24W<Bank0info1Spec> {
        Reserved31_24W::new(self, 24)
    }
}
#[doc = "Bank Info1 Register for bank 0. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank0info1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank0info1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank0info1Spec;
impl crate::RegisterSpec for Bank0info1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank0info1::R`](R) reader structure"]
impl crate::Readable for Bank0info1Spec {}
#[doc = "`write(|w| ..)` method takes [`bank0info1::W`](W) writer structure"]
impl crate::Writable for Bank0info1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK0INFO1 to value 0x0001_0101"]
impl crate::Resettable for Bank0info1Spec {
    const RESET_VALUE: u32 = 0x0001_0101;
}
