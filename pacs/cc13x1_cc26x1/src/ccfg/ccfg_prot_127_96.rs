#[doc = "Register `CCFG_PROT_127_96` reader"]
pub struct R(crate::R<CCFG_PROT_127_96_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCFG_PROT_127_96_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCFG_PROT_127_96_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCFG_PROT_127_96_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCFG_PROT_127_96` writer"]
pub struct W(crate::W<CCFG_PROT_127_96_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCFG_PROT_127_96_SPEC>;
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
impl From<crate::W<CCFG_PROT_127_96_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCFG_PROT_127_96_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRT_PROT_SEC_96` reader - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_96_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_96` writer - 0:0\\]
0: Sector protected"]
pub type WRT_PROT_SEC_96_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_97` reader - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_97_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_97` writer - 1:1\\]
0: Sector protected"]
pub type WRT_PROT_SEC_97_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_98` reader - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_98_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_98` writer - 2:2\\]
0: Sector protected"]
pub type WRT_PROT_SEC_98_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_99` reader - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_99_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_99` writer - 3:3\\]
0: Sector protected"]
pub type WRT_PROT_SEC_99_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_100` reader - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_100_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_100` writer - 4:4\\]
0: Sector protected"]
pub type WRT_PROT_SEC_100_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_101` reader - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_101_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_101` writer - 5:5\\]
0: Sector protected"]
pub type WRT_PROT_SEC_101_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_102` reader - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_102_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_102` writer - 6:6\\]
0: Sector protected"]
pub type WRT_PROT_SEC_102_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_103` reader - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_103_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_103` writer - 7:7\\]
0: Sector protected"]
pub type WRT_PROT_SEC_103_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_104` reader - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_104_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_104` writer - 8:8\\]
0: Sector protected"]
pub type WRT_PROT_SEC_104_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_105` reader - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_105_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_105` writer - 9:9\\]
0: Sector protected"]
pub type WRT_PROT_SEC_105_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_106` reader - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_106_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_106` writer - 10:10\\]
0: Sector protected"]
pub type WRT_PROT_SEC_106_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_107` reader - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_107_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_107` writer - 11:11\\]
0: Sector protected"]
pub type WRT_PROT_SEC_107_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_108` reader - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_108_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_108` writer - 12:12\\]
0: Sector protected"]
pub type WRT_PROT_SEC_108_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_109` reader - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_109_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_109` writer - 13:13\\]
0: Sector protected"]
pub type WRT_PROT_SEC_109_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_110` reader - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_110_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_110` writer - 14:14\\]
0: Sector protected"]
pub type WRT_PROT_SEC_110_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_111` reader - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_111_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_111` writer - 15:15\\]
0: Sector protected"]
pub type WRT_PROT_SEC_111_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_112` reader - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_112_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_112` writer - 16:16\\]
0: Sector protected"]
pub type WRT_PROT_SEC_112_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_113` reader - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_113_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_113` writer - 17:17\\]
0: Sector protected"]
pub type WRT_PROT_SEC_113_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_114` reader - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_114_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_114` writer - 18:18\\]
0: Sector protected"]
pub type WRT_PROT_SEC_114_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_115` reader - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_115_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_115` writer - 19:19\\]
0: Sector protected"]
pub type WRT_PROT_SEC_115_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_116` reader - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_116_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_116` writer - 20:20\\]
0: Sector protected"]
pub type WRT_PROT_SEC_116_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_117` reader - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_117_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_117` writer - 21:21\\]
0: Sector protected"]
pub type WRT_PROT_SEC_117_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_118` reader - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_118_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_118` writer - 22:22\\]
0: Sector protected"]
pub type WRT_PROT_SEC_118_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_119` reader - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_119_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_119` writer - 23:23\\]
0: Sector protected"]
pub type WRT_PROT_SEC_119_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_120` reader - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_120_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_120` writer - 24:24\\]
0: Sector protected"]
pub type WRT_PROT_SEC_120_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_121` reader - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_121_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_121` writer - 25:25\\]
0: Sector protected"]
pub type WRT_PROT_SEC_121_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_122` reader - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_122_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_122` writer - 26:26\\]
0: Sector protected"]
pub type WRT_PROT_SEC_122_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_123` reader - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_123_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_123` writer - 27:27\\]
0: Sector protected"]
pub type WRT_PROT_SEC_123_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_124` reader - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_124_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_124` writer - 28:28\\]
0: Sector protected"]
pub type WRT_PROT_SEC_124_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_125` reader - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_125_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_125` writer - 29:29\\]
0: Sector protected"]
pub type WRT_PROT_SEC_125_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_126` reader - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_126_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_126` writer - 30:30\\]
0: Sector protected"]
pub type WRT_PROT_SEC_126_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
#[doc = "Field `WRT_PROT_SEC_127` reader - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_127_R = crate::BitReader<bool>;
#[doc = "Field `WRT_PROT_SEC_127` writer - 31:31\\]
0: Sector protected"]
pub type WRT_PROT_SEC_127_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCFG_PROT_127_96_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_96(&self) -> WRT_PROT_SEC_96_R {
        WRT_PROT_SEC_96_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_97(&self) -> WRT_PROT_SEC_97_R {
        WRT_PROT_SEC_97_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_98(&self) -> WRT_PROT_SEC_98_R {
        WRT_PROT_SEC_98_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_99(&self) -> WRT_PROT_SEC_99_R {
        WRT_PROT_SEC_99_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_100(&self) -> WRT_PROT_SEC_100_R {
        WRT_PROT_SEC_100_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_101(&self) -> WRT_PROT_SEC_101_R {
        WRT_PROT_SEC_101_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_102(&self) -> WRT_PROT_SEC_102_R {
        WRT_PROT_SEC_102_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_103(&self) -> WRT_PROT_SEC_103_R {
        WRT_PROT_SEC_103_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_104(&self) -> WRT_PROT_SEC_104_R {
        WRT_PROT_SEC_104_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_105(&self) -> WRT_PROT_SEC_105_R {
        WRT_PROT_SEC_105_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_106(&self) -> WRT_PROT_SEC_106_R {
        WRT_PROT_SEC_106_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_107(&self) -> WRT_PROT_SEC_107_R {
        WRT_PROT_SEC_107_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_108(&self) -> WRT_PROT_SEC_108_R {
        WRT_PROT_SEC_108_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_109(&self) -> WRT_PROT_SEC_109_R {
        WRT_PROT_SEC_109_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_110(&self) -> WRT_PROT_SEC_110_R {
        WRT_PROT_SEC_110_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_111(&self) -> WRT_PROT_SEC_111_R {
        WRT_PROT_SEC_111_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_112(&self) -> WRT_PROT_SEC_112_R {
        WRT_PROT_SEC_112_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_113(&self) -> WRT_PROT_SEC_113_R {
        WRT_PROT_SEC_113_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_114(&self) -> WRT_PROT_SEC_114_R {
        WRT_PROT_SEC_114_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_115(&self) -> WRT_PROT_SEC_115_R {
        WRT_PROT_SEC_115_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_116(&self) -> WRT_PROT_SEC_116_R {
        WRT_PROT_SEC_116_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_117(&self) -> WRT_PROT_SEC_117_R {
        WRT_PROT_SEC_117_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_118(&self) -> WRT_PROT_SEC_118_R {
        WRT_PROT_SEC_118_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_119(&self) -> WRT_PROT_SEC_119_R {
        WRT_PROT_SEC_119_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_120(&self) -> WRT_PROT_SEC_120_R {
        WRT_PROT_SEC_120_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_121(&self) -> WRT_PROT_SEC_121_R {
        WRT_PROT_SEC_121_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_122(&self) -> WRT_PROT_SEC_122_R {
        WRT_PROT_SEC_122_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_123(&self) -> WRT_PROT_SEC_123_R {
        WRT_PROT_SEC_123_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_124(&self) -> WRT_PROT_SEC_124_R {
        WRT_PROT_SEC_124_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_125(&self) -> WRT_PROT_SEC_125_R {
        WRT_PROT_SEC_125_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_126(&self) -> WRT_PROT_SEC_126_R {
        WRT_PROT_SEC_126_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    pub fn wrt_prot_sec_127(&self) -> WRT_PROT_SEC_127_R {
        WRT_PROT_SEC_127_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_96(&mut self) -> WRT_PROT_SEC_96_W<0> {
        WRT_PROT_SEC_96_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_97(&mut self) -> WRT_PROT_SEC_97_W<1> {
        WRT_PROT_SEC_97_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_98(&mut self) -> WRT_PROT_SEC_98_W<2> {
        WRT_PROT_SEC_98_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_99(&mut self) -> WRT_PROT_SEC_99_W<3> {
        WRT_PROT_SEC_99_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_100(&mut self) -> WRT_PROT_SEC_100_W<4> {
        WRT_PROT_SEC_100_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_101(&mut self) -> WRT_PROT_SEC_101_W<5> {
        WRT_PROT_SEC_101_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_102(&mut self) -> WRT_PROT_SEC_102_W<6> {
        WRT_PROT_SEC_102_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_103(&mut self) -> WRT_PROT_SEC_103_W<7> {
        WRT_PROT_SEC_103_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_104(&mut self) -> WRT_PROT_SEC_104_W<8> {
        WRT_PROT_SEC_104_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_105(&mut self) -> WRT_PROT_SEC_105_W<9> {
        WRT_PROT_SEC_105_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_106(&mut self) -> WRT_PROT_SEC_106_W<10> {
        WRT_PROT_SEC_106_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_107(&mut self) -> WRT_PROT_SEC_107_W<11> {
        WRT_PROT_SEC_107_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_108(&mut self) -> WRT_PROT_SEC_108_W<12> {
        WRT_PROT_SEC_108_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_109(&mut self) -> WRT_PROT_SEC_109_W<13> {
        WRT_PROT_SEC_109_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_110(&mut self) -> WRT_PROT_SEC_110_W<14> {
        WRT_PROT_SEC_110_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_111(&mut self) -> WRT_PROT_SEC_111_W<15> {
        WRT_PROT_SEC_111_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_112(&mut self) -> WRT_PROT_SEC_112_W<16> {
        WRT_PROT_SEC_112_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_113(&mut self) -> WRT_PROT_SEC_113_W<17> {
        WRT_PROT_SEC_113_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_114(&mut self) -> WRT_PROT_SEC_114_W<18> {
        WRT_PROT_SEC_114_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_115(&mut self) -> WRT_PROT_SEC_115_W<19> {
        WRT_PROT_SEC_115_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_116(&mut self) -> WRT_PROT_SEC_116_W<20> {
        WRT_PROT_SEC_116_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_117(&mut self) -> WRT_PROT_SEC_117_W<21> {
        WRT_PROT_SEC_117_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_118(&mut self) -> WRT_PROT_SEC_118_W<22> {
        WRT_PROT_SEC_118_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_119(&mut self) -> WRT_PROT_SEC_119_W<23> {
        WRT_PROT_SEC_119_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_120(&mut self) -> WRT_PROT_SEC_120_W<24> {
        WRT_PROT_SEC_120_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_121(&mut self) -> WRT_PROT_SEC_121_W<25> {
        WRT_PROT_SEC_121_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_122(&mut self) -> WRT_PROT_SEC_122_W<26> {
        WRT_PROT_SEC_122_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_123(&mut self) -> WRT_PROT_SEC_123_W<27> {
        WRT_PROT_SEC_123_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_124(&mut self) -> WRT_PROT_SEC_124_W<28> {
        WRT_PROT_SEC_124_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_125(&mut self) -> WRT_PROT_SEC_125_W<29> {
        WRT_PROT_SEC_125_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_126(&mut self) -> WRT_PROT_SEC_126_W<30> {
        WRT_PROT_SEC_126_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
0: Sector protected"]
    #[inline(always)]
    #[must_use]
    pub fn wrt_prot_sec_127(&mut self) -> WRT_PROT_SEC_127_W<31> {
        WRT_PROT_SEC_127_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Protect Sectors 96-127 Each bit write protects one flash sector from being both programmed and erased. Bit must be set to 0 in order to enable sector write protect. Not in use.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccfg_prot_127_96](index.html) module"]
pub struct CCFG_PROT_127_96_SPEC;
impl crate::RegisterSpec for CCFG_PROT_127_96_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccfg_prot_127_96::R](R) reader structure"]
impl crate::Readable for CCFG_PROT_127_96_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccfg_prot_127_96::W](W) writer structure"]
impl crate::Writable for CCFG_PROT_127_96_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCFG_PROT_127_96 to value 0xffff_ffff"]
impl crate::Resettable for CCFG_PROT_127_96_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
