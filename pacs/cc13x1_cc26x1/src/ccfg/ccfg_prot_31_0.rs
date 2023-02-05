#[doc = "Register `CCFG_PROT_31_0` reader"]
pub struct R(crate::R<CCFG_PROT_31_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_31_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_31_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_31_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_31_0` writer"]
pub struct W(crate::W<CCFG_PROT_31_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_31_0_SPEC>;
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
impl From<crate::W<CCFG_PROT_31_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_31_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_0` reader - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_0_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_0` writer - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_0_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_1` reader - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_1_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_1` writer - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_2` reader - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_2_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_2` writer - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_3` reader - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_3_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_3` writer - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_3_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_4` reader - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_4_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_4` writer - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_4_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_5` reader - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_5_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_5` writer - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_5_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_6` reader - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_6_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_6` writer - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_6_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_7` reader - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_7_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_7` writer - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_7_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_8` reader - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_8_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_8` writer - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_8_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_9` reader - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_9_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_9` writer - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_10` reader - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_10_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_10` writer - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_10_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_11` reader - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_11_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_11` writer - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_11_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_12` reader - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_12_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_12` writer - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_12_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_13` reader - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_13_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_13` writer - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_13_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_14` reader - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_14_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_14` writer - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_14_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_15` reader - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_15_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_15` writer - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_15_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_16` reader - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_16_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_16` writer - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_16_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_17` reader - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_17_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_17` writer - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_17_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_18` reader - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_18_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_18` writer - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_18_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_19` reader - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_19_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_19` writer - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_19_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_20` reader - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_20_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_20` writer - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_20_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_21` reader - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_21_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_21` writer - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_21_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_22` reader - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_22_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_22` writer - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_22_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_23` reader - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_23_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_23` writer - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_23_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_24` reader - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_24_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_24` writer - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_24_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_25` reader - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_25_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_25` writer - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_25_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_26` reader - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_26_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_26` writer - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_26_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_27` reader - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_27_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_27` writer - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_27_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_28` reader - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_28_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_28` writer - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_28_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_29` reader - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_29_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_29` writer - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_29_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_30` reader - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_30_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_30` writer - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_30_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_31` reader - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_31_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_31` writer - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_31_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_31_0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_0(&self) -> WRT_PROT_SEC_0_R {
        WRT_PROT_SEC_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_1(&self) -> WRT_PROT_SEC_1_R {
        WRT_PROT_SEC_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_2(&self) -> WRT_PROT_SEC_2_R {
        WRT_PROT_SEC_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_3(&self) -> WRT_PROT_SEC_3_R {
        WRT_PROT_SEC_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_4(&self) -> WRT_PROT_SEC_4_R {
        WRT_PROT_SEC_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_5(&self) -> WRT_PROT_SEC_5_R {
        WRT_PROT_SEC_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_6(&self) -> WRT_PROT_SEC_6_R {
        WRT_PROT_SEC_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_7(&self) -> WRT_PROT_SEC_7_R {
        WRT_PROT_SEC_7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_8(&self) -> WRT_PROT_SEC_8_R {
        WRT_PROT_SEC_8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_9(&self) -> WRT_PROT_SEC_9_R {
        WRT_PROT_SEC_9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_10(&self) -> WRT_PROT_SEC_10_R {
        WRT_PROT_SEC_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_11(&self) -> WRT_PROT_SEC_11_R {
        WRT_PROT_SEC_11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_12(&self) -> WRT_PROT_SEC_12_R {
        WRT_PROT_SEC_12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_13(&self) -> WRT_PROT_SEC_13_R {
        WRT_PROT_SEC_13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_14(&self) -> WRT_PROT_SEC_14_R {
        WRT_PROT_SEC_14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_15(&self) -> WRT_PROT_SEC_15_R {
        WRT_PROT_SEC_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_16(&self) -> WRT_PROT_SEC_16_R {
        WRT_PROT_SEC_16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_17(&self) -> WRT_PROT_SEC_17_R {
        WRT_PROT_SEC_17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_18(&self) -> WRT_PROT_SEC_18_R {
        WRT_PROT_SEC_18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_19(&self) -> WRT_PROT_SEC_19_R {
        WRT_PROT_SEC_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_20(&self) -> WRT_PROT_SEC_20_R {
        WRT_PROT_SEC_20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_21(&self) -> WRT_PROT_SEC_21_R {
        WRT_PROT_SEC_21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_22(&self) -> WRT_PROT_SEC_22_R {
        WRT_PROT_SEC_22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_23(&self) -> WRT_PROT_SEC_23_R {
        WRT_PROT_SEC_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_24(&self) -> WRT_PROT_SEC_24_R {
        WRT_PROT_SEC_24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_25(&self) -> WRT_PROT_SEC_25_R {
        WRT_PROT_SEC_25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_26(&self) -> WRT_PROT_SEC_26_R {
        WRT_PROT_SEC_26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_27(&self) -> WRT_PROT_SEC_27_R {
        WRT_PROT_SEC_27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_28(&self) -> WRT_PROT_SEC_28_R {
        WRT_PROT_SEC_28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_29(&self) -> WRT_PROT_SEC_29_R {
        WRT_PROT_SEC_29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_30(&self) -> WRT_PROT_SEC_30_R {
        WRT_PROT_SEC_30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_31(&self) -> WRT_PROT_SEC_31_R {
        WRT_PROT_SEC_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_0(&mut self) -> WRT_PROT_SEC_0_W<0> {
        WRT_PROT_SEC_0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_1(&mut self) -> WRT_PROT_SEC_1_W<1> {
        WRT_PROT_SEC_1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_2(&mut self) -> WRT_PROT_SEC_2_W<2> {
        WRT_PROT_SEC_2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_3(&mut self) -> WRT_PROT_SEC_3_W<3> {
        WRT_PROT_SEC_3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_4(&mut self) -> WRT_PROT_SEC_4_W<4> {
        WRT_PROT_SEC_4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_5(&mut self) -> WRT_PROT_SEC_5_W<5> {
        WRT_PROT_SEC_5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_6(&mut self) -> WRT_PROT_SEC_6_W<6> {
        WRT_PROT_SEC_6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_7(&mut self) -> WRT_PROT_SEC_7_W<7> {
        WRT_PROT_SEC_7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_8(&mut self) -> WRT_PROT_SEC_8_W<8> {
        WRT_PROT_SEC_8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_9(&mut self) -> WRT_PROT_SEC_9_W<9> {
        WRT_PROT_SEC_9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_10(&mut self) -> WRT_PROT_SEC_10_W<10> {
        WRT_PROT_SEC_10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_11(&mut self) -> WRT_PROT_SEC_11_W<11> {
        WRT_PROT_SEC_11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_12(&mut self) -> WRT_PROT_SEC_12_W<12> {
        WRT_PROT_SEC_12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_13(&mut self) -> WRT_PROT_SEC_13_W<13> {
        WRT_PROT_SEC_13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_14(&mut self) -> WRT_PROT_SEC_14_W<14> {
        WRT_PROT_SEC_14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_15(&mut self) -> WRT_PROT_SEC_15_W<15> {
        WRT_PROT_SEC_15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_16(&mut self) -> WRT_PROT_SEC_16_W<16> {
        WRT_PROT_SEC_16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_17(&mut self) -> WRT_PROT_SEC_17_W<17> {
        WRT_PROT_SEC_17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_18(&mut self) -> WRT_PROT_SEC_18_W<18> {
        WRT_PROT_SEC_18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_19(&mut self) -> WRT_PROT_SEC_19_W<19> {
        WRT_PROT_SEC_19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_20(&mut self) -> WRT_PROT_SEC_20_W<20> {
        WRT_PROT_SEC_20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_21(&mut self) -> WRT_PROT_SEC_21_W<21> {
        WRT_PROT_SEC_21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_22(&mut self) -> WRT_PROT_SEC_22_W<22> {
        WRT_PROT_SEC_22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_23(&mut self) -> WRT_PROT_SEC_23_W<23> {
        WRT_PROT_SEC_23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_24(&mut self) -> WRT_PROT_SEC_24_W<24> {
        WRT_PROT_SEC_24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_25(&mut self) -> WRT_PROT_SEC_25_W<25> {
        WRT_PROT_SEC_25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_26(&mut self) -> WRT_PROT_SEC_26_W<26> {
        WRT_PROT_SEC_26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_27(&mut self) -> WRT_PROT_SEC_27_W<27> {
        WRT_PROT_SEC_27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_28(&mut self) -> WRT_PROT_SEC_28_W<28> {
        WRT_PROT_SEC_28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_29(&mut self) -> WRT_PROT_SEC_29_W<29> {
        WRT_PROT_SEC_29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_30(&mut self) -> WRT_PROT_SEC_30_W<30> {
        WRT_PROT_SEC_30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_31(&mut self) -> WRT_PROT_SEC_31_W<31> {
        WRT_PROT_SEC_31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 0-31 Each bit write protects one 8KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_31_0](index.html) module"]
pub struct CCFG_PROT_31_0_SPEC;
impl crate::RegisterSpec for CCFG_PROT_31_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_31_0::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_31_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_31_0::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_31_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_PROT_31_0 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_31_0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
