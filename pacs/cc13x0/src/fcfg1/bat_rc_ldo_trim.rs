#[doc = "Register `BAT_RC_LDO_TRIM` reader"]
pub type R = crate::R<BatRcLdoTrimSpec>;
#[doc = "Register `BAT_RC_LDO_TRIM` writer"]
pub type W = crate::W<BatRcLdoTrimSpec>;
#[doc = "Field `MEASUREPER` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MeasureperR = crate::FieldReader;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `RCOSCHF_ITUNE_TRIM` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfItuneTrimR = crate::FieldReader;
#[doc = "Field `RESERVED2` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `VTRIM_UDIG` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VtrimUdigR = crate::FieldReader;
#[doc = "Field `RESERVED3` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `VTRIM_BOD` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type VtrimBodR = crate::FieldReader;
#[doc = "Field `RESERVED4` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type Reserved4R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn measureper(&self) -> MeasureperR {
        MeasureperR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_itune_trim(&self) -> RcoschfItuneTrimR {
        RcoschfItuneTrimR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_udig(&self) -> VtrimUdigR {
        VtrimUdigR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_bod(&self) -> VtrimBodR {
        VtrimBodR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bat_rc_ldo_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bat_rc_ldo_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BatRcLdoTrimSpec;
impl crate::RegisterSpec for BatRcLdoTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bat_rc_ldo_trim::R`](R) reader structure"]
impl crate::Readable for BatRcLdoTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`bat_rc_ldo_trim::W`](W) writer structure"]
impl crate::Writable for BatRcLdoTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAT_RC_LDO_TRIM to value 0xf0f0_f0fc"]
impl crate::Resettable for BatRcLdoTrimSpec {
    const RESET_VALUE: u32 = 0xf0f0_f0fc;
}
