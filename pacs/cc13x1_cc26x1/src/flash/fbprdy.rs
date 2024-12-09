#[doc = "Register `FBPRDY` reader"]
pub type R = crate::R<FbprdySpec>;
#[doc = "Register `FBPRDY` writer"]
pub type W = crate::W<FbprdySpec>;
#[doc = "Field `BANKRDY` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type BankrdyR = crate::BitReader;
#[doc = "Field `RESERVED1` reader - 14:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `PUMPRDY` reader - 15:15\\]
Internal. Only to be used through TI provided API."]
pub type PumprdyR = crate::BitReader;
#[doc = "Field `BANKBUSY` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type BankbusyR = crate::BitReader;
#[doc = "Field `RESERVED17` reader - 31:17\\]
Internal. Only to be used through TI provided API."]
pub type Reserved17R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankrdy(&self) -> BankrdyR {
        BankrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - 14:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bit 15 - 15:15\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumprdy(&self) -> PumprdyR {
        PumprdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bankbusy(&self) -> BankbusyR {
        BankbusyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 17) & 0x7fff) as u16)
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
#[doc = "`reset()` method sets FBPRDY to value 0x00ff_00fe"]
impl crate::Resettable for FbprdySpec {
    const RESET_VALUE: u32 = 0x00ff_00fe;
}
