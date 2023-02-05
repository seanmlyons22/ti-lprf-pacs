#[doc = "Register `CCFG_PROT_95_64` reader"]
pub struct R(crate::R<CCFG_PROT_95_64_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_95_64_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_95_64_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_95_64_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_95_64` writer"]
pub struct W(crate::W<CCFG_PROT_95_64_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_95_64_SPEC>;
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
impl From<crate::W<CCFG_PROT_95_64_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_95_64_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_64` reader - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_64_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_64` writer - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_64_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_65` reader - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_65_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_65` writer - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_65_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_66` reader - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_66_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_66` writer - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_66_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_67` reader - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_67_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_67` writer - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_67_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_68` reader - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_68_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_68` writer - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_68_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_69` reader - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_69_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_69` writer - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_69_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_70` reader - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_70_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_70` writer - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_70_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_71` reader - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_71_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_71` writer - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_71_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_72` reader - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_72_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_72` writer - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_72_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_73` reader - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_73_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_73` writer - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_73_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_74` reader - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_74_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_74` writer - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_74_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_75` reader - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_75_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_75` writer - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_75_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_76` reader - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_76_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_76` writer - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_76_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_77` reader - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_77_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_77` writer - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_77_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_78` reader - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_78_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_78` writer - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_78_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_79` reader - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_79_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_79` writer - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_79_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_80` reader - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_80_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_80` writer - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_80_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_81` reader - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_81_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_81` writer - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_81_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_82` reader - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_82_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_82` writer - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_82_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_83` reader - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_83_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_83` writer - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_83_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_84` reader - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_84_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_84` writer - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_84_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_85` reader - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_85_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_85` writer - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_85_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_86` reader - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_86_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_86` writer - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_86_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_87` reader - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_87_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_87` writer - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_87_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_88` reader - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_88_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_88` writer - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_88_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_89` reader - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_89_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_89` writer - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_89_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_90` reader - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_90_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_90` writer - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_90_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_91` reader - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_91_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_91` writer - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_91_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_92` reader - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_92_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_92` writer - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_92_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_93` reader - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_93_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_93` writer - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_93_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_94` reader - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_94_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_94` writer - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_94_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_95` reader - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_95_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_95` writer - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_95_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_95_64_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_64(&self) -> WRT_PROT_SEC_64_R {
        WRT_PROT_SEC_64_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_65(&self) -> WRT_PROT_SEC_65_R {
        WRT_PROT_SEC_65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_66(&self) -> WRT_PROT_SEC_66_R {
        WRT_PROT_SEC_66_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_67(&self) -> WRT_PROT_SEC_67_R {
        WRT_PROT_SEC_67_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_68(&self) -> WRT_PROT_SEC_68_R {
        WRT_PROT_SEC_68_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_69(&self) -> WRT_PROT_SEC_69_R {
        WRT_PROT_SEC_69_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_70(&self) -> WRT_PROT_SEC_70_R {
        WRT_PROT_SEC_70_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_71(&self) -> WRT_PROT_SEC_71_R {
        WRT_PROT_SEC_71_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_72(&self) -> WRT_PROT_SEC_72_R {
        WRT_PROT_SEC_72_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_73(&self) -> WRT_PROT_SEC_73_R {
        WRT_PROT_SEC_73_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_74(&self) -> WRT_PROT_SEC_74_R {
        WRT_PROT_SEC_74_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_75(&self) -> WRT_PROT_SEC_75_R {
        WRT_PROT_SEC_75_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_76(&self) -> WRT_PROT_SEC_76_R {
        WRT_PROT_SEC_76_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_77(&self) -> WRT_PROT_SEC_77_R {
        WRT_PROT_SEC_77_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_78(&self) -> WRT_PROT_SEC_78_R {
        WRT_PROT_SEC_78_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_79(&self) -> WRT_PROT_SEC_79_R {
        WRT_PROT_SEC_79_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_80(&self) -> WRT_PROT_SEC_80_R {
        WRT_PROT_SEC_80_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_81(&self) -> WRT_PROT_SEC_81_R {
        WRT_PROT_SEC_81_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_82(&self) -> WRT_PROT_SEC_82_R {
        WRT_PROT_SEC_82_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_83(&self) -> WRT_PROT_SEC_83_R {
        WRT_PROT_SEC_83_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_84(&self) -> WRT_PROT_SEC_84_R {
        WRT_PROT_SEC_84_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_85(&self) -> WRT_PROT_SEC_85_R {
        WRT_PROT_SEC_85_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_86(&self) -> WRT_PROT_SEC_86_R {
        WRT_PROT_SEC_86_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_87(&self) -> WRT_PROT_SEC_87_R {
        WRT_PROT_SEC_87_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_88(&self) -> WRT_PROT_SEC_88_R {
        WRT_PROT_SEC_88_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_89(&self) -> WRT_PROT_SEC_89_R {
        WRT_PROT_SEC_89_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_90(&self) -> WRT_PROT_SEC_90_R {
        WRT_PROT_SEC_90_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_91(&self) -> WRT_PROT_SEC_91_R {
        WRT_PROT_SEC_91_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_92(&self) -> WRT_PROT_SEC_92_R {
        WRT_PROT_SEC_92_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_93(&self) -> WRT_PROT_SEC_93_R {
        WRT_PROT_SEC_93_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_94(&self) -> WRT_PROT_SEC_94_R {
        WRT_PROT_SEC_94_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_95(&self) -> WRT_PROT_SEC_95_R {
        WRT_PROT_SEC_95_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_64(&mut self) -> WRT_PROT_SEC_64_W<0> {
        WRT_PROT_SEC_64_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_65(&mut self) -> WRT_PROT_SEC_65_W<1> {
        WRT_PROT_SEC_65_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_66(&mut self) -> WRT_PROT_SEC_66_W<2> {
        WRT_PROT_SEC_66_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_67(&mut self) -> WRT_PROT_SEC_67_W<3> {
        WRT_PROT_SEC_67_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_68(&mut self) -> WRT_PROT_SEC_68_W<4> {
        WRT_PROT_SEC_68_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_69(&mut self) -> WRT_PROT_SEC_69_W<5> {
        WRT_PROT_SEC_69_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_70(&mut self) -> WRT_PROT_SEC_70_W<6> {
        WRT_PROT_SEC_70_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_71(&mut self) -> WRT_PROT_SEC_71_W<7> {
        WRT_PROT_SEC_71_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_72(&mut self) -> WRT_PROT_SEC_72_W<8> {
        WRT_PROT_SEC_72_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_73(&mut self) -> WRT_PROT_SEC_73_W<9> {
        WRT_PROT_SEC_73_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_74(&mut self) -> WRT_PROT_SEC_74_W<10> {
        WRT_PROT_SEC_74_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_75(&mut self) -> WRT_PROT_SEC_75_W<11> {
        WRT_PROT_SEC_75_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_76(&mut self) -> WRT_PROT_SEC_76_W<12> {
        WRT_PROT_SEC_76_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_77(&mut self) -> WRT_PROT_SEC_77_W<13> {
        WRT_PROT_SEC_77_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_78(&mut self) -> WRT_PROT_SEC_78_W<14> {
        WRT_PROT_SEC_78_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_79(&mut self) -> WRT_PROT_SEC_79_W<15> {
        WRT_PROT_SEC_79_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_80(&mut self) -> WRT_PROT_SEC_80_W<16> {
        WRT_PROT_SEC_80_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_81(&mut self) -> WRT_PROT_SEC_81_W<17> {
        WRT_PROT_SEC_81_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_82(&mut self) -> WRT_PROT_SEC_82_W<18> {
        WRT_PROT_SEC_82_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_83(&mut self) -> WRT_PROT_SEC_83_W<19> {
        WRT_PROT_SEC_83_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_84(&mut self) -> WRT_PROT_SEC_84_W<20> {
        WRT_PROT_SEC_84_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_85(&mut self) -> WRT_PROT_SEC_85_W<21> {
        WRT_PROT_SEC_85_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_86(&mut self) -> WRT_PROT_SEC_86_W<22> {
        WRT_PROT_SEC_86_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_87(&mut self) -> WRT_PROT_SEC_87_W<23> {
        WRT_PROT_SEC_87_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_88(&mut self) -> WRT_PROT_SEC_88_W<24> {
        WRT_PROT_SEC_88_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_89(&mut self) -> WRT_PROT_SEC_89_W<25> {
        WRT_PROT_SEC_89_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_90(&mut self) -> WRT_PROT_SEC_90_W<26> {
        WRT_PROT_SEC_90_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_91(&mut self) -> WRT_PROT_SEC_91_W<27> {
        WRT_PROT_SEC_91_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_92(&mut self) -> WRT_PROT_SEC_92_W<28> {
        WRT_PROT_SEC_92_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_93(&mut self) -> WRT_PROT_SEC_93_W<29> {
        WRT_PROT_SEC_93_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_94(&mut self) -> WRT_PROT_SEC_94_W<30> {
        WRT_PROT_SEC_94_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_95(&mut self) -> WRT_PROT_SEC_95_W<31> {
        WRT_PROT_SEC_95_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 64-95 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_95_64](index.html) module"]
pub struct CCFG_PROT_95_64_SPEC;
impl crate::RegisterSpec for CCFG_PROT_95_64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_95_64::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_95_64_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_95_64::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_95_64_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_PROT_95_64 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_95_64_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
