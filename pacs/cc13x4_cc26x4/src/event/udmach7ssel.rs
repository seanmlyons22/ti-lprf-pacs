#[doc = "Register `UDMACH7SSEL` reader"]
pub type R = crate::R<Udmach7sselSpec>;
#[doc = "Register `UDMACH7SSEL` writer"]
pub type W = crate::W<Udmach7sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 117"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "117: DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    AuxDmasreq = 117,
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
            117 => Some(Ev::AuxDmasreq),
            _ => None,
        }
    }
    #[doc = "DMA single request event from AUX, configured by AUX_EVCTL:DMACTL"]
    #[inline(always)]
    pub fn is_aux_dmasreq(&self) -> bool {
        *self == Ev::AuxDmasreq
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
#[doc = "Output Selection for DMA Channel 7 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach7ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach7ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach7sselSpec;
impl crate::RegisterSpec for Udmach7sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach7ssel::R`](R) reader structure"]
impl crate::Readable for Udmach7sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach7ssel::W`](W) writer structure"]
impl crate::Writable for Udmach7sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH7SSEL to value 0x75"]
impl crate::Resettable for Udmach7sselSpec {
    const RESET_VALUE: u32 = 0x75;
}
