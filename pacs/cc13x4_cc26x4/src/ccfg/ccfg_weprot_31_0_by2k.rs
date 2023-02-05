#[doc = "Register `CCFG_WEPROT_31_0_BY2K` reader"]
pub struct R(crate::R<CCFG_WEPROT_31_0_BY2K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_WEPROT_31_0_BY2K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_WEPROT_31_0_BY2K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_WEPROT_31_0_BY2K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_WEPROT_31_0_BY2K` writer"]
pub struct W(crate::W<CCFG_WEPROT_31_0_BY2K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_WEPROT_31_0_BY2K_SPEC>;
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
impl From<crate::W<CCFG_WEPROT_31_0_BY2K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_WEPROT_31_0_BY2K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEPROT_SEC_0_N` reader - 0:0\\]
0: Sector protected"]
pub type WEPROT_SEC_0_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_0_N` writer - 0:0\\]
0: Sector protected"]
pub type WEPROT_SEC_0_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_1_N` reader - 1:1\\]
0: Sector protected"]
pub type WEPROT_SEC_1_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_1_N` writer - 1:1\\]
0: Sector protected"]
pub type WEPROT_SEC_1_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_2_N` reader - 2:2\\]
0: Sector protected"]
pub type WEPROT_SEC_2_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_2_N` writer - 2:2\\]
0: Sector protected"]
pub type WEPROT_SEC_2_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_3_N` reader - 3:3\\]
0: Sector protected"]
pub type WEPROT_SEC_3_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_3_N` writer - 3:3\\]
0: Sector protected"]
pub type WEPROT_SEC_3_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_4_N` reader - 4:4\\]
0: Sector protected"]
pub type WEPROT_SEC_4_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_4_N` writer - 4:4\\]
0: Sector protected"]
pub type WEPROT_SEC_4_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_5_N` reader - 5:5\\]
0: Sector protected"]
pub type WEPROT_SEC_5_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_5_N` writer - 5:5\\]
0: Sector protected"]
pub type WEPROT_SEC_5_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_6_N` reader - 6:6\\]
0: Sector protected"]
pub type WEPROT_SEC_6_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_6_N` writer - 6:6\\]
0: Sector protected"]
pub type WEPROT_SEC_6_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_7_N` reader - 7:7\\]
0: Sector protected"]
pub type WEPROT_SEC_7_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_7_N` writer - 7:7\\]
0: Sector protected"]
pub type WEPROT_SEC_7_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_8_N` reader - 8:8\\]
0: Sector protected"]
pub type WEPROT_SEC_8_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_8_N` writer - 8:8\\]
0: Sector protected"]
pub type WEPROT_SEC_8_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_9_N` reader - 9:9\\]
0: Sector protected"]
pub type WEPROT_SEC_9_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_9_N` writer - 9:9\\]
0: Sector protected"]
pub type WEPROT_SEC_9_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_10_N` reader - 10:10\\]
0: Sector protected"]
pub type WEPROT_SEC_10_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_10_N` writer - 10:10\\]
0: Sector protected"]
pub type WEPROT_SEC_10_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_11_N` reader - 11:11\\]
0: Sector protected"]
pub type WEPROT_SEC_11_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_11_N` writer - 11:11\\]
0: Sector protected"]
pub type WEPROT_SEC_11_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_12_N` reader - 12:12\\]
0: Sector protected"]
pub type WEPROT_SEC_12_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_12_N` writer - 12:12\\]
0: Sector protected"]
pub type WEPROT_SEC_12_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_13_N` reader - 13:13\\]
0: Sector protected"]
pub type WEPROT_SEC_13_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_13_N` writer - 13:13\\]
0: Sector protected"]
pub type WEPROT_SEC_13_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_14_N` reader - 14:14\\]
0: Sector protected"]
pub type WEPROT_SEC_14_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_14_N` writer - 14:14\\]
0: Sector protected"]
pub type WEPROT_SEC_14_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_15_N` reader - 15:15\\]
0: Sector protected"]
pub type WEPROT_SEC_15_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_15_N` writer - 15:15\\]
0: Sector protected"]
pub type WEPROT_SEC_15_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_16_N` reader - 16:16\\]
0: Sector protected"]
pub type WEPROT_SEC_16_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_16_N` writer - 16:16\\]
0: Sector protected"]
pub type WEPROT_SEC_16_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_17_N` reader - 17:17\\]
0: Sector protected"]
pub type WEPROT_SEC_17_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_17_N` writer - 17:17\\]
0: Sector protected"]
pub type WEPROT_SEC_17_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_18_N` reader - 18:18\\]
0: Sector protected"]
pub type WEPROT_SEC_18_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_18_N` writer - 18:18\\]
0: Sector protected"]
pub type WEPROT_SEC_18_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_19_N` reader - 19:19\\]
0: Sector protected"]
pub type WEPROT_SEC_19_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_19_N` writer - 19:19\\]
0: Sector protected"]
pub type WEPROT_SEC_19_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_20_N` reader - 20:20\\]
0: Sector protected"]
pub type WEPROT_SEC_20_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_20_N` writer - 20:20\\]
0: Sector protected"]
pub type WEPROT_SEC_20_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_21_N` reader - 21:21\\]
0: Sector protected"]
pub type WEPROT_SEC_21_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_21_N` writer - 21:21\\]
0: Sector protected"]
pub type WEPROT_SEC_21_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_22_N` reader - 22:22\\]
0: Sector protected"]
pub type WEPROT_SEC_22_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_22_N` writer - 22:22\\]
0: Sector protected"]
pub type WEPROT_SEC_22_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_23_N` reader - 23:23\\]
0: Sector protected"]
pub type WEPROT_SEC_23_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_23_N` writer - 23:23\\]
0: Sector protected"]
pub type WEPROT_SEC_23_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_24_N` reader - 24:24\\]
0: Sector protected"]
pub type WEPROT_SEC_24_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_24_N` writer - 24:24\\]
0: Sector protected"]
pub type WEPROT_SEC_24_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_25_N` reader - 25:25\\]
0: Sector protected"]
pub type WEPROT_SEC_25_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_25_N` writer - 25:25\\]
0: Sector protected"]
pub type WEPROT_SEC_25_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_26_N` reader - 26:26\\]
0: Sector protected"]
pub type WEPROT_SEC_26_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_26_N` writer - 26:26\\]
0: Sector protected"]
pub type WEPROT_SEC_26_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_27_N` reader - 27:27\\]
0: Sector protected"]
pub type WEPROT_SEC_27_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_27_N` writer - 27:27\\]
0: Sector protected"]
pub type WEPROT_SEC_27_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_28_N` reader - 28:28\\]
0: Sector protected"]
pub type WEPROT_SEC_28_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_28_N` writer - 28:28\\]
0: Sector protected"]
pub type WEPROT_SEC_28_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_29_N` reader - 29:29\\]
0: Sector protected"]
pub type WEPROT_SEC_29_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_29_N` writer - 29:29\\]
0: Sector protected"]
pub type WEPROT_SEC_29_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_30_N` reader - 30:30\\]
0: Sector protected"]
pub type WEPROT_SEC_30_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_30_N` writer - 30:30\\]
0: Sector protected"]
pub type WEPROT_SEC_30_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
#[doc = "Field `WEPROT_SEC_31_N` reader - 31:31\\]
0: Sector protected"]
pub type WEPROT_SEC_31_N_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_SEC_31_N` writer - 31:31\\]
0: Sector protected"]
pub type WEPROT_SEC_31_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_WEPROT_31_0_BY2K_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_0_n(&self) -> WEPROT_SEC_0_N_R {
        WEPROT_SEC_0_N_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_1_n(&self) -> WEPROT_SEC_1_N_R {
        WEPROT_SEC_1_N_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_2_n(&self) -> WEPROT_SEC_2_N_R {
        WEPROT_SEC_2_N_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_3_n(&self) -> WEPROT_SEC_3_N_R {
        WEPROT_SEC_3_N_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_4_n(&self) -> WEPROT_SEC_4_N_R {
        WEPROT_SEC_4_N_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_5_n(&self) -> WEPROT_SEC_5_N_R {
        WEPROT_SEC_5_N_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_6_n(&self) -> WEPROT_SEC_6_N_R {
        WEPROT_SEC_6_N_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_7_n(&self) -> WEPROT_SEC_7_N_R {
        WEPROT_SEC_7_N_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_8_n(&self) -> WEPROT_SEC_8_N_R {
        WEPROT_SEC_8_N_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_9_n(&self) -> WEPROT_SEC_9_N_R {
        WEPROT_SEC_9_N_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_10_n(&self) -> WEPROT_SEC_10_N_R {
        WEPROT_SEC_10_N_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_11_n(&self) -> WEPROT_SEC_11_N_R {
        WEPROT_SEC_11_N_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_12_n(&self) -> WEPROT_SEC_12_N_R {
        WEPROT_SEC_12_N_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_13_n(&self) -> WEPROT_SEC_13_N_R {
        WEPROT_SEC_13_N_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_14_n(&self) -> WEPROT_SEC_14_N_R {
        WEPROT_SEC_14_N_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_15_n(&self) -> WEPROT_SEC_15_N_R {
        WEPROT_SEC_15_N_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_16_n(&self) -> WEPROT_SEC_16_N_R {
        WEPROT_SEC_16_N_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_17_n(&self) -> WEPROT_SEC_17_N_R {
        WEPROT_SEC_17_N_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_18_n(&self) -> WEPROT_SEC_18_N_R {
        WEPROT_SEC_18_N_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_19_n(&self) -> WEPROT_SEC_19_N_R {
        WEPROT_SEC_19_N_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_20_n(&self) -> WEPROT_SEC_20_N_R {
        WEPROT_SEC_20_N_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_21_n(&self) -> WEPROT_SEC_21_N_R {
        WEPROT_SEC_21_N_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_22_n(&self) -> WEPROT_SEC_22_N_R {
        WEPROT_SEC_22_N_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_23_n(&self) -> WEPROT_SEC_23_N_R {
        WEPROT_SEC_23_N_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_24_n(&self) -> WEPROT_SEC_24_N_R {
        WEPROT_SEC_24_N_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_25_n(&self) -> WEPROT_SEC_25_N_R {
        WEPROT_SEC_25_N_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_26_n(&self) -> WEPROT_SEC_26_N_R {
        WEPROT_SEC_26_N_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_27_n(&self) -> WEPROT_SEC_27_N_R {
        WEPROT_SEC_27_N_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_28_n(&self) -> WEPROT_SEC_28_N_R {
        WEPROT_SEC_28_N_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_29_n(&self) -> WEPROT_SEC_29_N_R {
        WEPROT_SEC_29_N_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_30_n(&self) -> WEPROT_SEC_30_N_R {
        WEPROT_SEC_30_N_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn weprot_sec_31_n(&self) -> WEPROT_SEC_31_N_R {
        WEPROT_SEC_31_N_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_0_n(&mut self) -> WEPROT_SEC_0_N_W<0> {
        WEPROT_SEC_0_N_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_1_n(&mut self) -> WEPROT_SEC_1_N_W<1> {
        WEPROT_SEC_1_N_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_2_n(&mut self) -> WEPROT_SEC_2_N_W<2> {
        WEPROT_SEC_2_N_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_3_n(&mut self) -> WEPROT_SEC_3_N_W<3> {
        WEPROT_SEC_3_N_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_4_n(&mut self) -> WEPROT_SEC_4_N_W<4> {
        WEPROT_SEC_4_N_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_5_n(&mut self) -> WEPROT_SEC_5_N_W<5> {
        WEPROT_SEC_5_N_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_6_n(&mut self) -> WEPROT_SEC_6_N_W<6> {
        WEPROT_SEC_6_N_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_7_n(&mut self) -> WEPROT_SEC_7_N_W<7> {
        WEPROT_SEC_7_N_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_8_n(&mut self) -> WEPROT_SEC_8_N_W<8> {
        WEPROT_SEC_8_N_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_9_n(&mut self) -> WEPROT_SEC_9_N_W<9> {
        WEPROT_SEC_9_N_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_10_n(&mut self) -> WEPROT_SEC_10_N_W<10> {
        WEPROT_SEC_10_N_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_11_n(&mut self) -> WEPROT_SEC_11_N_W<11> {
        WEPROT_SEC_11_N_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_12_n(&mut self) -> WEPROT_SEC_12_N_W<12> {
        WEPROT_SEC_12_N_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_13_n(&mut self) -> WEPROT_SEC_13_N_W<13> {
        WEPROT_SEC_13_N_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_14_n(&mut self) -> WEPROT_SEC_14_N_W<14> {
        WEPROT_SEC_14_N_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_15_n(&mut self) -> WEPROT_SEC_15_N_W<15> {
        WEPROT_SEC_15_N_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_16_n(&mut self) -> WEPROT_SEC_16_N_W<16> {
        WEPROT_SEC_16_N_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_17_n(&mut self) -> WEPROT_SEC_17_N_W<17> {
        WEPROT_SEC_17_N_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_18_n(&mut self) -> WEPROT_SEC_18_N_W<18> {
        WEPROT_SEC_18_N_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_19_n(&mut self) -> WEPROT_SEC_19_N_W<19> {
        WEPROT_SEC_19_N_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_20_n(&mut self) -> WEPROT_SEC_20_N_W<20> {
        WEPROT_SEC_20_N_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_21_n(&mut self) -> WEPROT_SEC_21_N_W<21> {
        WEPROT_SEC_21_N_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_22_n(&mut self) -> WEPROT_SEC_22_N_W<22> {
        WEPROT_SEC_22_N_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_23_n(&mut self) -> WEPROT_SEC_23_N_W<23> {
        WEPROT_SEC_23_N_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_24_n(&mut self) -> WEPROT_SEC_24_N_W<24> {
        WEPROT_SEC_24_N_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_25_n(&mut self) -> WEPROT_SEC_25_N_W<25> {
        WEPROT_SEC_25_N_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_26_n(&mut self) -> WEPROT_SEC_26_N_W<26> {
        WEPROT_SEC_26_N_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_27_n(&mut self) -> WEPROT_SEC_27_N_W<27> {
        WEPROT_SEC_27_N_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_28_n(&mut self) -> WEPROT_SEC_28_N_W<28> {
        WEPROT_SEC_28_N_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_29_n(&mut self) -> WEPROT_SEC_29_N_W<29> {
        WEPROT_SEC_29_N_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_30_n(&mut self) -> WEPROT_SEC_30_N_W<30> {
        WEPROT_SEC_30_N_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn weprot_sec_31_n(&mut self) -> WEPROT_SEC_31_N_W<31> {
        WEPROT_SEC_31_N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 0-31 Each bit write protects one 2KB flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector WriteErase protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_weprot_31_0_by2k](index.html) module"]
pub struct CCFG_WEPROT_31_0_BY2K_SPEC;
impl crate::RegisterSpec for CCFG_WEPROT_31_0_BY2K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_weprot_31_0_by2k::R](R) reader structure"]
impl crate::Readable for CCFG_WEPROT_31_0_BY2K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_weprot_31_0_by2k::W](W) writer structure"]
impl crate::Writable for CCFG_WEPROT_31_0_BY2K_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_WEPROT_31_0_BY2K to value 0xffff_ffff"]
impl crate::Resettable for CCFG_WEPROT_31_0_BY2K_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
