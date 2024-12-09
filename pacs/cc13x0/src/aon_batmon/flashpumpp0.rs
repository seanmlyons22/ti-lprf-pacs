#[doc = "Register `FLASHPUMPP0` reader"]
pub type R = crate::R<Flashpumpp0Spec>;
#[doc = "Register `FLASHPUMPP0` writer"]
pub type W = crate::W<Flashpumpp0Spec>;
#[doc = "Field `CFG` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgR = crate::FieldReader;
#[doc = "Field `CFG` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OVR` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOWLIM` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type LowlimR = crate::BitReader;
#[doc = "Field `LOWLIM` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type LowlimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIGHLIM` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type HighlimR = crate::FieldReader;
#[doc = "Field `HIGHLIM` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type HighlimW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FALLB` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FallbR = crate::BitReader;
#[doc = "Field `FALLB` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type FallbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 31:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lowlim(&self) -> LowlimR {
        LowlimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn highlim(&self) -> HighlimR {
        HighlimR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fallb(&self) -> FallbR {
        FallbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<Flashpumpp0Spec> {
        CfgW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OvrW<Flashpumpp0Spec> {
        OvrW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lowlim(&mut self) -> LowlimW<Flashpumpp0Spec> {
        LowlimW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn highlim(&mut self) -> HighlimW<Flashpumpp0Spec> {
        HighlimW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn fallb(&mut self) -> FallbW<Flashpumpp0Spec> {
        FallbW::new(self, 8)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashpumpp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashpumpp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Flashpumpp0Spec;
impl crate::RegisterSpec for Flashpumpp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashpumpp0::R`](R) reader structure"]
impl crate::Readable for Flashpumpp0Spec {}
#[doc = "`write(|w| ..)` method takes [`flashpumpp0::W`](W) writer structure"]
impl crate::Writable for Flashpumpp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASHPUMPP0 to value 0"]
impl crate::Resettable for Flashpumpp0Spec {
    const RESET_VALUE: u32 = 0;
}
