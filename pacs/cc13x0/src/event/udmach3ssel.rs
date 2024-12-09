#[doc = "Register `UDMACH3SSEL` reader"]
pub type R = crate::R<Udmach3sselSpec>;
#[doc = "Register `UDMACH3SSEL` writer"]
pub type W = crate::W<Udmach3sselSpec>;
#[doc = "6:0\\]
Read only selection value\n\nValue on reset: 41"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "41: SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    Ssi0RxDmasreq = 41,
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
            41 => Some(Ev::Ssi0RxDmasreq),
            _ => None,
        }
    }
    #[doc = "SSI0 RX DMA single request, controlled by SSI0:DMACR.RXDMAE"]
    #[inline(always)]
    pub fn is_ssi0_rx_dmasreq(&self) -> bool {
        *self == Ev::Ssi0RxDmasreq
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
#[doc = "Output Selection for DMA Channel 3 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach3ssel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach3ssel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach3sselSpec;
impl crate::RegisterSpec for Udmach3sselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach3ssel::R`](R) reader structure"]
impl crate::Readable for Udmach3sselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach3ssel::W`](W) writer structure"]
impl crate::Writable for Udmach3sselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH3SSEL to value 0x29"]
impl crate::Resettable for Udmach3sselSpec {
    const RESET_VALUE: u32 = 0x29;
}
