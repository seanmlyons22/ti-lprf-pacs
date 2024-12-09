#[doc = "Register `CPUIRQ17SEL` reader"]
pub type R = crate::R<Cpuirq17selSpec>;
#[doc = "Register `CPUIRQ17SEL` writer"]
pub type W = crate::W<Cpuirq17selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 49"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "49: LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    Lgpt2Comb = 49,
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
            49 => Some(Pubid::Lgpt2Comb),
            _ => None,
        }
    }
    #[doc = "LGPT2 combined interrupt, interrupt flags are found here LGPT2:MIS"]
    #[inline(always)]
    pub fn is_lgpt2_comb(&self) -> bool {
        *self == Pubid::Lgpt2Comb
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior"]
pub type Reserved6R = crate::FieldReader<u32>;
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
impl W {}
#[doc = "Output Selection for CPU Interrupt CPUIRQ17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq17sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq17sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirq17selSpec;
impl crate::RegisterSpec for Cpuirq17selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirq17sel::R`](R) reader structure"]
impl crate::Readable for Cpuirq17selSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuirq17sel::W`](W) writer structure"]
impl crate::Writable for Cpuirq17selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQ17SEL to value 0x31"]
impl crate::Resettable for Cpuirq17selSpec {
    const RESET_VALUE: u32 = 0x31;
}
