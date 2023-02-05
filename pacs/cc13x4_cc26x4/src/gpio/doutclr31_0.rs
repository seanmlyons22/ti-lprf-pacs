#[doc = "Register `DOUTCLR31_0` reader"]
pub struct R(crate::R<DOUTCLR31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTCLR31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTCLR31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTCLR31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOUTCLR31_0` writer"]
pub struct W(crate::W<DOUTCLR31_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOUTCLR31_0_SPEC>;
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
impl From<crate::W<DOUTCLR31_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOUTCLR31_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIO0` reader - 0:0\\]
Clears bit 0"]
pub type DIO0_R = crate::BitReader<bool>;
#[doc = "Field `DIO0` writer - 0:0\\]
Clears bit 0"]
pub type DIO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO1` reader - 1:1\\]
Clears bit 1"]
pub type DIO1_R = crate::BitReader<bool>;
#[doc = "Field `DIO1` writer - 1:1\\]
Clears bit 1"]
pub type DIO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO2` reader - 2:2\\]
Clears bit 2"]
pub type DIO2_R = crate::BitReader<bool>;
#[doc = "Field `DIO2` writer - 2:2\\]
Clears bit 2"]
pub type DIO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO3` reader - 3:3\\]
Clears bit 3"]
pub type DIO3_R = crate::BitReader<bool>;
#[doc = "Field `DIO3` writer - 3:3\\]
Clears bit 3"]
pub type DIO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO4` reader - 4:4\\]
Clears bit 4"]
pub type DIO4_R = crate::BitReader<bool>;
#[doc = "Field `DIO4` writer - 4:4\\]
Clears bit 4"]
pub type DIO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO5` reader - 5:5\\]
Clears bit 5"]
pub type DIO5_R = crate::BitReader<bool>;
#[doc = "Field `DIO5` writer - 5:5\\]
Clears bit 5"]
pub type DIO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO6` reader - 6:6\\]
Clears bit 6"]
pub type DIO6_R = crate::BitReader<bool>;
#[doc = "Field `DIO6` writer - 6:6\\]
Clears bit 6"]
pub type DIO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO7` reader - 7:7\\]
Clears bit 7"]
pub type DIO7_R = crate::BitReader<bool>;
#[doc = "Field `DIO7` writer - 7:7\\]
Clears bit 7"]
pub type DIO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO8` reader - 8:8\\]
Clears bit 8"]
pub type DIO8_R = crate::BitReader<bool>;
#[doc = "Field `DIO8` writer - 8:8\\]
Clears bit 8"]
pub type DIO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO9` reader - 9:9\\]
Clears bit 9"]
pub type DIO9_R = crate::BitReader<bool>;
#[doc = "Field `DIO9` writer - 9:9\\]
Clears bit 9"]
pub type DIO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO10` reader - 10:10\\]
Clears bit 10"]
pub type DIO10_R = crate::BitReader<bool>;
#[doc = "Field `DIO10` writer - 10:10\\]
Clears bit 10"]
pub type DIO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO11` reader - 11:11\\]
Clears bit 11"]
pub type DIO11_R = crate::BitReader<bool>;
#[doc = "Field `DIO11` writer - 11:11\\]
Clears bit 11"]
pub type DIO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO12` reader - 12:12\\]
Clears bit 12"]
pub type DIO12_R = crate::BitReader<bool>;
#[doc = "Field `DIO12` writer - 12:12\\]
Clears bit 12"]
pub type DIO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO13` reader - 13:13\\]
Clears bit 13"]
pub type DIO13_R = crate::BitReader<bool>;
#[doc = "Field `DIO13` writer - 13:13\\]
Clears bit 13"]
pub type DIO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO14` reader - 14:14\\]
Clears bit 14"]
pub type DIO14_R = crate::BitReader<bool>;
#[doc = "Field `DIO14` writer - 14:14\\]
Clears bit 14"]
pub type DIO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO15` reader - 15:15\\]
Clears bit 15"]
pub type DIO15_R = crate::BitReader<bool>;
#[doc = "Field `DIO15` writer - 15:15\\]
Clears bit 15"]
pub type DIO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO16` reader - 16:16\\]
Clears bit 16"]
pub type DIO16_R = crate::BitReader<bool>;
#[doc = "Field `DIO16` writer - 16:16\\]
Clears bit 16"]
pub type DIO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO17` reader - 17:17\\]
Clears bit 17"]
pub type DIO17_R = crate::BitReader<bool>;
#[doc = "Field `DIO17` writer - 17:17\\]
Clears bit 17"]
pub type DIO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO18` reader - 18:18\\]
Clears bit 18"]
pub type DIO18_R = crate::BitReader<bool>;
#[doc = "Field `DIO18` writer - 18:18\\]
Clears bit 18"]
pub type DIO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO19` reader - 19:19\\]
Clears bit 19"]
pub type DIO19_R = crate::BitReader<bool>;
#[doc = "Field `DIO19` writer - 19:19\\]
Clears bit 19"]
pub type DIO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO20` reader - 20:20\\]
Clears bit 20"]
pub type DIO20_R = crate::BitReader<bool>;
#[doc = "Field `DIO20` writer - 20:20\\]
Clears bit 20"]
pub type DIO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO21` reader - 21:21\\]
Clears bit 21"]
pub type DIO21_R = crate::BitReader<bool>;
#[doc = "Field `DIO21` writer - 21:21\\]
Clears bit 21"]
pub type DIO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO22` reader - 22:22\\]
Clears bit 22"]
pub type DIO22_R = crate::BitReader<bool>;
#[doc = "Field `DIO22` writer - 22:22\\]
Clears bit 22"]
pub type DIO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO23` reader - 23:23\\]
Clears bit 23"]
pub type DIO23_R = crate::BitReader<bool>;
#[doc = "Field `DIO23` writer - 23:23\\]
Clears bit 23"]
pub type DIO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO24` reader - 24:24\\]
Clears bit 24"]
pub type DIO24_R = crate::BitReader<bool>;
#[doc = "Field `DIO24` writer - 24:24\\]
Clears bit 24"]
pub type DIO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO25` reader - 25:25\\]
Clears bit 25"]
pub type DIO25_R = crate::BitReader<bool>;
#[doc = "Field `DIO25` writer - 25:25\\]
Clears bit 25"]
pub type DIO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO26` reader - 26:26\\]
Clears bit 26"]
pub type DIO26_R = crate::BitReader<bool>;
#[doc = "Field `DIO26` writer - 26:26\\]
Clears bit 26"]
pub type DIO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO27` reader - 27:27\\]
Clears bit 27"]
pub type DIO27_R = crate::BitReader<bool>;
#[doc = "Field `DIO27` writer - 27:27\\]
Clears bit 27"]
pub type DIO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO28` reader - 28:28\\]
Clears bit 28"]
pub type DIO28_R = crate::BitReader<bool>;
#[doc = "Field `DIO28` writer - 28:28\\]
Clears bit 28"]
pub type DIO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO29` reader - 29:29\\]
Clears bit 29"]
pub type DIO29_R = crate::BitReader<bool>;
#[doc = "Field `DIO29` writer - 29:29\\]
Clears bit 29"]
pub type DIO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO30` reader - 30:30\\]
Clears bit 30"]
pub type DIO30_R = crate::BitReader<bool>;
#[doc = "Field `DIO30` writer - 30:30\\]
Clears bit 30"]
pub type DIO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
#[doc = "Field `DIO31` reader - 31:31\\]
Clears bit 31"]
pub type DIO31_R = crate::BitReader<bool>;
#[doc = "Field `DIO31` writer - 31:31\\]
Clears bit 31"]
pub type DIO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOUTCLR31_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clears bit 0"]
    #[inline(always)]
    pub fn dio0(&self) -> DIO0_R {
        DIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears bit 1"]
    #[inline(always)]
    pub fn dio1(&self) -> DIO1_R {
        DIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears bit 2"]
    #[inline(always)]
    pub fn dio2(&self) -> DIO2_R {
        DIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears bit 3"]
    #[inline(always)]
    pub fn dio3(&self) -> DIO3_R {
        DIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears bit 4"]
    #[inline(always)]
    pub fn dio4(&self) -> DIO4_R {
        DIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears bit 5"]
    #[inline(always)]
    pub fn dio5(&self) -> DIO5_R {
        DIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clears bit 6"]
    #[inline(always)]
    pub fn dio6(&self) -> DIO6_R {
        DIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clears bit 7"]
    #[inline(always)]
    pub fn dio7(&self) -> DIO7_R {
        DIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Clears bit 8"]
    #[inline(always)]
    pub fn dio8(&self) -> DIO8_R {
        DIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Clears bit 9"]
    #[inline(always)]
    pub fn dio9(&self) -> DIO9_R {
        DIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Clears bit 10"]
    #[inline(always)]
    pub fn dio10(&self) -> DIO10_R {
        DIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Clears bit 11"]
    #[inline(always)]
    pub fn dio11(&self) -> DIO11_R {
        DIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Clears bit 12"]
    #[inline(always)]
    pub fn dio12(&self) -> DIO12_R {
        DIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Clears bit 13"]
    #[inline(always)]
    pub fn dio13(&self) -> DIO13_R {
        DIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Clears bit 14"]
    #[inline(always)]
    pub fn dio14(&self) -> DIO14_R {
        DIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Clears bit 15"]
    #[inline(always)]
    pub fn dio15(&self) -> DIO15_R {
        DIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears bit 16"]
    #[inline(always)]
    pub fn dio16(&self) -> DIO16_R {
        DIO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Clears bit 17"]
    #[inline(always)]
    pub fn dio17(&self) -> DIO17_R {
        DIO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Clears bit 18"]
    #[inline(always)]
    pub fn dio18(&self) -> DIO18_R {
        DIO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Clears bit 19"]
    #[inline(always)]
    pub fn dio19(&self) -> DIO19_R {
        DIO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Clears bit 20"]
    #[inline(always)]
    pub fn dio20(&self) -> DIO20_R {
        DIO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Clears bit 21"]
    #[inline(always)]
    pub fn dio21(&self) -> DIO21_R {
        DIO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Clears bit 22"]
    #[inline(always)]
    pub fn dio22(&self) -> DIO22_R {
        DIO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Clears bit 23"]
    #[inline(always)]
    pub fn dio23(&self) -> DIO23_R {
        DIO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears bit 24"]
    #[inline(always)]
    pub fn dio24(&self) -> DIO24_R {
        DIO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Clears bit 25"]
    #[inline(always)]
    pub fn dio25(&self) -> DIO25_R {
        DIO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Clears bit 26"]
    #[inline(always)]
    pub fn dio26(&self) -> DIO26_R {
        DIO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Clears bit 27"]
    #[inline(always)]
    pub fn dio27(&self) -> DIO27_R {
        DIO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Clears bit 28"]
    #[inline(always)]
    pub fn dio28(&self) -> DIO28_R {
        DIO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Clears bit 29"]
    #[inline(always)]
    pub fn dio29(&self) -> DIO29_R {
        DIO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Clears bit 30"]
    #[inline(always)]
    pub fn dio30(&self) -> DIO30_R {
        DIO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Clears bit 31"]
    #[inline(always)]
    pub fn dio31(&self) -> DIO31_R {
        DIO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dio0(&mut self) -> DIO0_W<0> {
        DIO0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dio1(&mut self) -> DIO1_W<1> {
        DIO1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dio2(&mut self) -> DIO2_W<2> {
        DIO2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dio3(&mut self) -> DIO3_W<3> {
        DIO3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> DIO4_W<4> {
        DIO4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> DIO5_W<5> {
        DIO5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Clears bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> DIO6_W<6> {
        DIO6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Clears bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> DIO7_W<7> {
        DIO7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Clears bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dio8(&mut self) -> DIO8_W<8> {
        DIO8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Clears bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dio9(&mut self) -> DIO9_W<9> {
        DIO9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Clears bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dio10(&mut self) -> DIO10_W<10> {
        DIO10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Clears bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dio11(&mut self) -> DIO11_W<11> {
        DIO11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Clears bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dio12(&mut self) -> DIO12_W<12> {
        DIO12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Clears bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dio13(&mut self) -> DIO13_W<13> {
        DIO13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Clears bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dio14(&mut self) -> DIO14_W<14> {
        DIO14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Clears bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dio15(&mut self) -> DIO15_W<15> {
        DIO15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> DIO16_W<16> {
        DIO16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Clears bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> DIO17_W<17> {
        DIO17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Clears bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> DIO18_W<18> {
        DIO18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Clears bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> DIO19_W<19> {
        DIO19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Clears bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> DIO20_W<20> {
        DIO20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Clears bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> DIO21_W<21> {
        DIO21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Clears bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> DIO22_W<22> {
        DIO22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Clears bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> DIO23_W<23> {
        DIO23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> DIO24_W<24> {
        DIO24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Clears bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> DIO25_W<25> {
        DIO25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Clears bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn dio26(&mut self) -> DIO26_W<26> {
        DIO26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Clears bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn dio27(&mut self) -> DIO27_W<27> {
        DIO27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Clears bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dio28(&mut self) -> DIO28_W<28> {
        DIO28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Clears bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn dio29(&mut self) -> DIO29_W<29> {
        DIO29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Clears bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn dio30(&mut self) -> DIO30_W<30> {
        DIO30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Clears bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dio31(&mut self) -> DIO31_W<31> {
        DIO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doutclr31_0](index.html) module"]
pub struct DOUTCLR31_0_SPEC;
impl crate::RegisterSpec for DOUTCLR31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doutclr31_0::R](R) reader structure"]
impl crate::Readable for DOUTCLR31_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doutclr31_0::W](W) writer structure"]
impl crate::Writable for DOUTCLR31_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOUTCLR31_0 to value 0"]
impl crate::Resettable for DOUTCLR31_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
