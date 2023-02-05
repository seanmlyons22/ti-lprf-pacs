#[doc = "Register `EVSTAT0` reader"]
pub struct R(crate::R<EVSTAT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT0` writer"]
pub struct W(crate::W<EVSTAT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT0_SPEC>;
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
impl From<crate::W<EVSTAT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXIO0` reader - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type AUXIO0_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO0` writer - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
pub type AUXIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO1` reader - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type AUXIO1_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO1` writer - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
pub type AUXIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO2` reader - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type AUXIO2_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO2` writer - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
pub type AUXIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO3` reader - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type AUXIO3_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO3` writer - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
pub type AUXIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO4` reader - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type AUXIO4_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO4` writer - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
pub type AUXIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO5` reader - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type AUXIO5_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO5` writer - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
pub type AUXIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO6` reader - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type AUXIO6_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO6` writer - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
pub type AUXIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO7` reader - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type AUXIO7_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO7` writer - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
pub type AUXIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO8` reader - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type AUXIO8_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO8` writer - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
pub type AUXIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO9` reader - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type AUXIO9_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO9` writer - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
pub type AUXIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO10` reader - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type AUXIO10_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO10` writer - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
pub type AUXIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO11` reader - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type AUXIO11_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO11` writer - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
pub type AUXIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO12` reader - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type AUXIO12_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO12` writer - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
pub type AUXIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO13` reader - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type AUXIO13_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO13` writer - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
pub type AUXIO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO14` reader - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type AUXIO14_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO14` writer - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
pub type AUXIO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `AUXIO15` reader - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type AUXIO15_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO15` writer - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
pub type AUXIO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT0_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio0(&self) -> AUXIO0_R {
        AUXIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio1(&self) -> AUXIO1_R {
        AUXIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio2(&self) -> AUXIO2_R {
        AUXIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio3(&self) -> AUXIO3_R {
        AUXIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio4(&self) -> AUXIO4_R {
        AUXIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio5(&self) -> AUXIO5_R {
        AUXIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio6(&self) -> AUXIO6_R {
        AUXIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio7(&self) -> AUXIO7_R {
        AUXIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio8(&self) -> AUXIO8_R {
        AUXIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio9(&self) -> AUXIO9_R {
        AUXIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio10(&self) -> AUXIO10_R {
        AUXIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio11(&self) -> AUXIO11_R {
        AUXIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio12(&self) -> AUXIO12_R {
        AUXIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio13(&self) -> AUXIO13_R {
        AUXIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio14(&self) -> AUXIO14_R {
        AUXIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio15(&self) -> AUXIO15_R {
        AUXIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
AUXIO0 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio0(&mut self) -> AUXIO0_W<0> {
        AUXIO0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO1 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio1(&mut self) -> AUXIO1_W<1> {
        AUXIO1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO2 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio2(&mut self) -> AUXIO2_W<2> {
        AUXIO2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO3 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio3(&mut self) -> AUXIO3_W<3> {
        AUXIO3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO4 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio4(&mut self) -> AUXIO4_W<4> {
        AUXIO4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO5 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio5(&mut self) -> AUXIO5_W<5> {
        AUXIO5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO6 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio6(&mut self) -> AUXIO6_W<6> {
        AUXIO6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO7 pin level, read value corresponds to AUX_AIODIO0:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio7(&mut self) -> AUXIO7_W<7> {
        AUXIO7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO8 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio8(&mut self) -> AUXIO8_W<8> {
        AUXIO8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO9 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio9(&mut self) -> AUXIO9_W<9> {
        AUXIO9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO10 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio10(&mut self) -> AUXIO10_W<10> {
        AUXIO10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO11 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio11(&mut self) -> AUXIO11_W<11> {
        AUXIO11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO12 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio12(&mut self) -> AUXIO12_W<12> {
        AUXIO12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO13 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio13(&mut self) -> AUXIO13_W<13> {
        AUXIO13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO14 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio14(&mut self) -> AUXIO14_W<14> {
        AUXIO14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO15 pin level, read value corresponds to AUX_AIODIO1:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio15(&mut self) -> AUXIO15_W<15> {
        AUXIO15_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat0](index.html) module"]
pub struct EVSTAT0_SPEC;
impl crate::RegisterSpec for EVSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat0::R](R) reader structure"]
impl crate::Readable for EVSTAT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat0::W](W) writer structure"]
impl crate::Writable for EVSTAT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT0 to value 0"]
impl crate::Resettable for EVSTAT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
