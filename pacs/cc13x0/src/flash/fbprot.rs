#[doc = "Register `FBPROT` reader"]
pub type R = crate::R<FbprotSpec>;
#[doc = "Register `FBPROT` writer"]
pub type W = crate::W<FbprotSpec>;
#[doc = "Field `PROTL1DIS` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Protl1disR = crate::BitReader;
#[doc = "Field `PROTL1DIS` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Protl1disW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED1` writer - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn protl1dis(&self) -> Protl1disR {
        Protl1disR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn protl1dis(&mut self) -> Protl1disW<FbprotSpec> {
        Protl1disW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<FbprotSpec> {
        Reserved1W::new(self, 1)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbprot::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbprot::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbprotSpec;
impl crate::RegisterSpec for FbprotSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbprot::R`](R) reader structure"]
impl crate::Readable for FbprotSpec {}
#[doc = "`write(|w| ..)` method takes [`fbprot::W`](W) writer structure"]
impl crate::Writable for FbprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBPROT to value 0"]
impl crate::Resettable for FbprotSpec {
    const RESET_VALUE: u32 = 0;
}
