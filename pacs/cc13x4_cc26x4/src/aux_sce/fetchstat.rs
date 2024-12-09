#[doc = "Register `FETCHSTAT` reader"]
pub type R = crate::R<FetchstatSpec>;
#[doc = "Register `FETCHSTAT` writer"]
pub type W = crate::W<FetchstatSpec>;
#[doc = "Field `PC` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type PcR = crate::FieldReader<u16>;
#[doc = "Field `OPCODE` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type OpcodeR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn opcode(&self) -> OpcodeR {
        OpcodeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fetchstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fetchstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchstatSpec;
impl crate::RegisterSpec for FetchstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fetchstat::R`](R) reader structure"]
impl crate::Readable for FetchstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fetchstat::W`](W) writer structure"]
impl crate::Writable for FetchstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FETCHSTAT to value 0"]
impl crate::Resettable for FetchstatSpec {
    const RESET_VALUE: u32 = 0;
}
