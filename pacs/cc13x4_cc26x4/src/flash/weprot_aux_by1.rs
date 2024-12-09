#[doc = "Register `WEPROT_AUX_BY1` reader"]
pub type R = crate::R<WeprotAuxBy1Spec>;
#[doc = "Register `WEPROT_AUX_BY1` writer"]
pub type W = crate::W<WeprotAuxBy1Spec>;
#[doc = "Field `WEPROT_B0_CCFG_BY1` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0CcfgBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B0_CCFG_BY1` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0CcfgBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPROT_B1_FCFG_BY1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1FcfgBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B1_FCFG_BY1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1FcfgBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPROT_B0_TRIM_BY1` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0TrimBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B0_TRIM_BY1` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0TrimBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPROT_B1_TRIM_BY1` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1TrimBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B1_TRIM_BY1` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1TrimBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPROT_B0_ENGR_BY1` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0EngrBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B0_ENGR_BY1` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB0EngrBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEPROT_B1_ENGR_BY1` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1EngrBy1R = crate::BitReader;
#[doc = "Field `WEPROT_B1_ENGR_BY1` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type WeprotB1EngrBy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type Reserved6R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_ccfg_by1(&self) -> WeprotB0CcfgBy1R {
        WeprotB0CcfgBy1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_fcfg_by1(&self) -> WeprotB1FcfgBy1R {
        WeprotB1FcfgBy1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_trim_by1(&self) -> WeprotB0TrimBy1R {
        WeprotB0TrimBy1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_trim_by1(&self) -> WeprotB1TrimBy1R {
        WeprotB1TrimBy1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_engr_by1(&self) -> WeprotB0EngrBy1R {
        WeprotB0EngrBy1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_engr_by1(&self) -> WeprotB1EngrBy1R {
        WeprotB1EngrBy1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_ccfg_by1(&mut self) -> WeprotB0CcfgBy1W<WeprotAuxBy1Spec> {
        WeprotB0CcfgBy1W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_fcfg_by1(&mut self) -> WeprotB1FcfgBy1W<WeprotAuxBy1Spec> {
        WeprotB1FcfgBy1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_trim_by1(&mut self) -> WeprotB0TrimBy1W<WeprotAuxBy1Spec> {
        WeprotB0TrimBy1W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_trim_by1(&mut self) -> WeprotB1TrimBy1W<WeprotAuxBy1Spec> {
        WeprotB1TrimBy1W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_engr_by1(&mut self) -> WeprotB0EngrBy1W<WeprotAuxBy1Spec> {
        WeprotB0EngrBy1W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_engr_by1(&mut self) -> WeprotB1EngrBy1W<WeprotAuxBy1Spec> {
        WeprotB1EngrBy1W::new(self, 5)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprot_aux_by1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprot_aux_by1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WeprotAuxBy1Spec;
impl crate::RegisterSpec for WeprotAuxBy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weprot_aux_by1::R`](R) reader structure"]
impl crate::Readable for WeprotAuxBy1Spec {}
#[doc = "`write(|w| ..)` method takes [`weprot_aux_by1::W`](W) writer structure"]
impl crate::Writable for WeprotAuxBy1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEPROT_AUX_BY1 to value 0x3f"]
impl crate::Resettable for WeprotAuxBy1Spec {
    const RESET_VALUE: u32 = 0x3f;
}
