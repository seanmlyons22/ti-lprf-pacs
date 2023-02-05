#[doc = "Register `EVSTAT1` reader"]
pub struct R(crate::R<EVSTAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVSTAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVSTAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVSTAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVSTAT1` writer"]
pub struct W(crate::W<EVSTAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVSTAT1_SPEC>;
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
impl From<crate::W<EVSTAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVSTAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXIO16` reader - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
pub type AUXIO16_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO16` writer - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
pub type AUXIO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO17` reader - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
pub type AUXIO17_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO17` writer - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
pub type AUXIO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO18` reader - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
pub type AUXIO18_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO18` writer - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
pub type AUXIO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO19` reader - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
pub type AUXIO19_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO19` writer - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
pub type AUXIO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO20` reader - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
pub type AUXIO20_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO20` writer - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
pub type AUXIO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO21` reader - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
pub type AUXIO21_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO21` writer - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
pub type AUXIO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO22` reader - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
pub type AUXIO22_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO22` writer - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
pub type AUXIO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO23` reader - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
pub type AUXIO23_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO23` writer - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
pub type AUXIO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO24` reader - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
pub type AUXIO24_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO24` writer - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
pub type AUXIO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO25` reader - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
pub type AUXIO25_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO25` writer - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
pub type AUXIO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO26` reader - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
pub type AUXIO26_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO26` writer - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
pub type AUXIO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO27` reader - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
pub type AUXIO27_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO27` writer - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
pub type AUXIO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO28` reader - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
pub type AUXIO28_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO28` writer - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
pub type AUXIO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO29` reader - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
pub type AUXIO29_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO29` writer - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
pub type AUXIO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO30` reader - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
pub type AUXIO30_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO30` writer - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
pub type AUXIO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `AUXIO31` reader - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
pub type AUXIO31_R = crate::BitReader<bool>;
#[doc = "Field `AUXIO31` writer - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
pub type AUXIO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVSTAT1_SPEC, bool, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EVSTAT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio16(&self) -> AUXIO16_R {
        AUXIO16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio17(&self) -> AUXIO17_R {
        AUXIO17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio18(&self) -> AUXIO18_R {
        AUXIO18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio19(&self) -> AUXIO19_R {
        AUXIO19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio20(&self) -> AUXIO20_R {
        AUXIO20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio21(&self) -> AUXIO21_R {
        AUXIO21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio22(&self) -> AUXIO22_R {
        AUXIO22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio23(&self) -> AUXIO23_R {
        AUXIO23_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline(always)]
    pub fn auxio24(&self) -> AUXIO24_R {
        AUXIO24_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline(always)]
    pub fn auxio25(&self) -> AUXIO25_R {
        AUXIO25_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline(always)]
    pub fn auxio26(&self) -> AUXIO26_R {
        AUXIO26_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline(always)]
    pub fn auxio27(&self) -> AUXIO27_R {
        AUXIO27_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline(always)]
    pub fn auxio28(&self) -> AUXIO28_R {
        AUXIO28_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline(always)]
    pub fn auxio29(&self) -> AUXIO29_R {
        AUXIO29_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline(always)]
    pub fn auxio30(&self) -> AUXIO30_R {
        AUXIO30_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline(always)]
    pub fn auxio31(&self) -> AUXIO31_R {
        AUXIO31_R::new(((self.bits >> 15) & 1) != 0)
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
AUXIO16 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio16(&mut self) -> AUXIO16_W<0> {
        AUXIO16_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
AUXIO17 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio17(&mut self) -> AUXIO17_W<1> {
        AUXIO17_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
AUXIO18 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio18(&mut self) -> AUXIO18_W<2> {
        AUXIO18_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
AUXIO19 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio19(&mut self) -> AUXIO19_W<3> {
        AUXIO19_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
AUXIO20 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio20(&mut self) -> AUXIO20_W<4> {
        AUXIO20_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
AUXIO21 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio21(&mut self) -> AUXIO21_W<5> {
        AUXIO21_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
AUXIO22 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio22(&mut self) -> AUXIO22_W<6> {
        AUXIO22_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
AUXIO23 pin level, read value corresponds to AUX_AIODIO2:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio23(&mut self) -> AUXIO23_W<7> {
        AUXIO23_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
AUXIO24 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 0."]
    #[inline(always)]
    #[must_use]
    pub fn auxio24(&mut self) -> AUXIO24_W<8> {
        AUXIO24_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
AUXIO25 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 1."]
    #[inline(always)]
    #[must_use]
    pub fn auxio25(&mut self) -> AUXIO25_W<9> {
        AUXIO25_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
AUXIO26 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 2."]
    #[inline(always)]
    #[must_use]
    pub fn auxio26(&mut self) -> AUXIO26_W<10> {
        AUXIO26_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
AUXIO27 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 3."]
    #[inline(always)]
    #[must_use]
    pub fn auxio27(&mut self) -> AUXIO27_W<11> {
        AUXIO27_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
AUXIO28 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 4."]
    #[inline(always)]
    #[must_use]
    pub fn auxio28(&mut self) -> AUXIO28_W<12> {
        AUXIO28_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
AUXIO29 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 5."]
    #[inline(always)]
    #[must_use]
    pub fn auxio29(&mut self) -> AUXIO29_W<13> {
        AUXIO29_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
AUXIO30 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 6."]
    #[inline(always)]
    #[must_use]
    pub fn auxio30(&mut self) -> AUXIO30_W<14> {
        AUXIO30_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
AUXIO31 pin level, read value corresponds to AUX_AIODIO3:GPIODIN bit 7."]
    #[inline(always)]
    #[must_use]
    pub fn auxio31(&mut self) -> AUXIO31_W<15> {
        AUXIO31_W::new(self)
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
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evstat1](index.html) module"]
pub struct EVSTAT1_SPEC;
impl crate::RegisterSpec for EVSTAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evstat1::R](R) reader structure"]
impl crate::Readable for EVSTAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evstat1::W](W) writer structure"]
impl crate::Writable for EVSTAT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVSTAT1 to value 0"]
impl crate::Resettable for EVSTAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
