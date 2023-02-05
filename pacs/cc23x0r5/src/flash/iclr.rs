#[doc = "Register `ICLR` reader"]
pub type R = crate::R<IclrSpec>;
#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "1: Clear IPSTANDARD.RIS bit"]
    Clr = 1,
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
0: No effect 1: Clear the DONE interrupt in the RIS register"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear IPSTANDARD.RIS bit"]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Done::Clr)
    }
    #[doc = "Writing a 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Done::NoEffect)
    }
}
#[doc = "Field `RESERVED_31_1` writer - 31:1\\]
Reserved"]
pub type Reserved31_1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: No effect 1: Clear the DONE interrupt in the RIS register"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<IclrSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved_31_1(&mut self) -> Reserved31_1W<IclrSpec> {
        Reserved31_1W::new(self, 1)
    }
}
#[doc = "Interrupt Clear Register. The ICLR register allows allows software to write a 1 to clear corresponding interrupt. PSD compliant register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iclr::R`](R) reader structure"]
impl crate::Readable for IclrSpec {}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
