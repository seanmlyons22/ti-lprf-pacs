#[doc = "Register `CPUIRQSEL33` reader"]
pub type R = crate::R<Cpuirqsel33Spec>;
#[doc = "Register `CPUIRQSEL33` writer"]
pub type W = crate::W<Cpuirqsel33Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 104"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "104: TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    TrngIrq = 104,
}
impl From<Ev> for u8 {
    #[inline(always)]
    fn from(variant: Ev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ev {
    type Ux = u8;
}
impl crate::IsEnum for Ev {}
#[doc = "Field `EV` reader - 7:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            104 => Some(Ev::TrngIrq),
            _ => None,
        }
    }
    #[doc = "TRNG Interrupt event, controlled by TRNG:IRQEN.EN"]
    #[inline(always)]
    pub fn is_trng_irq(&self) -> bool {
        *self == Ev::TrngIrq
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Output Selection for CPU Interrupt 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpuirqsel33Spec;
impl crate::RegisterSpec for Cpuirqsel33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuirqsel33::R`](R) reader structure"]
impl crate::Readable for Cpuirqsel33Spec {}
#[doc = "`write(|w| ..)` method takes [`cpuirqsel33::W`](W) writer structure"]
impl crate::Writable for Cpuirqsel33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUIRQSEL33 to value 0x68"]
impl crate::Resettable for Cpuirqsel33Spec {
    const RESET_VALUE: u32 = 0x68;
}
