#[doc = "Register `FBSTROBES` reader"]
pub struct R(crate::R<FBSTROBES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBSTROBES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBSTROBES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBSTROBES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBSTROBES` writer"]
pub struct W(crate::W<FBSTROBES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBSTROBES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FBSTROBES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBSTROBES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBSTROBES_SPEC, u8, u8, 2, O>;
#[doc = "Field `TEZ` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type TEZ_R = crate::BitReader<bool>;
#[doc = "Field `TEZ` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type TEZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `OTP` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type OTP_R = crate::BitReader<bool>;
#[doc = "Field `OTP` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type OTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `TI_OTP` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type TI_OTP_R = crate::BitReader<bool>;
#[doc = "Field `TI_OTP` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type TI_OTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `PRECOL` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PRECOL_R = crate::BitReader<bool>;
#[doc = "Field `PRECOL` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type PRECOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `NOCOLRED` reader - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type NOCOLRED_R = crate::BitReader<bool>;
#[doc = "Field `NOCOLRED` writer - 6:6\\]
Internal. Only to be used through TI provided API."]
pub type NOCOLRED_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED7_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `CTRLENZ` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type CTRLENZ_R = crate::BitReader<bool>;
#[doc = "Field `CTRLENZ` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type CTRLENZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED9` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED9` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBSTROBES_SPEC, u8, u8, 7, O>;
#[doc = "Field `FLCLKEN` reader - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FLCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `FLCLKEN` writer - 16:16\\]
Internal. Only to be used through TI provided API."]
pub type FLCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RWAIT_FLCLK` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT_FLCLK_R = crate::BitReader<bool>;
#[doc = "Field `RWAIT_FLCLK` writer - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT_FLCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RWAIT2_FLCLK` reader - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT2_FLCLK_R = crate::BitReader<bool>;
#[doc = "Field `RWAIT2_FLCLK` writer - 18:18\\]
Internal. Only to be used through TI provided API."]
pub type RWAIT2_FLCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED19` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED19` writer - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBSTROBES_SPEC, u8, u8, 5, O>;
#[doc = "Field `ECBIT` reader - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type ECBIT_R = crate::BitReader<bool>;
#[doc = "Field `ECBIT` writer - 24:24\\]
Internal. Only to be used through TI provided API."]
pub type ECBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FBSTROBES_SPEC, bool, O>;
#[doc = "Field `RESERVED25` reader - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED25` writer - 31:25\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBSTROBES_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn tez(&self) -> TEZ_R {
        TEZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn otp(&self) -> OTP_R {
        OTP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ti_otp(&self) -> TI_OTP_R {
        TI_OTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn precol(&self) -> PRECOL_R {
        PRECOL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn nocolred(&self) -> NOCOLRED_R {
        NOCOLRED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved7(&self) -> RESERVED7_R {
        RESERVED7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctrlenz(&self) -> CTRLENZ_R {
        CTRLENZ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn flclken(&self) -> FLCLKEN_R {
        FLCLKEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait_flclk(&self) -> RWAIT_FLCLK_R {
        RWAIT_FLCLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rwait2_flclk(&self) -> RWAIT2_FLCLK_R {
        RWAIT2_FLCLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved19(&self) -> RESERVED19_R {
        RESERVED19_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ecbit(&self) -> ECBIT_R {
        ECBIT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved25(&self) -> RESERVED25_R {
        RESERVED25_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn tez(&mut self) -> TEZ_W<2> {
        TEZ_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn otp(&mut self) -> OTP_W<3> {
        OTP_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ti_otp(&mut self) -> TI_OTP_W<4> {
        TI_OTP_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn precol(&mut self) -> PRECOL_W<5> {
        PRECOL_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn nocolred(&mut self) -> NOCOLRED_W<6> {
        NOCOLRED_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> RESERVED7_W<7> {
        RESERVED7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctrlenz(&mut self) -> CTRLENZ_W<8> {
        CTRLENZ_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn flclken(&mut self) -> FLCLKEN_W<16> {
        FLCLKEN_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait_flclk(&mut self) -> RWAIT_FLCLK_W<17> {
        RWAIT_FLCLK_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rwait2_flclk(&mut self) -> RWAIT2_FLCLK_W<18> {
        RWAIT2_FLCLK_W::new(self)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved19(&mut self) -> RESERVED19_W<19> {
        RESERVED19_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ecbit(&mut self) -> ECBIT_W<24> {
        ECBIT_W::new(self)
    }
    #[doc = "Bits 25:31 - 31:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved25(&mut self) -> RESERVED25_W<25> {
        RESERVED25_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbstrobes](index.html) module"]
pub struct FBSTROBES_SPEC;
impl crate::RegisterSpec for FBSTROBES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbstrobes::R](R) reader structure"]
impl crate::Readable for FBSTROBES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbstrobes::W](W) writer structure"]
impl crate::Writable for FBSTROBES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBSTROBES to value 0x0104"]
impl crate::Resettable for FBSTROBES_SPEC {
    const RESET_VALUE: Self::Ux = 0x0104;
}
