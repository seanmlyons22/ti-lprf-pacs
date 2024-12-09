#[doc = "Register `DESCEX` reader"]
pub type R = crate::R<DescexSpec>;
#[doc = "Register `DESCEX` writer"]
pub type W = crate::W<DescexSpec>;
#[doc = "Field `RESERVED0` reader - 25:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "26:26\\]
LPCMP (low power comparator) IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpcmp {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Lpcmp> for bool {
    #[inline(always)]
    fn from(variant: Lpcmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPCMP` reader - 26:26\\]
LPCMP (low power comparator) IP status on device"]
pub type LpcmpR = crate::BitReader<Lpcmp>;
impl LpcmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpcmp {
        match self.bits {
            true => Lpcmp::IpAvail,
            false => Lpcmp::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Lpcmp::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Lpcmp::IpUnavail
    }
}
#[doc = "27:27\\]
TSD (thermal shutdown) IP status on device\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsd {
    #[doc = "1: IP is available"]
    IpAvail = 1,
    #[doc = "0: IP is unavailable"]
    IpUnavail = 0,
}
impl From<Tsd> for bool {
    #[inline(always)]
    fn from(variant: Tsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSD` reader - 27:27\\]
TSD (thermal shutdown) IP status on device"]
pub type TsdR = crate::BitReader<Tsd>;
impl TsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsd {
        match self.bits {
            true => Tsd::IpAvail,
            false => Tsd::IpUnavail,
        }
    }
    #[doc = "IP is available"]
    #[inline(always)]
    pub fn is_ip_avail(&self) -> bool {
        *self == Tsd::IpAvail
    }
    #[doc = "IP is unavailable"]
    #[inline(always)]
    pub fn is_ip_unavail(&self) -> bool {
        *self == Tsd::IpUnavail
    }
}
#[doc = "29:28\\]
System SRAM availability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsz {
    #[doc = "3: SRAM size set to level 3 (Max size)"]
    Sz3 = 3,
    #[doc = "2: SRAM size set to level 2"]
    Sz2 = 2,
    #[doc = "1: SRAM size set to level 1"]
    Sz1 = 1,
    #[doc = "0: SRAM size set to level 0 (Min size)"]
    Sz0 = 0,
}
impl From<Sramsz> for u8 {
    #[inline(always)]
    fn from(variant: Sramsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsz {
    type Ux = u8;
}
impl crate::IsEnum for Sramsz {}
#[doc = "Field `SRAMSZ` reader - 29:28\\]
System SRAM availability"]
pub type SramszR = crate::FieldReader<Sramsz>;
impl SramszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sramsz {
        match self.bits {
            3 => Sramsz::Sz3,
            2 => Sramsz::Sz2,
            1 => Sramsz::Sz1,
            0 => Sramsz::Sz0,
            _ => unreachable!(),
        }
    }
    #[doc = "SRAM size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn is_sz3(&self) -> bool {
        *self == Sramsz::Sz3
    }
    #[doc = "SRAM size set to level 2"]
    #[inline(always)]
    pub fn is_sz2(&self) -> bool {
        *self == Sramsz::Sz2
    }
    #[doc = "SRAM size set to level 1"]
    #[inline(always)]
    pub fn is_sz1(&self) -> bool {
        *self == Sramsz::Sz1
    }
    #[doc = "SRAM size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn is_sz0(&self) -> bool {
        *self == Sramsz::Sz0
    }
}
#[doc = "31:30\\]
System flash availability\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashsz {
    #[doc = "3: Flash size set to level 3 (Max size)"]
    Sz3 = 3,
    #[doc = "2: Flash size set to level 2"]
    Sz2 = 2,
    #[doc = "1: Flash size set to level 1"]
    Sz1 = 1,
    #[doc = "0: Flash size set to level 0 (Min size)"]
    Sz0 = 0,
}
impl From<Flashsz> for u8 {
    #[inline(always)]
    fn from(variant: Flashsz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashsz {
    type Ux = u8;
}
impl crate::IsEnum for Flashsz {}
#[doc = "Field `FLASHSZ` reader - 31:30\\]
System flash availability"]
pub type FlashszR = crate::FieldReader<Flashsz>;
impl FlashszR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashsz {
        match self.bits {
            3 => Flashsz::Sz3,
            2 => Flashsz::Sz2,
            1 => Flashsz::Sz1,
            0 => Flashsz::Sz0,
            _ => unreachable!(),
        }
    }
    #[doc = "Flash size set to level 3 (Max size)"]
    #[inline(always)]
    pub fn is_sz3(&self) -> bool {
        *self == Flashsz::Sz3
    }
    #[doc = "Flash size set to level 2"]
    #[inline(always)]
    pub fn is_sz2(&self) -> bool {
        *self == Flashsz::Sz2
    }
    #[doc = "Flash size set to level 1"]
    #[inline(always)]
    pub fn is_sz1(&self) -> bool {
        *self == Flashsz::Sz1
    }
    #[doc = "Flash size set to level 0 (Min size)"]
    #[inline(always)]
    pub fn is_sz0(&self) -> bool {
        *self == Flashsz::Sz0
    }
}
impl R {
    #[doc = "Bits 0:25 - 25:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bit 26 - 26:26\\]
LPCMP (low power comparator) IP status on device"]
    #[inline(always)]
    pub fn lpcmp(&self) -> LpcmpR {
        LpcmpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
TSD (thermal shutdown) IP status on device"]
    #[inline(always)]
    pub fn tsd(&self) -> TsdR {
        TsdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - 29:28\\]
System SRAM availability"]
    #[inline(always)]
    pub fn sramsz(&self) -> SramszR {
        SramszR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
System flash availability"]
    #[inline(always)]
    pub fn flashsz(&self) -> FlashszR {
        FlashszR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {}
#[doc = "Extended Description Register. This register shows ULL IP availability and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescexSpec;
impl crate::RegisterSpec for DescexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descex::R`](R) reader structure"]
impl crate::Readable for DescexSpec {}
#[doc = "`write(|w| ..)` method takes [`descex::W`](W) writer structure"]
impl crate::Writable for DescexSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DESCEX to value 0xfc00_0000"]
impl crate::Resettable for DescexSpec {
    const RESET_VALUE: u32 = 0xfc00_0000;
}
