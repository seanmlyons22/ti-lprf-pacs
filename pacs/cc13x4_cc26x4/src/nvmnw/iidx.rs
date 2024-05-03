#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Register `IIDX` writer"]
pub type W = crate::W<IidxSpec>;
#[doc = "0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stat {
    #[doc = "1: DONE Interrupt Pending"]
    Done = 1,
    #[doc = "0: No Interrupt Pending"]
    NoIntr = 0,
}
impl From<Stat> for bool {
    #[inline(always)]
    fn from(variant: Stat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT` reader - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
pub type StatR = crate::BitReader<Stat>;
impl StatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stat {
        match self.bits {
            true => Stat::Done,
            false => Stat::NoIntr,
        }
    }
    #[doc = "DONE Interrupt Pending"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == Stat::Done
    }
    #[doc = "No Interrupt Pending"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == Stat::NoIntr
    }
}
#[doc = "Field `STAT` writer - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
pub type StatW<'a, REG> = crate::BitWriter<'a, REG, Stat>;
impl<'a, REG> StatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DONE Interrupt Pending"]
    #[inline(always)]
    pub fn done(self) -> &'a mut crate::W<REG> {
        self.variant(Stat::Done)
    }
    #[doc = "No Interrupt Pending"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut crate::W<REG> {
        self.variant(Stat::NoIntr)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
    #[inline(always)]
    pub fn stat(&self) -> StatR {
        StatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Indicates which interrupt has fired. 0x0 means no event pending. The priority order is fixed. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flags in the RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register must be updated with the next highest priority interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> StatW<IidxSpec> {
        StatW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<IidxSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Interrupt Index Register: The IIDX register provides the highest priority enabled interrupt index. PSD compliant register. Note that it is not recommended to use this register if the system clock is running at a slower clock frequency than the NoWrapper clock. If this is the case, then reading this register may fail to update the RIS register correctly. The MIS register should be read directly, and a write to ICLR should be used to clear interrupts when this clock relationship is present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iidx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iidx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`write(|w| ..)` method takes [`iidx::W`](W) writer structure"]
impl crate::Writable for IidxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {
    const RESET_VALUE: u32 = 0;
}
