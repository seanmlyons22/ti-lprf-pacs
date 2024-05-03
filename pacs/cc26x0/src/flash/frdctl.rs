#[doc = "Register `FRDCTL` reader"]
pub type R = crate::R<FrdctlSpec>;
#[doc = "Register `FRDCTL` writer"]
pub type W = crate::W<FrdctlSpec>;
#[doc = "Field `RM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RmR = crate::FieldReader;
#[doc = "Field `RM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RWAIT` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RwaitR = crate::FieldReader;
#[doc = "Field `RWAIT` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RwaitW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type Reserved12W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait(&self) -> RwaitR {
        RwaitR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RmW<FrdctlSpec> {
        RmW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait(&mut self) -> RwaitW<FrdctlSpec> {
        RwaitW::new(self, 8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> Reserved12W<FrdctlSpec> {
        Reserved12W::new(self, 12)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frdctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frdctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrdctlSpec;
impl crate::RegisterSpec for FrdctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frdctl::R`](R) reader structure"]
impl crate::Readable for FrdctlSpec {}
#[doc = "`write(|w| ..)` method takes [`frdctl::W`](W) writer structure"]
impl crate::Writable for FrdctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRDCTL to value 0x0200"]
impl crate::Resettable for FrdctlSpec {
    const RESET_VALUE: u32 = 0x0200;
}
