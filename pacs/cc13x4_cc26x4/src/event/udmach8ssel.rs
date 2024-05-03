#[doc = "Register `UDMACH8SSEL` reader"]
pub type R = crate::R<Udmach8sselSpec>;
#[doc = "Register `UDMACH8SSEL` writer"]
pub type W = crate::W<Udmach8sselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 116"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "116: DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    AuxSwDmabreq = 116,
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
            116 => Some(Ev::AuxSwDmabreq),
            _ => None,
        }
    }
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn is_aux_sw_dmabreq(&self) -> bool {
        *self == Ev::AuxSwDmabreq
    }
}
#[doc = "Field `EV` writer - 7:0\\]
Read only selection value"]
pub type EvW<'a, REG> = crate::FieldWriter<'a, REG, 8, Ev>;
impl<'a, REG> EvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA sofware trigger from AUX, triggered by AUX_EVCTL:DMASWREQ.START"]
    #[inline(always)]
    pub fn aux_sw_dmabreq(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::AuxSwDmabreq)
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
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Read only selection value"]
    #[inline(always)]
    #[must_use]
    pub fn ev(&mut self) -> EvW<Udmach8sselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach8ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach8ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach8sselSpec;
impl crate::RegisterSpec for Udmach8sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach8ssel::R`](R) reader structure"]
impl crate::Readable for Udmach8sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach8ssel::W`](W) writer structure"]
impl crate::Writable for Udmach8sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH8SSEL to value 0x74"]
impl crate::Resettable for Udmach8sselSpec {
    const RESET_VALUE: u32 = 0x74;
}
