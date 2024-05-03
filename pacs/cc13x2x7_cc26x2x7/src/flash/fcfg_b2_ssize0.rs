#[doc = "Register `FCFG_B2_SSIZE0` reader"]
pub type R = crate::R<FcfgB2Ssize0Spec>;
#[doc = "Register `FCFG_B2_SSIZE0` writer"]
pub type W = crate::W<FcfgB2Ssize0Spec>;
#[doc = "Field `B0_SECT_SIZE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0SectSizeR = crate::FieldReader;
#[doc = "Field `B0_SECT_SIZE` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0SectSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED4` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED4` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `B0_NUM_SECTORS` reader - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type B0NumSectorsR = crate::FieldReader<u16>;
#[doc = "Field `B0_NUM_SECTORS` writer - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type B0NumSectorsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28R = crate::FieldReader;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved28W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_sect_size(&self) -> B0SectSizeR {
        B0SectSizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_num_sectors(&self) -> B0NumSectorsR {
        B0NumSectorsR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> Reserved28R {
        Reserved28R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_sect_size(&mut self) -> B0SectSizeW<FcfgB2Ssize0Spec> {
        B0SectSizeW::new(self, 0)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> Reserved4W<FcfgB2Ssize0Spec> {
        Reserved4W::new(self, 4)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b0_num_sectors(&mut self) -> B0NumSectorsW<FcfgB2Ssize0Spec> {
        B0NumSectorsW::new(self, 16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> Reserved28W<FcfgB2Ssize0Spec> {
        Reserved28W::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_b2_ssize0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_b2_ssize0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgB2Ssize0Spec;
impl crate::RegisterSpec for FcfgB2Ssize0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_b2_ssize0::R`](R) reader structure"]
impl crate::Readable for FcfgB2Ssize0Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_b2_ssize0::W`](W) writer structure"]
impl crate::Writable for FcfgB2Ssize0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_B2_SSIZE0 to value 0"]
impl crate::Resettable for FcfgB2Ssize0Spec {
    const RESET_VALUE: u32 = 0;
}
