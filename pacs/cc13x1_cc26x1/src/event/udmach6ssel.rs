#[doc = "Register `UDMACH6SSEL` reader"]
pub type R = crate::R<Udmach6sselSpec>;
#[doc = "Register `UDMACH6SSEL` writer"]
pub type W = crate::W<Udmach6sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 55"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "55: Not used tied to 0"]
    TieLow55 = 55,
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
            55 => Some(Ev::TieLow55),
            _ => None,
        }
    }
    #[doc = "Not used tied to 0"]
    #[inline(always)]
    pub fn is_tie_low55(&self) -> bool {
        *self == Ev::TieLow55
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
#[doc = "Output Selection for DMA Channel 6 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach6ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach6ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach6sselSpec;
impl crate::RegisterSpec for Udmach6sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach6ssel::R`](R) reader structure"]
impl crate::Readable for Udmach6sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach6ssel::W`](W) writer structure"]
impl crate::Writable for Udmach6sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH6SSEL to value 0x37"]
impl crate::Resettable for Udmach6sselSpec {
    const RESET_VALUE: u32 = 0x37;
}
