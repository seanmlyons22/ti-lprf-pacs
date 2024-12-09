#[doc = "Register `INFRCLKDIVR` reader"]
pub type R = crate::R<InfrclkdivrSpec>;
#[doc = "Register `INFRCLKDIVR` writer"]
pub type W = crate::W<InfrclkdivrSpec>;
#[doc = "1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ratio {
    #[doc = "3: Divide by 32"]
    Div32 = 3,
    #[doc = "2: Divide by 8"]
    Div8 = 2,
    #[doc = "1: Divide by 2"]
    Div2 = 1,
    #[doc = "0: Divide by 1"]
    Div1 = 0,
}
impl From<Ratio> for u8 {
    #[inline(always)]
    fn from(variant: Ratio) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ratio {
    type Ux = u8;
}
impl crate::IsEnum for Ratio {}
#[doc = "Field `RATIO` reader - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub type RatioR = crate::FieldReader<Ratio>;
impl RatioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ratio {
        match self.bits {
            3 => Ratio::Div32,
            2 => Ratio::Div8,
            1 => Ratio::Div2,
            0 => Ratio::Div1,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Ratio::Div32
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Ratio::Div8
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Ratio::Div2
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Ratio::Div1
    }
}
#[doc = "Field `RATIO` writer - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
pub type RatioW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ratio, crate::Safe>;
impl<'a, REG> RatioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div32)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div8)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div2)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Ratio::Div1)
    }
}
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    pub fn ratio(&self) -> RatioR {
        RatioR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Division rate for clocks driving modules in the MCU_AON domain when system CPU is in run mode. Division ratio affects both infrastructure clock and perbusull clock."]
    #[inline(always)]
    #[must_use]
    pub fn ratio(&mut self) -> RatioW<InfrclkdivrSpec> {
        RatioW::new(self, 0)
    }
}
#[doc = "Infrastructure Clock Division Factor For Run Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infrclkdivr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infrclkdivr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfrclkdivrSpec;
impl crate::RegisterSpec for InfrclkdivrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`infrclkdivr::R`](R) reader structure"]
impl crate::Readable for InfrclkdivrSpec {}
#[doc = "`write(|w| ..)` method takes [`infrclkdivr::W`](W) writer structure"]
impl crate::Writable for InfrclkdivrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFRCLKDIVR to value 0"]
impl crate::Resettable for InfrclkdivrSpec {
    const RESET_VALUE: u32 = 0;
}
