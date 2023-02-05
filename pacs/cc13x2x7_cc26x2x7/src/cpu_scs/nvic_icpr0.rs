#[doc = "Register `NVIC_ICPR0` reader"]
pub struct R(crate::R<NVIC_ICPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ICPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ICPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ICPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ICPR0` writer"]
pub struct W(crate::W<NVIC_ICPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ICPR0_SPEC>;
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
impl From<crate::W<NVIC_ICPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ICPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRPEND0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type CLRPEND0_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type CLRPEND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type CLRPEND1_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type CLRPEND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type CLRPEND2_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type CLRPEND2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type CLRPEND3_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type CLRPEND3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type CLRPEND4_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type CLRPEND4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type CLRPEND5_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type CLRPEND5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type CLRPEND6_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type CLRPEND6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type CLRPEND7_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type CLRPEND7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type CLRPEND8_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type CLRPEND8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type CLRPEND9_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type CLRPEND9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type CLRPEND10_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type CLRPEND10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type CLRPEND11_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type CLRPEND11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type CLRPEND12_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type CLRPEND12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type CLRPEND13_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type CLRPEND13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type CLRPEND14_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type CLRPEND14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type CLRPEND15_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type CLRPEND15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type CLRPEND16_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type CLRPEND16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type CLRPEND17_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type CLRPEND17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type CLRPEND18_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type CLRPEND18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type CLRPEND19_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type CLRPEND19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type CLRPEND20_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type CLRPEND20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type CLRPEND21_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type CLRPEND21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type CLRPEND22_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type CLRPEND22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type CLRPEND23_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type CLRPEND23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type CLRPEND24_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type CLRPEND24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type CLRPEND25_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type CLRPEND25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type CLRPEND26_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type CLRPEND26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type CLRPEND27_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type CLRPEND27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type CLRPEND28_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type CLRPEND28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type CLRPEND29_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type CLRPEND29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type CLRPEND30_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type CLRPEND30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
#[doc = "Field `CLRPEND31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type CLRPEND31_R = crate::BitReader<bool>;
#[doc = "Field `CLRPEND31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type CLRPEND31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICPR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend0(&self) -> CLRPEND0_R {
        CLRPEND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend1(&self) -> CLRPEND1_R {
        CLRPEND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend2(&self) -> CLRPEND2_R {
        CLRPEND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend3(&self) -> CLRPEND3_R {
        CLRPEND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend4(&self) -> CLRPEND4_R {
        CLRPEND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend5(&self) -> CLRPEND5_R {
        CLRPEND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend6(&self) -> CLRPEND6_R {
        CLRPEND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend7(&self) -> CLRPEND7_R {
        CLRPEND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend8(&self) -> CLRPEND8_R {
        CLRPEND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend9(&self) -> CLRPEND9_R {
        CLRPEND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend10(&self) -> CLRPEND10_R {
        CLRPEND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend11(&self) -> CLRPEND11_R {
        CLRPEND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend12(&self) -> CLRPEND12_R {
        CLRPEND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend13(&self) -> CLRPEND13_R {
        CLRPEND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend14(&self) -> CLRPEND14_R {
        CLRPEND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend15(&self) -> CLRPEND15_R {
        CLRPEND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend16(&self) -> CLRPEND16_R {
        CLRPEND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend17(&self) -> CLRPEND17_R {
        CLRPEND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend18(&self) -> CLRPEND18_R {
        CLRPEND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend19(&self) -> CLRPEND19_R {
        CLRPEND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend20(&self) -> CLRPEND20_R {
        CLRPEND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend21(&self) -> CLRPEND21_R {
        CLRPEND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend22(&self) -> CLRPEND22_R {
        CLRPEND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend23(&self) -> CLRPEND23_R {
        CLRPEND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend24(&self) -> CLRPEND24_R {
        CLRPEND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend25(&self) -> CLRPEND25_R {
        CLRPEND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend26(&self) -> CLRPEND26_R {
        CLRPEND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend27(&self) -> CLRPEND27_R {
        CLRPEND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend28(&self) -> CLRPEND28_R {
        CLRPEND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend29(&self) -> CLRPEND29_R {
        CLRPEND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend30(&self) -> CLRPEND30_R {
        CLRPEND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn clrpend31(&self) -> CLRPEND31_R {
        CLRPEND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend0(&mut self) -> CLRPEND0_W<0> {
        CLRPEND0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend1(&mut self) -> CLRPEND1_W<1> {
        CLRPEND1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend2(&mut self) -> CLRPEND2_W<2> {
        CLRPEND2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend3(&mut self) -> CLRPEND3_W<3> {
        CLRPEND3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend4(&mut self) -> CLRPEND4_W<4> {
        CLRPEND4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend5(&mut self) -> CLRPEND5_W<5> {
        CLRPEND5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend6(&mut self) -> CLRPEND6_W<6> {
        CLRPEND6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend7(&mut self) -> CLRPEND7_W<7> {
        CLRPEND7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend8(&mut self) -> CLRPEND8_W<8> {
        CLRPEND8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend9(&mut self) -> CLRPEND9_W<9> {
        CLRPEND9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend10(&mut self) -> CLRPEND10_W<10> {
        CLRPEND10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend11(&mut self) -> CLRPEND11_W<11> {
        CLRPEND11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend12(&mut self) -> CLRPEND12_W<12> {
        CLRPEND12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend13(&mut self) -> CLRPEND13_W<13> {
        CLRPEND13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend14(&mut self) -> CLRPEND14_W<14> {
        CLRPEND14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend15(&mut self) -> CLRPEND15_W<15> {
        CLRPEND15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend16(&mut self) -> CLRPEND16_W<16> {
        CLRPEND16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend17(&mut self) -> CLRPEND17_W<17> {
        CLRPEND17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend18(&mut self) -> CLRPEND18_W<18> {
        CLRPEND18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend19(&mut self) -> CLRPEND19_W<19> {
        CLRPEND19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend20(&mut self) -> CLRPEND20_W<20> {
        CLRPEND20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend21(&mut self) -> CLRPEND21_W<21> {
        CLRPEND21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend22(&mut self) -> CLRPEND22_W<22> {
        CLRPEND22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend23(&mut self) -> CLRPEND23_W<23> {
        CLRPEND23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend24(&mut self) -> CLRPEND24_W<24> {
        CLRPEND24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend25(&mut self) -> CLRPEND25_W<25> {
        CLRPEND25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend26(&mut self) -> CLRPEND26_W<26> {
        CLRPEND26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend27(&mut self) -> CLRPEND27_W<27> {
        CLRPEND27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend28(&mut self) -> CLRPEND28_W<28> {
        CLRPEND28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend29(&mut self) -> CLRPEND29_W<29> {
        CLRPEND29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend30(&mut self) -> CLRPEND30_W<30> {
        CLRPEND30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit clears the corresponding pending interrupt 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn clrpend31(&mut self) -> CLRPEND31_W<31> {
        CLRPEND31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Clear Pending This register is used to clear pending interrupts and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icpr0](index.html) module"]
pub struct NVIC_ICPR0_SPEC;
impl crate::RegisterSpec for NVIC_ICPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_icpr0::R](R) reader structure"]
impl crate::Readable for NVIC_ICPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_icpr0::W](W) writer structure"]
impl crate::Writable for NVIC_ICPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ICPR0 to value 0"]
impl crate::Resettable for NVIC_ICPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
