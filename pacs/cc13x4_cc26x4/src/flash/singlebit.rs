#[doc = "Register `SINGLEBIT` reader"]
pub type R = crate::R<SinglebitSpec>;
#[doc = "Register `SINGLEBIT` writer"]
pub type W = crate::W<SinglebitSpec>;
#[doc = "Field `FROM0` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type From0R = crate::BitReader;
#[doc = "Field `FROMN` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type FromnR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn from0(&self) -> From0R {
        From0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn fromn(&self) -> FromnR {
        FromnR::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`singlebit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`singlebit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglebitSpec;
impl crate::RegisterSpec for SinglebitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlebit::R`](R) reader structure"]
impl crate::Readable for SinglebitSpec {}
#[doc = "`write(|w| ..)` method takes [`singlebit::W`](W) writer structure"]
impl crate::Writable for SinglebitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SINGLEBIT to value 0"]
impl crate::Resettable for SinglebitSpec {
    const RESET_VALUE: u32 = 0;
}
