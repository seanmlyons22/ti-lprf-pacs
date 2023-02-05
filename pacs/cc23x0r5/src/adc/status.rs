#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "0:0\\]
Busy. This bit indicates that an active ADC sample or conversion operation is in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "1: ADC sampling or conversion is in progress."]
    Active = 1,
    #[doc = "0: No ADC sampling or conversion in progress."]
    Idle = 0,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - 0:0\\]
Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            true => Busy::Active,
            false => Busy::Idle,
        }
    }
    #[doc = "ADC sampling or conversion is in progress."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Busy::Active
    }
    #[doc = "No ADC sampling or conversion in progress."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Busy::Idle
    }
}
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::BitReader;
#[doc = "2:2\\]
ASC active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ascact {
    #[doc = "1: ASC active"]
    Active = 1,
    #[doc = "0: Idle or done"]
    Idle = 0,
}
impl From<Ascact> for bool {
    #[inline(always)]
    fn from(variant: Ascact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ASCACT` reader - 2:2\\]
ASC active"]
pub type AscactR = crate::BitReader<Ascact>;
impl AscactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ascact {
        match self.bits {
            true => Ascact::Active,
            false => Ascact::Idle,
        }
    }
    #[doc = "ASC active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Ascact::Active
    }
    #[doc = "Idle or done"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Ascact::Idle
    }
}
#[doc = "Field `RESERVED3` reader - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
ASC active"]
    #[inline(always)]
    pub fn ascact(&self) -> AscactR {
        AscactR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {}
#[doc = "Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
