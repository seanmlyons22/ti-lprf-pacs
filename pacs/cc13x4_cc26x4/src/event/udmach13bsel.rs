#[doc = "Register `UDMACH13BSEL` reader"]
pub type R = crate::R<Udmach13bselSpec>;
#[doc = "Register `UDMACH13BSEL` writer"]
pub type W = crate::W<Udmach13bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "3: AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    AonProg2 = 3,
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
            3 => Some(Ev::AonProg2),
            _ => None,
        }
    }
    #[doc = "AON programmable event 2. Event selected by AON_EVENT MCU event selector, AON_EVENT:EVTOMCUSEL.AON_PROG2_EV"]
    #[inline(always)]
    pub fn is_aon_prog2(&self) -> bool {
        *self == Ev::AonProg2
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
#[doc = "Output Selection for DMA Channel 13 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach13bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach13bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach13bselSpec;
impl crate::RegisterSpec for Udmach13bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach13bsel::R`](R) reader structure"]
impl crate::Readable for Udmach13bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach13bsel::W`](W) writer structure"]
impl crate::Writable for Udmach13bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH13BSEL to value 0x03"]
impl crate::Resettable for Udmach13bselSpec {
    const RESET_VALUE: u32 = 0x03;
}
