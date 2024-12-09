#[doc = "Register `FBBUSY` reader"]
pub type R = crate::R<FbbusySpec>;
#[doc = "Register `FBBUSY` writer"]
pub type W = crate::W<FbbusySpec>;
#[doc = "Field `BUSY` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type BusyR = crate::FieldReader;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type Reserved8R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbbusy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbbusy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbbusySpec;
impl crate::RegisterSpec for FbbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbbusy::R`](R) reader structure"]
impl crate::Readable for FbbusySpec {}
#[doc = "`write(|w| ..)` method takes [`fbbusy::W`](W) writer structure"]
impl crate::Writable for FbbusySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBBUSY to value 0xfe"]
impl crate::Resettable for FbbusySpec {
    const RESET_VALUE: u32 = 0xfe;
}
