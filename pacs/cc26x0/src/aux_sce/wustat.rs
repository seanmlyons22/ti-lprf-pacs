#[doc = "Register `WUSTAT` reader"]
pub type R = crate::R<WustatSpec>;
#[doc = "Register `WUSTAT` writer"]
pub type W = crate::W<WustatSpec>;
#[doc = "Field `EV_SIGNALS` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvSignalsR = crate::FieldReader;
#[doc = "Field `EV_SIGNALS` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type EvSignalsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WU_SIGNAL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WuSignalR = crate::BitReader;
#[doc = "Field `WU_SIGNAL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type WuSignalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EXC_VECTOR` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type ExcVectorR = crate::FieldReader;
#[doc = "Field `EXC_VECTOR` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type ExcVectorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED18` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved18R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED18` writer - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved18W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ev_signals(&self) -> EvSignalsR {
        EvSignalsR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wu_signal(&self) -> WuSignalR {
        WuSignalR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn exc_vector(&self) -> ExcVectorR {
        ExcVectorR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ev_signals(&mut self) -> EvSignalsW<WustatSpec> {
        EvSignalsW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wu_signal(&mut self) -> WuSignalW<WustatSpec> {
        WuSignalW::new(self, 8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<WustatSpec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn exc_vector(&mut self) -> ExcVectorW<WustatSpec> {
        ExcVectorW::new(self, 16)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved18(&mut self) -> Reserved18W<WustatSpec> {
        Reserved18W::new(self, 18)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wustat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wustat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WustatSpec;
impl crate::RegisterSpec for WustatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wustat::R`](R) reader structure"]
impl crate::Readable for WustatSpec {}
#[doc = "`write(|w| ..)` method takes [`wustat::W`](W) writer structure"]
impl crate::Writable for WustatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WUSTAT to value 0"]
impl crate::Resettable for WustatSpec {
    const RESET_VALUE: u32 = 0;
}
