#[doc = "Register `AMPCOMP_TH1` reader"]
pub type R = crate::R<AmpcompTh1Spec>;
#[doc = "Register `AMPCOMP_TH1` writer"]
pub type W = crate::W<AmpcompTh1Spec>;
#[doc = "Field `HPMRAMP1_TH` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type Hpmramp1ThR = crate::FieldReader;
#[doc = "Field `IBIASCAP_LPTOHP_OL_CNT` reader - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IbiascapLptohpOlCntR = crate::FieldReader;
#[doc = "Field `HPMRAMP3_HTH` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type Hpmramp3HthR = crate::FieldReader;
#[doc = "Field `RESERVED0` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `HPMRAMP3_LTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type Hpmramp3LthR = crate::FieldReader;
#[doc = "Field `RESERVED1` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&self) -> Hpmramp1ThR {
        Hpmramp1ThR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IbiascapLptohpOlCntR {
        IbiascapLptohpOlCntR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_hth(&self) -> Hpmramp3HthR {
        Hpmramp3HthR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&self) -> Hpmramp3LthR {
        Hpmramp3LthR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcomp_th1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcomp_th1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmpcompTh1Spec;
impl crate::RegisterSpec for AmpcompTh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ampcomp_th1::R`](R) reader structure"]
impl crate::Readable for AmpcompTh1Spec {}
#[doc = "`write(|w| ..)` method takes [`ampcomp_th1::W`](W) writer structure"]
impl crate::Writable for AmpcompTh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMPCOMP_TH1 to value 0xff7b_828e"]
impl crate::Resettable for AmpcompTh1Spec {
    const RESET_VALUE: u32 = 0xff7b_828e;
}
