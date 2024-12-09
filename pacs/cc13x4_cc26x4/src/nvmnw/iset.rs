#[doc = "Register `ISET` reader"]
pub type R = crate::R<IsetSpec>;
#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "0:0\\]
0: No effect 1: Set the DONE interrupt in the RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "1: Set IPSTANDARD.RIS bit"]
    Set = 1,
    #[doc = "0: Writing a 0 has no effect"]
    NoEffect = 0,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` writer - 0:0\\]
0: No effect 1: Set the DONE interrupt in the RIS register"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set IPSTANDARD.RIS bit"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Set)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Done::NoEffect)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No effect 1: Set the DONE interrupt in the RIS register"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IsetSpec> {
        DoneW::new(self, 0)
    }
}
#[doc = "Interrupt Set Register: The ISET register allows software to write a 1 to set corresponding interrupt. Safety: This meets a safety requirement to allow software diagnostics to trigger interrupts. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iset::R`](R) reader structure"]
impl crate::Readable for IsetSpec {}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
