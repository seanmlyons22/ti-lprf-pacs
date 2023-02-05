#[doc = "Register `NVIC_ISPR0` reader"]
pub struct R(crate::R<NVIC_ISPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISPR0` writer"]
pub struct W(crate::W<NVIC_ISPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISPR0_SPEC>;
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
impl From<crate::W<NVIC_ISPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETPEND0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type SETPEND0_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type SETPEND0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type SETPEND1_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type SETPEND1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type SETPEND2_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type SETPEND2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type SETPEND3_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type SETPEND3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type SETPEND4_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type SETPEND4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type SETPEND5_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type SETPEND5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type SETPEND6_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type SETPEND6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type SETPEND7_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type SETPEND7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type SETPEND8_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type SETPEND8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type SETPEND9_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type SETPEND9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type SETPEND10_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type SETPEND10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type SETPEND11_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type SETPEND11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type SETPEND12_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type SETPEND12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type SETPEND13_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type SETPEND13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type SETPEND14_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type SETPEND14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type SETPEND15_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type SETPEND15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type SETPEND16_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type SETPEND16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type SETPEND17_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type SETPEND17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type SETPEND18_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type SETPEND18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type SETPEND19_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type SETPEND19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type SETPEND20_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type SETPEND20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type SETPEND21_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type SETPEND21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type SETPEND22_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type SETPEND22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type SETPEND23_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type SETPEND23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type SETPEND24_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type SETPEND24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type SETPEND25_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type SETPEND25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type SETPEND26_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type SETPEND26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type SETPEND27_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type SETPEND27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type SETPEND28_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type SETPEND28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type SETPEND29_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type SETPEND29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type SETPEND30_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type SETPEND30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
#[doc = "Field `SETPEND31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type SETPEND31_R = crate::BitReader<bool>;
#[doc = "Field `SETPEND31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type SETPEND31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISPR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend0(&self) -> SETPEND0_R {
        SETPEND0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend1(&self) -> SETPEND1_R {
        SETPEND1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend2(&self) -> SETPEND2_R {
        SETPEND2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend3(&self) -> SETPEND3_R {
        SETPEND3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend4(&self) -> SETPEND4_R {
        SETPEND4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend5(&self) -> SETPEND5_R {
        SETPEND5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend6(&self) -> SETPEND6_R {
        SETPEND6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend7(&self) -> SETPEND7_R {
        SETPEND7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend8(&self) -> SETPEND8_R {
        SETPEND8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend9(&self) -> SETPEND9_R {
        SETPEND9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend10(&self) -> SETPEND10_R {
        SETPEND10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend11(&self) -> SETPEND11_R {
        SETPEND11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend12(&self) -> SETPEND12_R {
        SETPEND12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend13(&self) -> SETPEND13_R {
        SETPEND13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend14(&self) -> SETPEND14_R {
        SETPEND14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend15(&self) -> SETPEND15_R {
        SETPEND15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend16(&self) -> SETPEND16_R {
        SETPEND16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend17(&self) -> SETPEND17_R {
        SETPEND17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend18(&self) -> SETPEND18_R {
        SETPEND18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend19(&self) -> SETPEND19_R {
        SETPEND19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend20(&self) -> SETPEND20_R {
        SETPEND20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend21(&self) -> SETPEND21_R {
        SETPEND21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend22(&self) -> SETPEND22_R {
        SETPEND22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend23(&self) -> SETPEND23_R {
        SETPEND23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend24(&self) -> SETPEND24_R {
        SETPEND24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend25(&self) -> SETPEND25_R {
        SETPEND25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend26(&self) -> SETPEND26_R {
        SETPEND26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend27(&self) -> SETPEND27_R {
        SETPEND27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend28(&self) -> SETPEND28_R {
        SETPEND28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend29(&self) -> SETPEND29_R {
        SETPEND29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend30(&self) -> SETPEND30_R {
        SETPEND30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend31(&self) -> SETPEND31_R {
        SETPEND31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend0(&mut self) -> SETPEND0_W<0> {
        SETPEND0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend1(&mut self) -> SETPEND1_W<1> {
        SETPEND1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend2(&mut self) -> SETPEND2_W<2> {
        SETPEND2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend3(&mut self) -> SETPEND3_W<3> {
        SETPEND3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend4(&mut self) -> SETPEND4_W<4> {
        SETPEND4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend5(&mut self) -> SETPEND5_W<5> {
        SETPEND5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend6(&mut self) -> SETPEND6_W<6> {
        SETPEND6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend7(&mut self) -> SETPEND7_W<7> {
        SETPEND7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend8(&mut self) -> SETPEND8_W<8> {
        SETPEND8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend9(&mut self) -> SETPEND9_W<9> {
        SETPEND9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend10(&mut self) -> SETPEND10_W<10> {
        SETPEND10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend11(&mut self) -> SETPEND11_W<11> {
        SETPEND11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend12(&mut self) -> SETPEND12_W<12> {
        SETPEND12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend13(&mut self) -> SETPEND13_W<13> {
        SETPEND13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend14(&mut self) -> SETPEND14_W<14> {
        SETPEND14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend15(&mut self) -> SETPEND15_W<15> {
        SETPEND15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend16(&mut self) -> SETPEND16_W<16> {
        SETPEND16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend17(&mut self) -> SETPEND17_W<17> {
        SETPEND17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend18(&mut self) -> SETPEND18_W<18> {
        SETPEND18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend19(&mut self) -> SETPEND19_W<19> {
        SETPEND19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend20(&mut self) -> SETPEND20_W<20> {
        SETPEND20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend21(&mut self) -> SETPEND21_W<21> {
        SETPEND21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend22(&mut self) -> SETPEND22_W<22> {
        SETPEND22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend23(&mut self) -> SETPEND23_W<23> {
        SETPEND23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend24(&mut self) -> SETPEND24_W<24> {
        SETPEND24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend25(&mut self) -> SETPEND25_W<25> {
        SETPEND25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend26(&mut self) -> SETPEND26_W<26> {
        SETPEND26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend27(&mut self) -> SETPEND27_W<27> {
        SETPEND27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend28(&mut self) -> SETPEND28_W<28> {
        SETPEND28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend29(&mut self) -> SETPEND29_W<29> {
        SETPEND29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend30(&mut self) -> SETPEND30_W<30> {
        SETPEND30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend31(&mut self) -> SETPEND31_W<31> {
        SETPEND31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_ispr0](index.html) module"]
pub struct NVIC_ISPR0_SPEC;
impl crate::RegisterSpec for NVIC_ISPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ispr0::R](R) reader structure"]
impl crate::Readable for NVIC_ISPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ispr0::W](W) writer structure"]
impl crate::Writable for NVIC_ISPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR0 to value 0"]
impl crate::Resettable for NVIC_ISPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
