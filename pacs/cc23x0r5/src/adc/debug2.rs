#[doc = "Register `DEBUG2` reader"]
pub type R = crate::R<Debug2Spec>;
#[doc = "Register `DEBUG2` writer"]
pub type W = crate::W<Debug2Spec>;
#[doc = "Field `RESERVED0` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader<u32>;
#[doc = "Field `VTOI_TESTMODE_EN` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VtoiTestmodeEnR = crate::BitReader;
#[doc = "Field `VTOI_TESTMODE_EN` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type VtoiTestmodeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 27:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
#[doc = "Field `VTOI_CTRL` reader - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type VtoiCtrlR = crate::FieldReader;
#[doc = "Field `VTOI_CTRL` writer - 29:28\\]
Internal. Only to be used through TI provided API."]
pub type VtoiCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED30` reader - 31:30\\]
Internal. Only to be used through TI provided API."]
pub type Reserved30R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtoi_testmode_en(&self) -> VtoiTestmodeEnR {
        VtoiTestmodeEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - 27:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtoi_ctrl(&self) -> VtoiCtrlR {
        VtoiCtrlR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved30(&self) -> Reserved30R {
        Reserved30R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtoi_testmode_en(&mut self) -> VtoiTestmodeEnW<Debug2Spec> {
        VtoiTestmodeEnW::new(self, 24)
    }
    #[doc = "Bits 28:29 - 29:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtoi_ctrl(&mut self) -> VtoiCtrlW<Debug2Spec> {
        VtoiCtrlW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug2Spec;
impl crate::RegisterSpec for Debug2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug2::R`](R) reader structure"]
impl crate::Readable for Debug2Spec {}
#[doc = "`write(|w| ..)` method takes [`debug2::W`](W) writer structure"]
impl crate::Writable for Debug2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG2 to value 0"]
impl crate::Resettable for Debug2Spec {
    const RESET_VALUE: u32 = 0;
}
