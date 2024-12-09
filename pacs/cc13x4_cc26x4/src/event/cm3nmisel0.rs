#[doc = "Register `CM3NMISEL0` reader"]
pub type R = crate::R<Cm3nmisel0Spec>;
#[doc = "Register `CM3NMISEL0` writer"]
pub type W = crate::W<Cm3nmisel0Spec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 99"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "99: Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    WdtNmi = 99,
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
            99 => Some(Ev::WdtNmi),
            _ => None,
        }
    }
    #[doc = "Watchdog non maskable interrupt event, controlled by WDT:CTL.INTTYPE"]
    #[inline(always)]
    pub fn is_wdt_nmi(&self) -> bool {
        *self == Ev::WdtNmi
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
#[doc = "Output Selection for NMI Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm3nmisel0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm3nmisel0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm3nmisel0Spec;
impl crate::RegisterSpec for Cm3nmisel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm3nmisel0::R`](R) reader structure"]
impl crate::Readable for Cm3nmisel0Spec {}
#[doc = "`write(|w| ..)` method takes [`cm3nmisel0::W`](W) writer structure"]
impl crate::Writable for Cm3nmisel0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM3NMISEL0 to value 0x63"]
impl crate::Resettable for Cm3nmisel0Spec {
    const RESET_VALUE: u32 = 0x63;
}
