#[doc = "Register `CPUIRQ18SEL` reader"]
pub type R = crate::R<Cpuirq18selSpec>;
#[doc = "Register `CPUIRQ18SEL` writer"]
pub type W = crate::W<Cpuirq18selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 55"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "55: LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    Lgpt3Comb = 55,
}
impl From<Pubid> for u8 {
    #[inline(always)]
    fn from(variant: Pubid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pubid {
    type Ux = u8;
}
impl crate::IsEnum for Pubid {}
#[doc = "Field `PUBID` reader - 5:0\\]
Read only selection value"]
pub type PubidR = crate::FieldReader<Pubid>;
impl PubidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pubid> {
        match self.bits {
            55 => Some(Pubid::Lgpt3Comb),
            _ => None,
        }
    }
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn is_lgpt3_comb(&self) -> bool {
        *self == Pubid::Lgpt3Comb
    }
}
#[doc = "Field `PUBID` writer - 5:0\\]
Read only selection value"]
pub type PubidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pubid>;
impl<'a, REG> PubidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LGPT3 combined interrupt, interrupt flags are found here LGPT3:MIS"]
    #[inline(always)]
    pub fn lgpt3_comb(self) -> &'a mut crate::W<REG> {
        self.variant(Pubid::Lgpt3Comb)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn pubid(&self) -> PubidR {
        PubidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn pubid(&mut self) -> PubidW<Cpuirq18selSpec> {
        PubidW::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<Cpuirq18selSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Output Selection for CPU Interrupt CPUIRQ18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq18sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq18sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirq18selSpec;
impl crate::RegisterSpec for Cpuirq18selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirq18sel::R`](R) reader structure"]
impl crate::Readable for Cpuirq18selSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuirq18sel::W`](W) writer structure"]
impl crate::Writable for Cpuirq18selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQ18SEL to value 0x37"]
impl crate::Resettable for Cpuirq18selSpec {
    const RESET_VALUE: u32 = 0x37;
}
