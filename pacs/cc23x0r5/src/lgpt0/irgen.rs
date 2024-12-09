#[doc = "Register `IRGEN` reader"]
pub type R = crate::R<IrgenSpec>;
#[doc = "Register `IRGEN` writer"]
pub type W = crate::W<IrgenSpec>;
#[doc = "0:0\\]
Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl {
    #[doc = "1: Enable."]
    En = 1,
    #[doc = "0: Disable."]
    Dis = 0,
}
impl From<Ctl> for bool {
    #[inline(always)]
    fn from(variant: Ctl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL` reader - 0:0\\]
Control"]
pub type CtlR = crate::BitReader<Ctl>;
impl CtlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl {
        match self.bits {
            true => Ctl::En,
            false => Ctl::Dis,
        }
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Ctl::En
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Ctl::Dis
    }
}
#[doc = "Field `CTL` writer - 0:0\\]
Control"]
pub type CtlW<'a, REG> = crate::BitWriter<'a, REG, Ctl>;
impl<'a, REG> CtlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::En)
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl::Dis)
    }
}
#[doc = "Field `RESERVED1` reader - 31:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Control"]
    #[inline(always)]
    pub fn ctl(&self) -> CtlR {
        CtlR::new((self.bits & 1) != 0)
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
Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CtlW<IrgenSpec> {
        CtlW::new(self, 0)
    }
}
#[doc = "IR Generation Use this register to generate IR codes. When CTL = 1, an AND gate is enabled between IO output 0 in LGPT0 and IC output 0 in LGPT1. The output of the gate overrides IO output 0 in LGPT0. See OUTCTL for explanation of outputs. To generate IR codes let LGPT0 generate the carrier wave on output 0. Set this output as tick input of LGPT1, with PRECFG.TICKSRC = FALL_TICK. Generate wanted IR codes by adjusting LGPT1 PTGT and PC0CC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irgen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irgen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrgenSpec;
impl crate::RegisterSpec for IrgenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irgen::R`](R) reader structure"]
impl crate::Readable for IrgenSpec {}
#[doc = "`write(|w| ..)` method takes [`irgen::W`](W) writer structure"]
impl crate::Writable for IrgenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IRGEN to value 0"]
impl crate::Resettable for IrgenSpec {
    const RESET_VALUE: u32 = 0;
}
