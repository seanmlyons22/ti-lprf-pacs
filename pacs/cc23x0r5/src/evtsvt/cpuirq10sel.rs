#[doc = "Register `CPUIRQ10SEL` reader"]
pub type R = crate::R<Cpuirq10selSpec>;
#[doc = "Register `CPUIRQ10SEL` writer"]
pub type W = crate::W<Cpuirq10selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "15: SPI0 combined interrupt request, interrupt flags can be found here SPI0:MIS"]
    Spi0Comb = 15,
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
            15 => Some(Pubid::Spi0Comb),
            _ => None,
        }
    }
    #[doc = "SPI0 combined interrupt request, interrupt flags can be found here SPI0:MIS"]
    #[inline(always)]
    pub fn is_spi0_comb(&self) -> bool {
        *self == Pubid::Spi0Comb
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
#[doc = "Output Selection for CPU Interrupt CPUIRQ10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq10sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq10sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirq10selSpec;
impl crate::RegisterSpec for Cpuirq10selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirq10sel::R`](R) reader structure"]
impl crate::Readable for Cpuirq10selSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuirq10sel::W`](W) writer structure"]
impl crate::Writable for Cpuirq10selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQ10SEL to value 0x0f"]
impl crate::Resettable for Cpuirq10selSpec {
    const RESET_VALUE: u32 = 0x0f;
}
