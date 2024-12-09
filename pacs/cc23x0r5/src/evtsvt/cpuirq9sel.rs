#[doc = "Register `CPUIRQ9SEL` reader"]
pub type R = crate::R<Cpuirq9selSpec>;
#[doc = "Register `CPUIRQ9SEL` writer"]
pub type W = crate::W<Cpuirq9selSpec>;
#[doc = "5:0\\]
Read only selection value\n\nValue on reset: 22"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pubid {
    #[doc = "22: AES accelerator combined interrupt request, interrupt flags can be found here AES:MIS"]
    AesComb = 22,
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
            22 => Some(Pubid::AesComb),
            _ => None,
        }
    }
    #[doc = "AES accelerator combined interrupt request, interrupt flags can be found here AES:MIS"]
    #[inline(always)]
    pub fn is_aes_comb(&self) -> bool {
        *self == Pubid::AesComb
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
#[doc = "Output Selection for CPU Interrupt CPUIRQ9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirq9sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirq9sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirq9selSpec;
impl crate::RegisterSpec for Cpuirq9selSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirq9sel::R`](R) reader structure"]
impl crate::Readable for Cpuirq9selSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuirq9sel::W`](W) writer structure"]
impl crate::Writable for Cpuirq9selSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQ9SEL to value 0x16"]
impl crate::Resettable for Cpuirq9selSpec {
    const RESET_VALUE: u32 = 0x16;
}
