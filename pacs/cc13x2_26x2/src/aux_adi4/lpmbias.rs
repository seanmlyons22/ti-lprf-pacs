#[doc = "Register `LPMBIAS` reader"]
pub type R = crate::R<LpmbiasSpec>;
#[doc = "Register `LPMBIAS` writer"]
pub type W = crate::W<LpmbiasSpec>;
#[doc = "Field `LPM_TRIM_IOUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type LpmTrimIoutR = crate::FieldReader;
#[doc = "Field `LPM_TRIM_IOUT` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type LpmTrimIoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SPARE6` reader - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Spare6R = crate::FieldReader;
#[doc = "Field `SPARE6` writer - 7:6\\]
Internal. Only to be used through TI provided API."]
pub type Spare6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LpmTrimIoutR {
        LpmTrimIoutR::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare6(&self) -> Spare6R {
        Spare6R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_trim_iout(&mut self) -> LpmTrimIoutW<LpmbiasSpec> {
        LpmTrimIoutW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare6(&mut self) -> Spare6W<LpmbiasSpec> {
        Spare6W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmbias::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmbias::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmbiasSpec;
impl crate::RegisterSpec for LpmbiasSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmbias::R`](R) reader structure"]
impl crate::Readable for LpmbiasSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmbias::W`](W) writer structure"]
impl crate::Writable for LpmbiasSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LPMBIAS to value 0"]
impl crate::Resettable for LpmbiasSpec {
    const RESET_VALUE: u8 = 0;
}
