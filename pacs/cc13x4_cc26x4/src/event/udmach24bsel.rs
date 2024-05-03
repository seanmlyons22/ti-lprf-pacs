#[doc = "Register `UDMACH24BSEL` reader"]
pub type R = crate::R<Udmach24bselSpec>;
#[doc = "Register `UDMACH24BSEL` writer"]
pub type W = crate::W<Udmach24bselSpec>;
#[doc = "7:0\\]
Read only selection value\n\nValue on reset: 103"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ev {
    #[doc = "103: Software event 3, triggered by SWEV.SWEV3"]
    Swev3 = 103,
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
            103 => Some(Ev::Swev3),
            _ => None,
        }
    }
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn is_swev3(&self) -> bool {
        *self == Ev::Swev3
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
    #[doc = "Software event 3, triggered by SWEV.SWEV3"]
    #[inline(always)]
    pub fn swev3(self) -> &'a mut crate::W<REG> {
        self.variant(Ev::Swev3)
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
    pub fn ev(&mut self) -> EvW<Udmach24bselSpec> {
        EvW::new(self, 0)
    }
}
#[doc = "Output Selection for DMA Channel 24 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach24bsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach24bsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udmach24bselSpec;
impl crate::RegisterSpec for Udmach24bselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udmach24bsel::R`](R) reader structure"]
impl crate::Readable for Udmach24bselSpec {}
#[doc = "`write(|w| ..)` method takes [`udmach24bsel::W`](W) writer structure"]
impl crate::Writable for Udmach24bselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UDMACH24BSEL to value 0x67"]
impl crate::Resettable for Udmach24bselSpec {
    const RESET_VALUE: u32 = 0x67;
}
