#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "0:0\\]
RTC counter reset. Writing 1 to this bit will reset the RTC counter, and cause it to resume counting from 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rst {
    #[doc = "1: Reset the timer."]
    Clr = 1,
    #[doc = "0: No effect"]
    Noeff = 0,
}
impl From<Rst> for bool {
    #[inline(always)]
    fn from(variant: Rst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RST` writer - 0:0\\]
RTC counter reset. Writing 1 to this bit will reset the RTC counter, and cause it to resume counting from 0x0"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG, Rst>;
impl<'a, REG> RstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the timer."]
    #[inline(always)]
    pub fn clr(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Clr)
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn noeff(self) -> &'a mut crate::W<REG> {
        self.variant(Rst::Noeff)
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
RTC counter reset. Writing 1 to this bit will reset the RTC counter, and cause it to resume counting from 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<CtlSpec> {
        RstW::new(self, 0)
    }
}
#[doc = "RTC Control register. This register controls resetting the of RTC counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
