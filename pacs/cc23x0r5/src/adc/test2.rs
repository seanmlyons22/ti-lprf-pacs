#[doc = "Register `TEST2` reader"]
pub type R = crate::R<Test2Spec>;
#[doc = "Register `TEST2` writer"]
pub type W = crate::W<Test2Spec>;
#[doc = "Field `RESERVED0` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `RESERVED0` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MUX_TEST_SEL` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type MuxTestSelR = crate::BitReader;
#[doc = "Field `MUX_TEST_SEL` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type MuxTestSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 19:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED9` writer - 19:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `COMP_GAIN_TRIM` reader - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CompGainTrimR = crate::BitReader;
#[doc = "Field `COMP_GAIN_TRIM` writer - 20:20\\]
Internal. Only to be used through TI provided API."]
pub type CompGainTrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED21` reader - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21R = crate::FieldReader;
#[doc = "Field `RESERVED21` writer - 23:21\\]
Internal. Only to be used through TI provided API."]
pub type Reserved21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LATCH_TRIM_EN` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type LatchTrimEnR = crate::BitReader;
#[doc = "Field `LATCH_TRIM_EN` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type LatchTrimEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `RESERVED25` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CDAC_OVST_EN` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type CdacOvstEnR = crate::BitReader;
#[doc = "Field `CDAC_OVST_EN` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type CdacOvstEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn mux_test_sel(&self) -> MuxTestSelR {
        MuxTestSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:19 - 19:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x07ff) as u16)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn comp_gain_trim(&self) -> CompGainTrimR {
        CompGainTrimR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn latch_trim_en(&self) -> LatchTrimEnR {
        LatchTrimEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cdac_ovst_en(&self) -> CdacOvstEnR {
        CdacOvstEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<Test2Spec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn mux_test_sel(&mut self) -> MuxTestSelW<Test2Spec> {
        MuxTestSelW::new(self, 8)
    }
    #[doc = "Bits 9:19 - 19:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> Reserved9W<Test2Spec> {
        Reserved9W::new(self, 9)
    }
    #[doc = "Bit 20 - 20:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn comp_gain_trim(&mut self) -> CompGainTrimW<Test2Spec> {
        CompGainTrimW::new(self, 20)
    }
    #[doc = "Bits 21:23 - 23:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved21(&mut self) -> Reserved21W<Test2Spec> {
        Reserved21W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn latch_trim_en(&mut self) -> LatchTrimEnW<Test2Spec> {
        LatchTrimEnW::new(self, 24)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> Reserved25W<Test2Spec> {
        Reserved25W::new(self, 25)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cdac_ovst_en(&mut self) -> CdacOvstEnW<Test2Spec> {
        CdacOvstEnW::new(self, 31)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Test2Spec;
impl crate::RegisterSpec for Test2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test2::R`](R) reader structure"]
impl crate::Readable for Test2Spec {}
#[doc = "`write(|w| ..)` method takes [`test2::W`](W) writer structure"]
impl crate::Writable for Test2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST2 to value 0"]
impl crate::Resettable for Test2Spec {
    const RESET_VALUE: u32 = 0;
}
