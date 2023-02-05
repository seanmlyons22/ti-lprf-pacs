#[doc = "Register `CPUSTAT` reader"]
pub struct R(crate::R<CPUSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPUSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPUSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPUSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPUSTAT` writer"]
pub struct W(crate::W<CPUSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPUSTAT_SPEC>;
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
impl From<crate::W<CPUSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPUSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Z_FLAG` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Z_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `Z_FLAG` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type Z_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `N_FLAG` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type N_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `N_FLAG` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type N_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `C_FLAG` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type C_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `C_FLAG` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type C_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `V_FLAG` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type V_FLAG_R = crate::BitReader<bool>;
#[doc = "Field `V_FLAG` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type V_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED4` reader - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 7:4\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUSTAT_SPEC, u8, u8, 4, O>;
#[doc = "Field `SELF_STOP` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type SELF_STOP_R = crate::BitReader<bool>;
#[doc = "Field `SELF_STOP` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type SELF_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `WEV` reader - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type WEV_R = crate::BitReader<bool>;
#[doc = "Field `WEV` writer - 9:9\\]
Internal. Only to be used through TI provided API."]
pub type WEV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - 10:10\\]
Internal. Only to be used through TI provided API."]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `BUS_ERROR` reader - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type BUS_ERROR_R = crate::BitReader<bool>;
#[doc = "Field `BUS_ERROR` writer - 11:11\\]
Internal. Only to be used through TI provided API."]
pub type BUS_ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPUSTAT_SPEC, bool, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPUSTAT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn z_flag(&self) -> Z_FLAG_R {
        Z_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn n_flag(&self) -> N_FLAG_R {
        N_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn c_flag(&self) -> C_FLAG_R {
        C_FLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn v_flag(&self) -> V_FLAG_R {
        V_FLAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn self_stop(&self) -> SELF_STOP_R {
        SELF_STOP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn wev(&self) -> WEV_R {
        WEV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn z_flag(&mut self) -> Z_FLAG_W<0> {
        Z_FLAG_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn n_flag(&mut self) -> N_FLAG_W<1> {
        N_FLAG_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn c_flag(&mut self) -> C_FLAG_W<2> {
        C_FLAG_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn v_flag(&mut self) -> V_FLAG_W<3> {
        V_FLAG_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn self_stop(&mut self) -> SELF_STOP_W<8> {
        SELF_STOP_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn wev(&mut self) -> WEV_W<9> {
        WEV_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<10> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn bus_error(&mut self) -> BUS_ERROR_W<11> {
        BUS_ERROR_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpustat](index.html) module"]
pub struct CPUSTAT_SPEC;
impl crate::RegisterSpec for CPUSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpustat::R](R) reader structure"]
impl crate::Readable for CPUSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpustat::W](W) writer structure"]
impl crate::Writable for CPUSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPUSTAT to value 0"]
impl crate::Resettable for CPUSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
