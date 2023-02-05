#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "1: Interrupt will request an interrupt service routine and corresponding bit in IPSTANDARD.MIS will be set"]
    Enabled = 1,
    #[doc = "0: Interrupt is masked out"]
    Disabled = 0,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            true => Done::Enabled,
            false => Done::Disabled,
        }
    }
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in IPSTANDARD.MIS will be set"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Done::Enabled
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Done::Disabled
    }
}
#[doc = "Field `DONE` writer - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt will request an interrupt service routine and corresponding bit in IPSTANDARD.MIS will be set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Enabled)
    }
    #[doc = "Interrupt is masked out"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Disabled)
    }
}
#[doc = "Field `RESERVED_31_1` reader - 31:1\\]
Reserved"]
pub type Reserved31_1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_31_1` writer - 31:1\\]
Reserved"]
pub type Reserved31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn reserved_31_1(&self) -> Reserved31_1R {
        Reserved31_1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt mask for DONE: 0: Interrupt is disabled in MIS register 1: Interrupt is enabled in MIS register"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<ImaskSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_1(&mut self) -> Reserved31_1W<ImaskSpec> {
        Reserved31_1W::new(self, 1)
    }
}
#[doc = "Interrupt Mask Register: The IMASK register holds the current interrupt mask settings. Masked interrupts are read in the MIS register. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
