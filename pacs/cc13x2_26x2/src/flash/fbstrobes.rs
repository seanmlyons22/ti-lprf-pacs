#[doc = "Register `FBSTROBES` reader"]
pub type R = crate::R<FbstrobesSpec>;
#[doc = "Register `FBSTROBES` writer"]
pub type W = crate::W<FbstrobesSpec>;
#[doc = "Field `RESERVED0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type Reserved0R = crate::FieldReader;
#[doc = "Field `TEZ` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type TezR = crate::BitReader;
#[doc = "Field `TEZ` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type TezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTP` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type OtpR = crate::BitReader;
#[doc = "Field `OTP` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type OtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI_OTP` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type TiOtpR = crate::BitReader;
#[doc = "Field `TI_OTP` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type TiOtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRECOL` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PrecolR = crate::BitReader;
#[doc = "Field `PRECOL` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PrecolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOCOLRED` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type NocolredR = crate::BitReader;
#[doc = "Field `NOCOLRED` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type NocolredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `CTRLENZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type CtrlenzR = crate::BitReader;
#[doc = "Field `CTRLENZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type CtrlenzW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `FLCLKEN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FlclkenR = crate::BitReader;
#[doc = "Field `FLCLKEN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FlclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWAIT_FLCLK` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RwaitFlclkR = crate::BitReader;
#[doc = "Field `RWAIT_FLCLK` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RwaitFlclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWAIT2_FLCLK` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type Rwait2FlclkR = crate::BitReader;
#[doc = "Field `RWAIT2_FLCLK` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type Rwait2FlclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED19` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type Reserved19R = crate::FieldReader;
#[doc = "Field `ECBIT` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type EcbitR = crate::BitReader;
#[doc = "Field `ECBIT` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type EcbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type Reserved25R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tez(&self) -> TezR {
        TezR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp(&self) -> OtpR {
        OtpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ti_otp(&self) -> TiOtpR {
        TiOtpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn precol(&self) -> PrecolR {
        PrecolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nocolred(&self) -> NocolredR {
        NocolredR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CtrlenzR {
        CtrlenzR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flclken(&self) -> FlclkenR {
        FlclkenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait_flclk(&self) -> RwaitFlclkR {
        RwaitFlclkR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait2_flclk(&self) -> Rwait2FlclkR {
        Rwait2FlclkR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecbit(&self) -> EcbitR {
        EcbitR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tez(&mut self) -> TezW<FbstrobesSpec> {
        TezW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn otp(&mut self) -> OtpW<FbstrobesSpec> {
        OtpW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ti_otp(&mut self) -> TiOtpW<FbstrobesSpec> {
        TiOtpW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn precol(&mut self) -> PrecolW<FbstrobesSpec> {
        PrecolW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nocolred(&mut self) -> NocolredW<FbstrobesSpec> {
        NocolredW::new(self, 6)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlenz(&mut self) -> CtrlenzW<FbstrobesSpec> {
        CtrlenzW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flclken(&mut self) -> FlclkenW<FbstrobesSpec> {
        FlclkenW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait_flclk(&mut self) -> RwaitFlclkW<FbstrobesSpec> {
        RwaitFlclkW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait2_flclk(&mut self) -> Rwait2FlclkW<FbstrobesSpec> {
        Rwait2FlclkW::new(self, 18)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ecbit(&mut self) -> EcbitW<FbstrobesSpec> {
        EcbitW::new(self, 24)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbstrobes::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbstrobes::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FbstrobesSpec;
impl crate::RegisterSpec for FbstrobesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbstrobes::R`](R) reader structure"]
impl crate::Readable for FbstrobesSpec {}
#[doc = "`write(|w| ..)` method takes [`fbstrobes::W`](W) writer structure"]
impl crate::Writable for FbstrobesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FBSTROBES to value 0x0104"]
impl crate::Resettable for FbstrobesSpec {
    const RESET_VALUE: u32 = 0x0104;
}
