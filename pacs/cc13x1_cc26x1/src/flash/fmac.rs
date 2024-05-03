#[doc = "Register `FMAC` reader"]
pub type R = crate::R<FmacSpec>;
#[doc = "Register `FMAC` writer"]
pub type W = crate::W<FmacSpec>;
#[doc = "Field `BANK` reader - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type BankR = crate::FieldReader;
#[doc = "Field `BANK` writer - 2:0\\]
Internal. Only to be used through TI provided API."]
pub type BankW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESERVED3` reader - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:3\\]
Internal. Only to be used through TI provided API."]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bank(&self) -> BankR {
        BankR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bank(&mut self) -> BankW<FmacSpec> {
        BankW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<FmacSpec> {
        Reserved3W::new(self, 3)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmac::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmac::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmacSpec;
impl crate::RegisterSpec for FmacSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmac::R`](R) reader structure"]
impl crate::Readable for FmacSpec {}
#[doc = "`write(|w| ..)` method takes [`fmac::W`](W) writer structure"]
impl crate::Writable for FmacSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMAC to value 0"]
impl crate::Resettable for FmacSpec {
    const RESET_VALUE: u32 = 0;
}
