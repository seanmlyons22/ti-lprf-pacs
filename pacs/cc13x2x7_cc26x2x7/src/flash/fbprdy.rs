#[doc = "Register `FBPRDY` reader"]
pub type R = crate::R<FbprdySpec>;
#[doc = "Register `FBPRDY` writer"]
pub type W = crate::W<FbprdySpec>;
#[doc = "Field `BANKRDY` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type BankrdyR = crate::FieldReader;
#[doc = "Field `RESERVED3` reader - 14:2\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u16>;
#[doc = "Field `PUMPRDY` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type PumprdyR = crate::BitReader;
#[doc = "Field `BANKBUSY` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type BankbusyR = crate::FieldReader;
#[doc = "Field `RESERVED19` reader - 31:18\\]
Internal. Only to be used through TI provided API."]
pub type Reserved19R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&self) -> BankrdyR {
        BankrdyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:14 - 14:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 0x1fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&self) -> PumprdyR {
        PumprdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&self) -> BankbusyR {
        BankbusyR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31 - 31:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbprdy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbprdy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbprdySpec;
impl crate::RegisterSpec for FbprdySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbprdy::R`](R) reader structure"]
impl crate::Readable for FbprdySpec {}
#[doc = "`write(|w| ..)` method takes [`fbprdy::W`](W) writer structure"]
impl crate::Writable for FbprdySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBPRDY to value 0x00ff_00fc"]
impl crate::Resettable for FbprdySpec {
    const RESET_VALUE: u32 = 0x00ff_00fc;
}
