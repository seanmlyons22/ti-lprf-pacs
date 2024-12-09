#[doc = "Register `FCFG_BNK_TYPE` reader"]
pub type R = crate::R<FcfgBnkTypeSpec>;
#[doc = "Register `FCFG_BNK_TYPE` writer"]
pub type W = crate::W<FcfgBnkTypeSpec>;
#[doc = "Field `B0_TYPE` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type B0TypeR = crate::FieldReader;
#[doc = "Field `B1_TYPE` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type B1TypeR = crate::FieldReader;
#[doc = "Field `B2_TYPE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type B2TypeR = crate::FieldReader;
#[doc = "Field `B3_TYPE` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type B3TypeR = crate::FieldReader;
#[doc = "Field `B4_TYPE` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type B4TypeR = crate::FieldReader;
#[doc = "Field `B5_TYPE` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type B5TypeR = crate::FieldReader;
#[doc = "Field `B6_TYPE` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B6TypeR = crate::FieldReader;
#[doc = "Field `B7_TYPE` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B7TypeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b0_type(&self) -> B0TypeR {
        B0TypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_type(&self) -> B1TypeR {
        B1TypeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b2_type(&self) -> B2TypeR {
        B2TypeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b3_type(&self) -> B3TypeR {
        B3TypeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b4_type(&self) -> B4TypeR {
        B4TypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b5_type(&self) -> B5TypeR {
        B5TypeR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b6_type(&self) -> B6TypeR {
        B6TypeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b7_type(&self) -> B7TypeR {
        B7TypeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcfg_bnk_type::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcfg_bnk_type::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcfgBnkTypeSpec;
impl crate::RegisterSpec for FcfgBnkTypeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg_bnk_type::R`](R) reader structure"]
impl crate::Readable for FcfgBnkTypeSpec {}
#[doc = "`write(|w| ..)` method takes [`fcfg_bnk_type::W`](W) writer structure"]
impl crate::Writable for FcfgBnkTypeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG_BNK_TYPE to value 0x04"]
impl crate::Resettable for FcfgBnkTypeSpec {
    const RESET_VALUE: u32 = 0x04;
}
