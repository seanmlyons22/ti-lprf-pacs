#[doc = "Register `UDMACH22SSEL` reader"]
pub type R = crate::R<Udmach22sselSpec>;
#[doc = "Register `UDMACH22SSEL` writer"]
pub type W = crate::W<Udmach22sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 101"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "101: Software event 1, triggered by SWEV.SWEV1"]
    Swev1 = 101,
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
#[doc = "Field `EV` reader - 6:0\\]
Read only selection value"]
pub type EvR = crate::FieldReader<Ev>;
impl EvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ev> {
        match self.bits {
            101 => Some(Ev::Swev1),
            _ => None,
        }
    }
    #[doc = "Software event 1, triggered by SWEV.SWEV1"]
    #[inline(always)]
    pub fn is_swev1(&self) -> bool {
        *self == Ev::Swev1
    }
}
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Read only selection value"]
    #[inline(always)]
    pub fn ev(&self) -> EvR {
        EvR::new((self.bits & 0x7f) as u8)
    }
}
impl W {}
#[doc = "Output Selection for DMA Channel 22 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach22ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach22ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach22sselSpec;
impl crate::RegisterSpec for Udmach22sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach22ssel::R`](R) reader structure"]
impl crate::Readable for Udmach22sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach22ssel::W`](W) writer structure"]
impl crate::Writable for Udmach22sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH22SSEL to value 0x65"]
impl crate::Resettable for Udmach22sselSpec {
    const RESET_VALUE: u32 = 0x65;
}
