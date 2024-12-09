#[doc = "Register `BANK1INFO1` reader"]
pub type R = crate::R<Bank1info1Spec>;
#[doc = "Register `BANK1INFO1` writer"]
pub type W = crate::W<Bank1info1Spec>;
#[doc = "7:0\\]
Non-main region size in sectors\n\nValue on reset: 1"]
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
Non-main region size in sectors"]
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
#[doc = "15:8\\]
Trim region size in sectors\n\nValue on reset: 1"]
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
Trim region size in sectors"]
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
#[doc = "23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)\n\nValue on reset: 1"]
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
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
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
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Non-main region size in sectors"]
    #[inline(always)]
    pub fn nonmainsize(&self) -> NonmainsizeR {
        NonmainsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Trim region size in sectors"]
    #[inline(always)]
    pub fn trimsize(&self) -> TrimsizeR {
        TrimsizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Engr region size in sectors Minimum:0x0 (0) Maximum:0x10 (16)"]
    #[inline(always)]
    pub fn engrsize(&self) -> EngrsizeR {
        EngrsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Bank Info1 Register for bank 1. Read only register detailing information about Non-Main, Trim, and Engr region sizes in the bank.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bank1info1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bank1info1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bank1info1Spec;
impl crate::RegisterSpec for Bank1info1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bank1info1::R`](R) reader structure"]
impl crate::Readable for Bank1info1Spec {}
#[doc = "`write(|w| ..)` method takes [`bank1info1::W`](W) writer structure"]
impl crate::Writable for Bank1info1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANK1INFO1 to value 0x0001_0101"]
impl crate::Resettable for Bank1info1Spec {
    const RESET_VALUE: u32 = 0x0001_0101;
}
