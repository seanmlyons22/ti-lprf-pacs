#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Register `DESC` writer"]
pub type W = crate::W<DescSpec>;
#[doc = "3:0\\]
Minor revision version of IP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Minrev {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Minrev> for u8 {
    #[inline(always)]
    fn from(variant: Minrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Minrev {
    type Ux = u8;
}
impl crate::IsEnum for Minrev {}
#[doc = "Field `MINREV` reader - 3:0\\]
Minor revision version of IP"]
pub type MinrevR = crate::FieldReader<Minrev>;
impl MinrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Minrev> {
        match self.bits {
            15 => Some(Minrev::Maximum),
            0 => Some(Minrev::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Minrev::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Minrev::Minimum
    }
}
#[doc = "7:4\\]
Major revision version of IP\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Majrev {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Majrev> for u8 {
    #[inline(always)]
    fn from(variant: Majrev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Majrev {
    type Ux = u8;
}
impl crate::IsEnum for Majrev {}
#[doc = "Field `MAJREV` reader - 7:4\\]
Major revision version of IP"]
pub type MajrevR = crate::FieldReader<Majrev>;
impl MajrevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Majrev> {
        match self.bits {
            15 => Some(Majrev::Maximum),
            0 => Some(Majrev::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Majrev::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Majrev::Minimum
    }
}
#[doc = "11:8\\]
If multiple instances of IP exists in SOC, this field can identify the instance number 0-15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Instidx {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Instidx> for u8 {
    #[inline(always)]
    fn from(variant: Instidx) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Instidx {
    type Ux = u8;
}
impl crate::IsEnum for Instidx {}
#[doc = "Field `INSTIDX` reader - 11:8\\]
If multiple instances of IP exists in SOC, this field can identify the instance number 0-15"]
pub type InstidxR = crate::FieldReader<Instidx>;
impl InstidxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Instidx> {
        match self.bits {
            15 => Some(Instidx::Maximum),
            0 => Some(Instidx::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Instidx::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Instidx::Minimum
    }
}
#[doc = "15:12\\]
0: STDIP MMRs do not exist 1:15: These MMRs begin at offset 64*STDIPOFF from IP base address\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stdipoff {
    #[doc = "15: Highest possible value"]
    Maximum = 15,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Stdipoff> for u8 {
    #[inline(always)]
    fn from(variant: Stdipoff) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stdipoff {
    type Ux = u8;
}
impl crate::IsEnum for Stdipoff {}
#[doc = "Field `STDIPOFF` reader - 15:12\\]
0: STDIP MMRs do not exist 1:15: These MMRs begin at offset 64*STDIPOFF from IP base address"]
pub type StdipoffR = crate::FieldReader<Stdipoff>;
impl StdipoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Stdipoff> {
        match self.bits {
            15 => Some(Stdipoff::Maximum),
            0 => Some(Stdipoff::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Stdipoff::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Stdipoff::Minimum
    }
}
#[doc = "31:16\\]
Module identifier\n\nValue on reset: 31817"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Modid {
    #[doc = "65535: Highest possible value"]
    Maximum = 65535,
    #[doc = "0: Smallest value"]
    Minimum = 0,
}
impl From<Modid> for u16 {
    #[inline(always)]
    fn from(variant: Modid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modid {
    type Ux = u16;
}
impl crate::IsEnum for Modid {}
#[doc = "Field `MODID` reader - 31:16\\]
Module identifier"]
pub type ModidR = crate::FieldReader<Modid>;
impl ModidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modid> {
        match self.bits {
            65535 => Some(Modid::Maximum),
            0 => Some(Modid::Minimum),
            _ => None,
        }
    }
    #[doc = "Highest possible value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Modid::Maximum
    }
    #[doc = "Smallest value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Modid::Minimum
    }
}
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Minor revision version of IP"]
    #[inline(always)]
    pub fn minrev(&self) -> MinrevR {
        MinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Major revision version of IP"]
    #[inline(always)]
    pub fn majrev(&self) -> MajrevR {
        MajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
If multiple instances of IP exists in SOC, this field can identify the instance number 0-15"]
    #[inline(always)]
    pub fn instidx(&self) -> InstidxR {
        InstidxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
0: STDIP MMRs do not exist 1:15: These MMRs begin at offset 64*STDIPOFF from IP base address"]
    #[inline(always)]
    pub fn stdipoff(&self) -> StdipoffR {
        StdipoffR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Module identifier"]
    #[inline(always)]
    pub fn modid(&self) -> ModidR {
        ModidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Provides module ID, revision information, instance index\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`write(|w| ..)` method takes [`desc::W`](W) writer structure"]
impl crate::Writable for DescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESC to value 0x7c49_1010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x7c49_1010;
}
