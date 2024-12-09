#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "0:0\\]
Status of Global lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glock {
    #[doc = "1: Locked"]
    Locked = 1,
    #[doc = "0: Unlocked"]
    Unlocked = 0,
}
impl From<Glock> for bool {
    #[inline(always)]
    fn from(variant: Glock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLOCK` reader - 0:0\\]
Status of Global lock"]
pub type GlockR = crate::BitReader<Glock>;
impl GlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Glock {
        match self.bits {
            true => Glock::Locked,
            false => Glock::Unlocked,
        }
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Glock::Locked
    }
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Glock::Unlocked
    }
}
#[doc = "1:1\\]
Lock status of registers in mutable section.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mlock {
    #[doc = "1: Registers in the mutable section are locked for write"]
    Locked = 1,
    #[doc = "0: Registers in the mutable section are not locked, and are writable"]
    Unlocked = 0,
}
impl From<Mlock> for bool {
    #[inline(always)]
    fn from(variant: Mlock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MLOCK` reader - 1:1\\]
Lock status of registers in mutable section."]
pub type MlockR = crate::BitReader<Mlock>;
impl MlockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mlock {
        match self.bits {
            true => Mlock::Locked,
            false => Mlock::Unlocked,
        }
    }
    #[doc = "Registers in the mutable section are locked for write"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Mlock::Locked
    }
    #[doc = "Registers in the mutable section are not locked, and are writable"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Mlock::Unlocked
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
RESERVED"]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of Global lock"]
    #[inline(always)]
    pub fn glock(&self) -> GlockR {
        GlockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Lock status of registers in mutable section."]
    #[inline(always)]
    pub fn mlock(&self) -> MlockR {
        MlockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVED"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {}
#[doc = "Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
