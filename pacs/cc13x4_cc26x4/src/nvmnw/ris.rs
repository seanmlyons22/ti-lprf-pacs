#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Register `RIS` writer"]
pub type W = crate::W<RisSpec>;
#[doc = "0:0\\]
NoWrapper operation completed. This interrupt bit is set by firmware or the corresponding bit in the ISET register. It is cleared by the corresponding bit in in the ICLR register or reading the IIDX register when this interrupt is the highest priority.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "1: Interrupt occurred"]
    Set = 1,
    #[doc = "0: Interrupt did not occur"]
    Clr = 0,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - 0:0\\]
NoWrapper operation completed. This interrupt bit is set by firmware or the corresponding bit in the ISET register. It is cleared by the corresponding bit in in the ICLR register or reading the IIDX register when this interrupt is the highest priority."]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            true => Done::Set,
            false => Done::Clr,
        }
    }
    #[doc = "Interrupt occurred"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Done::Set
    }
    #[doc = "Interrupt did not occur"]
    #[inline(always)]
    pub fn is_clr(&self) -> bool {
        *self == Done::Clr
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
NoWrapper operation completed. This interrupt bit is set by firmware or the corresponding bit in the ISET register. It is cleared by the corresponding bit in in the ICLR register or reading the IIDX register when this interrupt is the highest priority."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Raw Interrupt Status Register: The RIS register reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing a 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled. A flag can be set by software by writing a 1 to the ISET register. Reading the IIDX register will also clear the corresponding bit in RIS. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`write(|w| ..)` method takes [`ris::W`](W) writer structure"]
impl crate::Writable for RisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
