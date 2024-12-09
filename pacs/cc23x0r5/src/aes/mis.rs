#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Register `MIS` writer"]
pub type W = crate::W<MisSpec>;
#[doc = "0:0\\]
Masked interrupt status for ECB Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbdone {
    #[doc = "1: Interrupt occurred"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ecbdone> for bool {
    #[inline(always)]
    fn from(variant: Ecbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBDONE` reader - 0:0\\]
Masked interrupt status for ECB Done"]
pub type EcbdoneR = crate::BitReader<Ecbdone>;
impl EcbdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbdone {
        match self.bits {
            true => Ecbdone::Set,
            false => Ecbdone::Clr,
        }
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ecbdone::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ecbdone::Clr
    }
}
#[doc = "1:1\\]
Masked interrupt status for ECB Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecbstart {
    #[doc = "1: Interrupt occurred"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Ecbstart> for bool {
    #[inline(always)]
    fn from(variant: Ecbstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECBSTART` reader - 1:1\\]
Masked interrupt status for ECB Start"]
pub type EcbstartR = crate::BitReader<Ecbstart>;
impl EcbstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecbstart {
        match self.bits {
            true => Ecbstart::Set,
            false => Ecbstart::Clr,
        }
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Ecbstart::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Ecbstart::Clr
    }
}
#[doc = "2:2\\]
Masked interrupt status for DMA Channel A Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chadone {
    #[doc = "1: Interrupt occurred"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Chadone> for bool {
    #[inline(always)]
    fn from(variant: Chadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHADONE` reader - 2:2\\]
Masked interrupt status for DMA Channel A Done"]
pub type ChadoneR = crate::BitReader<Chadone>;
impl ChadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chadone {
        match self.bits {
            true => Chadone::Set,
            false => Chadone::Clr,
        }
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Chadone::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Chadone::Clr
    }
}
#[doc = "3:3\\]
Masked interrupt status for DMA Channel B Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chbdone {
    #[doc = "1: Interrupt occurred"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Chbdone> for bool {
    #[inline(always)]
    fn from(variant: Chbdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHBDONE` reader - 3:3\\]
Masked interrupt status for DMA Channel B Done"]
pub type ChbdoneR = crate::BitReader<Chbdone>;
impl ChbdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chbdone {
        match self.bits {
            true => Chbdone::Set,
            false => Chbdone::Clr,
        }
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Chbdone::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Chbdone::Clr
    }
}
#[doc = "Field `RESERVED2` reader - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Masked interrupt status for ECB Done"]
    #[inline(always)]
    pub fn ecbdone(&self) -> EcbdoneR {
        EcbdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Masked interrupt status for ECB Start"]
    #[inline(always)]
    pub fn ecbstart(&self) -> EcbstartR {
        EcbstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Masked interrupt status for DMA Channel A Done"]
    #[inline(always)]
    pub fn chadone(&self) -> ChadoneR {
        ChadoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Masked interrupt status for DMA Channel B Done"]
    #[inline(always)]
    pub fn chbdone(&self) -> ChbdoneR {
        ChbdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {}
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`write(|w| ..)` method takes [`mis::W`](W) writer structure"]
impl crate::Writable for MisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
