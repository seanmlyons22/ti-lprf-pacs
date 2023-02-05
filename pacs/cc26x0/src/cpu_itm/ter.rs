#[doc = "Register `TER` reader"]
pub struct R(crate::R<TER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TER` writer"]
pub struct W(crate::W<TER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TER_SPEC>;
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
impl From<crate::W<TER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STIMENA0` reader - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
pub type STIMENA0_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA0` writer - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
pub type STIMENA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA1` reader - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
pub type STIMENA1_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA1` writer - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
pub type STIMENA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA2` reader - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
pub type STIMENA2_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA2` writer - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
pub type STIMENA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA3` reader - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
pub type STIMENA3_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA3` writer - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
pub type STIMENA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA4` reader - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
pub type STIMENA4_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA4` writer - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
pub type STIMENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA5` reader - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
pub type STIMENA5_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA5` writer - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
pub type STIMENA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA6` reader - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
pub type STIMENA6_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA6` writer - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
pub type STIMENA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA7` reader - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
pub type STIMENA7_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA7` writer - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
pub type STIMENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA8` reader - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
pub type STIMENA8_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA8` writer - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
pub type STIMENA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA9` reader - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
pub type STIMENA9_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA9` writer - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
pub type STIMENA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA10` reader - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
pub type STIMENA10_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA10` writer - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
pub type STIMENA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA11` reader - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
pub type STIMENA11_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA11` writer - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
pub type STIMENA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA12` reader - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
pub type STIMENA12_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA12` writer - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
pub type STIMENA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA13` reader - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
pub type STIMENA13_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA13` writer - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
pub type STIMENA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA14` reader - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
pub type STIMENA14_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA14` writer - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
pub type STIMENA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA15` reader - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
pub type STIMENA15_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA15` writer - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
pub type STIMENA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA16` reader - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
pub type STIMENA16_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA16` writer - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
pub type STIMENA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA17` reader - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
pub type STIMENA17_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA17` writer - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
pub type STIMENA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA18` reader - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
pub type STIMENA18_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA18` writer - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
pub type STIMENA18_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA19` reader - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
pub type STIMENA19_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA19` writer - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
pub type STIMENA19_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA20` reader - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
pub type STIMENA20_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA20` writer - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
pub type STIMENA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA21` reader - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
pub type STIMENA21_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA21` writer - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
pub type STIMENA21_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA22` reader - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
pub type STIMENA22_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA22` writer - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
pub type STIMENA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA23` reader - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
pub type STIMENA23_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA23` writer - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
pub type STIMENA23_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA24` reader - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
pub type STIMENA24_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA24` writer - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
pub type STIMENA24_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA25` reader - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
pub type STIMENA25_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA25` writer - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
pub type STIMENA25_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA26` reader - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
pub type STIMENA26_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA26` writer - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
pub type STIMENA26_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA27` reader - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
pub type STIMENA27_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA27` writer - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
pub type STIMENA27_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA28` reader - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
pub type STIMENA28_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA28` writer - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
pub type STIMENA28_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA29` reader - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
pub type STIMENA29_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA29` writer - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
pub type STIMENA29_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA30` reader - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
pub type STIMENA30_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA30` writer - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
pub type STIMENA30_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
#[doc = "Field `STIMENA31` reader - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
pub type STIMENA31_R = crate::BitReader<bool>;
#[doc = "Field `STIMENA31` writer - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
pub type STIMENA31_W<'a, const O: u8> = crate::BitWriter<'a, u32, TER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
    #[inline(always)]
    pub fn stimena0(&self) -> STIMENA0_R {
        STIMENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
    #[inline(always)]
    pub fn stimena1(&self) -> STIMENA1_R {
        STIMENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
    #[inline(always)]
    pub fn stimena2(&self) -> STIMENA2_R {
        STIMENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
    #[inline(always)]
    pub fn stimena3(&self) -> STIMENA3_R {
        STIMENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
    #[inline(always)]
    pub fn stimena4(&self) -> STIMENA4_R {
        STIMENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
    #[inline(always)]
    pub fn stimena5(&self) -> STIMENA5_R {
        STIMENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
    #[inline(always)]
    pub fn stimena6(&self) -> STIMENA6_R {
        STIMENA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
    #[inline(always)]
    pub fn stimena7(&self) -> STIMENA7_R {
        STIMENA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
    #[inline(always)]
    pub fn stimena8(&self) -> STIMENA8_R {
        STIMENA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
    #[inline(always)]
    pub fn stimena9(&self) -> STIMENA9_R {
        STIMENA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
    #[inline(always)]
    pub fn stimena10(&self) -> STIMENA10_R {
        STIMENA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
    #[inline(always)]
    pub fn stimena11(&self) -> STIMENA11_R {
        STIMENA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
    #[inline(always)]
    pub fn stimena12(&self) -> STIMENA12_R {
        STIMENA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
    #[inline(always)]
    pub fn stimena13(&self) -> STIMENA13_R {
        STIMENA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
    #[inline(always)]
    pub fn stimena14(&self) -> STIMENA14_R {
        STIMENA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
    #[inline(always)]
    pub fn stimena15(&self) -> STIMENA15_R {
        STIMENA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
    #[inline(always)]
    pub fn stimena16(&self) -> STIMENA16_R {
        STIMENA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
    #[inline(always)]
    pub fn stimena17(&self) -> STIMENA17_R {
        STIMENA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
    #[inline(always)]
    pub fn stimena18(&self) -> STIMENA18_R {
        STIMENA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
    #[inline(always)]
    pub fn stimena19(&self) -> STIMENA19_R {
        STIMENA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
    #[inline(always)]
    pub fn stimena20(&self) -> STIMENA20_R {
        STIMENA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
    #[inline(always)]
    pub fn stimena21(&self) -> STIMENA21_R {
        STIMENA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
    #[inline(always)]
    pub fn stimena22(&self) -> STIMENA22_R {
        STIMENA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
    #[inline(always)]
    pub fn stimena23(&self) -> STIMENA23_R {
        STIMENA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
    #[inline(always)]
    pub fn stimena24(&self) -> STIMENA24_R {
        STIMENA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
    #[inline(always)]
    pub fn stimena25(&self) -> STIMENA25_R {
        STIMENA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
    #[inline(always)]
    pub fn stimena26(&self) -> STIMENA26_R {
        STIMENA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
    #[inline(always)]
    pub fn stimena27(&self) -> STIMENA27_R {
        STIMENA27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
    #[inline(always)]
    pub fn stimena28(&self) -> STIMENA28_R {
        STIMENA28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
    #[inline(always)]
    pub fn stimena29(&self) -> STIMENA29_R {
        STIMENA29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
    #[inline(always)]
    pub fn stimena30(&self) -> STIMENA30_R {
        STIMENA30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
    #[inline(always)]
    pub fn stimena31(&self) -> STIMENA31_R {
        STIMENA31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Bit mask to enable tracing on ITM stimulus port 0."]
    #[inline(always)]
    #[must_use]
    pub fn stimena0(&mut self) -> STIMENA0_W<0> {
        STIMENA0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Bit mask to enable tracing on ITM stimulus port 1."]
    #[inline(always)]
    #[must_use]
    pub fn stimena1(&mut self) -> STIMENA1_W<1> {
        STIMENA1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Bit mask to enable tracing on ITM stimulus port 2."]
    #[inline(always)]
    #[must_use]
    pub fn stimena2(&mut self) -> STIMENA2_W<2> {
        STIMENA2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Bit mask to enable tracing on ITM stimulus port 3."]
    #[inline(always)]
    #[must_use]
    pub fn stimena3(&mut self) -> STIMENA3_W<3> {
        STIMENA3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Bit mask to enable tracing on ITM stimulus port 4."]
    #[inline(always)]
    #[must_use]
    pub fn stimena4(&mut self) -> STIMENA4_W<4> {
        STIMENA4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Bit mask to enable tracing on ITM stimulus port 5."]
    #[inline(always)]
    #[must_use]
    pub fn stimena5(&mut self) -> STIMENA5_W<5> {
        STIMENA5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Bit mask to enable tracing on ITM stimulus port 6."]
    #[inline(always)]
    #[must_use]
    pub fn stimena6(&mut self) -> STIMENA6_W<6> {
        STIMENA6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Bit mask to enable tracing on ITM stimulus port 7."]
    #[inline(always)]
    #[must_use]
    pub fn stimena7(&mut self) -> STIMENA7_W<7> {
        STIMENA7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Bit mask to enable tracing on ITM stimulus port 8."]
    #[inline(always)]
    #[must_use]
    pub fn stimena8(&mut self) -> STIMENA8_W<8> {
        STIMENA8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Bit mask to enable tracing on ITM stimulus port 9."]
    #[inline(always)]
    #[must_use]
    pub fn stimena9(&mut self) -> STIMENA9_W<9> {
        STIMENA9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Bit mask to enable tracing on ITM stimulus port 10."]
    #[inline(always)]
    #[must_use]
    pub fn stimena10(&mut self) -> STIMENA10_W<10> {
        STIMENA10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Bit mask to enable tracing on ITM stimulus port 11."]
    #[inline(always)]
    #[must_use]
    pub fn stimena11(&mut self) -> STIMENA11_W<11> {
        STIMENA11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Bit mask to enable tracing on ITM stimulus port 12."]
    #[inline(always)]
    #[must_use]
    pub fn stimena12(&mut self) -> STIMENA12_W<12> {
        STIMENA12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Bit mask to enable tracing on ITM stimulus port 13."]
    #[inline(always)]
    #[must_use]
    pub fn stimena13(&mut self) -> STIMENA13_W<13> {
        STIMENA13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Bit mask to enable tracing on ITM stimulus port 14."]
    #[inline(always)]
    #[must_use]
    pub fn stimena14(&mut self) -> STIMENA14_W<14> {
        STIMENA14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Bit mask to enable tracing on ITM stimulus port 15."]
    #[inline(always)]
    #[must_use]
    pub fn stimena15(&mut self) -> STIMENA15_W<15> {
        STIMENA15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Bit mask to enable tracing on ITM stimulus port 16."]
    #[inline(always)]
    #[must_use]
    pub fn stimena16(&mut self) -> STIMENA16_W<16> {
        STIMENA16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Bit mask to enable tracing on ITM stimulus port 17."]
    #[inline(always)]
    #[must_use]
    pub fn stimena17(&mut self) -> STIMENA17_W<17> {
        STIMENA17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Bit mask to enable tracing on ITM stimulus port 18."]
    #[inline(always)]
    #[must_use]
    pub fn stimena18(&mut self) -> STIMENA18_W<18> {
        STIMENA18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Bit mask to enable tracing on ITM stimulus port 19."]
    #[inline(always)]
    #[must_use]
    pub fn stimena19(&mut self) -> STIMENA19_W<19> {
        STIMENA19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Bit mask to enable tracing on ITM stimulus port 20."]
    #[inline(always)]
    #[must_use]
    pub fn stimena20(&mut self) -> STIMENA20_W<20> {
        STIMENA20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Bit mask to enable tracing on ITM stimulus port 21."]
    #[inline(always)]
    #[must_use]
    pub fn stimena21(&mut self) -> STIMENA21_W<21> {
        STIMENA21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Bit mask to enable tracing on ITM stimulus port 22."]
    #[inline(always)]
    #[must_use]
    pub fn stimena22(&mut self) -> STIMENA22_W<22> {
        STIMENA22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Bit mask to enable tracing on ITM stimulus port 23."]
    #[inline(always)]
    #[must_use]
    pub fn stimena23(&mut self) -> STIMENA23_W<23> {
        STIMENA23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Bit mask to enable tracing on ITM stimulus port 24."]
    #[inline(always)]
    #[must_use]
    pub fn stimena24(&mut self) -> STIMENA24_W<24> {
        STIMENA24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Bit mask to enable tracing on ITM stimulus port 25."]
    #[inline(always)]
    #[must_use]
    pub fn stimena25(&mut self) -> STIMENA25_W<25> {
        STIMENA25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Bit mask to enable tracing on ITM stimulus port 26."]
    #[inline(always)]
    #[must_use]
    pub fn stimena26(&mut self) -> STIMENA26_W<26> {
        STIMENA26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Bit mask to enable tracing on ITM stimulus port 27."]
    #[inline(always)]
    #[must_use]
    pub fn stimena27(&mut self) -> STIMENA27_W<27> {
        STIMENA27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Bit mask to enable tracing on ITM stimulus port 28."]
    #[inline(always)]
    #[must_use]
    pub fn stimena28(&mut self) -> STIMENA28_W<28> {
        STIMENA28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Bit mask to enable tracing on ITM stimulus port 29."]
    #[inline(always)]
    #[must_use]
    pub fn stimena29(&mut self) -> STIMENA29_W<29> {
        STIMENA29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Bit mask to enable tracing on ITM stimulus port 30."]
    #[inline(always)]
    #[must_use]
    pub fn stimena30(&mut self) -> STIMENA30_W<30> {
        STIMENA30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Bit mask to enable tracing on ITM stimulus port 31."]
    #[inline(always)]
    #[must_use]
    pub fn stimena31(&mut self) -> STIMENA31_W<31> {
        STIMENA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ter](index.html) module"]
pub struct TER_SPEC;
impl crate::RegisterSpec for TER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ter::R](R) reader structure"]
impl crate::Readable for TER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ter::W](W) writer structure"]
impl crate::Writable for TER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TER to value 0"]
impl crate::Resettable for TER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
