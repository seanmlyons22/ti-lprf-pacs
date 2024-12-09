#[doc = "Register `FEFUSESTAT` reader"]
pub type R = crate::R<FefusestatSpec>;
#[doc = "Register `FEFUSESTAT` writer"]
pub type W = crate::W<FefusestatSpec>;
#[doc = "Field `SHIFT_DONE` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ShiftDoneR = crate::BitReader;
#[doc = "Field `SHIFT_DONE` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type ShiftDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED1` reader - 31:1\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn shift_done(&self) -> ShiftDoneR {
        ShiftDoneR::new((self.bits & 1) != 0)
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
    pub fn shift_done(&mut self) -> ShiftDoneW<FefusestatSpec> {
        ShiftDoneW::new(self, 0)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fefusestat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fefusestat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FefusestatSpec;
impl crate::RegisterSpec for FefusestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fefusestat::R`](R) reader structure"]
impl crate::Readable for FefusestatSpec {}
#[doc = "`write(|w| ..)` method takes [`fefusestat::W`](W) writer structure"]
impl crate::Writable for FefusestatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FEFUSESTAT to value 0"]
impl crate::Resettable for FefusestatSpec {
    const RESET_VALUE: u32 = 0;
}
