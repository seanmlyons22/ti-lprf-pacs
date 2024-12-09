#[doc = "Register `DFTPCLKTESTSTAT` reader"]
pub type R = crate::R<DftpclkteststatSpec>;
#[doc = "Register `DFTPCLKTESTSTAT` writer"]
pub type W = crate::W<DftpclkteststatSpec>;
#[doc = "0:0\\]
Indicates that a pump clock measurement is in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: Indicates test in progress"]
    Inprogress = 1,
    #[doc = "0: Indicates test complete"]
    Complete = 0,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - 0:0\\]
Indicates that a pump clock measurement is in progress."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            true => Busy::Inprogress,
            false => Busy::Complete,
        }
    }
    #[doc = "Indicates test in progress"]
    #[inline(always)]
    pub fn is_inprogress(&self) -> bool {
        *self == Busy::Inprogress
    }
    #[doc = "Indicates test complete"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Busy::Complete
    }
}
#[doc = "Field `RESERVED1` reader - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader;
#[doc = "15:4\\]
Indicates the core clock count captured during the pump clock measurement.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Clockcnt {
    #[doc = "4095: Maximum count value"]
    Maximum = 4095,
    #[doc = "0: Minimum count value"]
    Minimum = 0,
}
impl From<Clockcnt> for u16 {
    #[inline(always)]
    fn from(variant: Clockcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clockcnt {
    type Ux = u16;
}
impl crate::IsEnum for Clockcnt {}
#[doc = "Field `CLOCKCNT` reader - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
pub type ClockcntR = crate::FieldReader<Clockcnt>;
impl ClockcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clockcnt> {
        match self.bits {
            4095 => Some(Clockcnt::Maximum),
            0 => Some(Clockcnt::Minimum),
            _ => None,
        }
    }
    #[doc = "Maximum count value"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == Clockcnt::Maximum
    }
    #[doc = "Minimum count value"]
    #[inline(always)]
    pub fn is_minimum(&self) -> bool {
        *self == Clockcnt::Minimum
    }
}
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates that a pump clock measurement is in progress."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - 3:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Indicates the core clock count captured during the pump clock measurement."]
    #[inline(always)]
    pub fn clockcnt(&self) -> ClockcntR {
        ClockcntR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "DFT Pump Clock Test Status Register. This register shows status reported by the hardware features that allow the pump clock to be characterized for trim development. This register is only writable when DFTEN.ENABLE is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dftpclkteststat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dftpclkteststat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DftpclkteststatSpec;
impl crate::RegisterSpec for DftpclkteststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dftpclkteststat::R`](R) reader structure"]
impl crate::Readable for DftpclkteststatSpec {}
#[doc = "`write(|w| ..)` method takes [`dftpclkteststat::W`](W) writer structure"]
impl crate::Writable for DftpclkteststatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFTPCLKTESTSTAT to value 0"]
impl crate::Resettable for DftpclkteststatSpec {
    const RESET_VALUE: u32 = 0;
}
