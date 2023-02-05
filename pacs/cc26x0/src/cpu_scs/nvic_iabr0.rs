#[doc = "Register `NVIC_IABR0` reader"]
pub struct R(crate::R<NVIC_IABR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IABR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IABR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IABR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IABR0` writer"]
pub struct W(crate::W<NVIC_IABR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IABR0_SPEC>;
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
impl From<crate::W<NVIC_IABR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IABR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTIVE0` reader - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub type ACTIVE0_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE0` writer - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub type ACTIVE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE1` reader - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub type ACTIVE1_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE1` writer - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub type ACTIVE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE2` reader - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub type ACTIVE2_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE2` writer - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub type ACTIVE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE3` reader - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub type ACTIVE3_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE3` writer - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub type ACTIVE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE4` reader - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub type ACTIVE4_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE4` writer - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub type ACTIVE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE5` reader - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub type ACTIVE5_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE5` writer - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub type ACTIVE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE6` reader - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub type ACTIVE6_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE6` writer - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub type ACTIVE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE7` reader - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub type ACTIVE7_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE7` writer - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub type ACTIVE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE8` reader - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub type ACTIVE8_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE8` writer - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub type ACTIVE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE9` reader - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub type ACTIVE9_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE9` writer - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub type ACTIVE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE10` reader - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub type ACTIVE10_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE10` writer - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub type ACTIVE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE11` reader - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub type ACTIVE11_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE11` writer - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub type ACTIVE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE12` reader - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub type ACTIVE12_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE12` writer - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub type ACTIVE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE13` reader - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub type ACTIVE13_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE13` writer - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub type ACTIVE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE14` reader - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub type ACTIVE14_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE14` writer - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub type ACTIVE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE15` reader - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub type ACTIVE15_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE15` writer - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub type ACTIVE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE16` reader - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub type ACTIVE16_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE16` writer - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub type ACTIVE16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE17` reader - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub type ACTIVE17_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE17` writer - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub type ACTIVE17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE18` reader - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub type ACTIVE18_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE18` writer - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub type ACTIVE18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE19` reader - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub type ACTIVE19_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE19` writer - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub type ACTIVE19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE20` reader - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub type ACTIVE20_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE20` writer - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub type ACTIVE20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE21` reader - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub type ACTIVE21_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE21` writer - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub type ACTIVE21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE22` reader - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub type ACTIVE22_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE22` writer - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub type ACTIVE22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE23` reader - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub type ACTIVE23_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE23` writer - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub type ACTIVE23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE24` reader - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub type ACTIVE24_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE24` writer - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub type ACTIVE24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE25` reader - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub type ACTIVE25_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE25` writer - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub type ACTIVE25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE26` reader - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub type ACTIVE26_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE26` writer - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub type ACTIVE26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE27` reader - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub type ACTIVE27_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE27` writer - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub type ACTIVE27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE28` reader - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub type ACTIVE28_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE28` writer - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub type ACTIVE28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE29` reader - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub type ACTIVE29_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE29` writer - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub type ACTIVE29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE30` reader - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub type ACTIVE30_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE30` writer - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub type ACTIVE30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
#[doc = "Field `ACTIVE31` reader - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub type ACTIVE31_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE31` writer - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub type ACTIVE31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_IABR0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub fn active0(&self) -> ACTIVE0_R {
        ACTIVE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub fn active1(&self) -> ACTIVE1_R {
        ACTIVE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub fn active2(&self) -> ACTIVE2_R {
        ACTIVE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub fn active3(&self) -> ACTIVE3_R {
        ACTIVE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn active4(&self) -> ACTIVE4_R {
        ACTIVE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn active5(&self) -> ACTIVE5_R {
        ACTIVE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn active6(&self) -> ACTIVE6_R {
        ACTIVE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn active7(&self) -> ACTIVE7_R {
        ACTIVE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn active8(&self) -> ACTIVE8_R {
        ACTIVE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn active9(&self) -> ACTIVE9_R {
        ACTIVE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn active10(&self) -> ACTIVE10_R {
        ACTIVE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn active11(&self) -> ACTIVE11_R {
        ACTIVE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn active12(&self) -> ACTIVE12_R {
        ACTIVE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn active13(&self) -> ACTIVE13_R {
        ACTIVE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn active14(&self) -> ACTIVE14_R {
        ACTIVE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn active15(&self) -> ACTIVE15_R {
        ACTIVE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn active16(&self) -> ACTIVE16_R {
        ACTIVE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn active17(&self) -> ACTIVE17_R {
        ACTIVE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn active18(&self) -> ACTIVE18_R {
        ACTIVE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn active19(&self) -> ACTIVE19_R {
        ACTIVE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn active20(&self) -> ACTIVE20_R {
        ACTIVE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn active21(&self) -> ACTIVE21_R {
        ACTIVE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn active22(&self) -> ACTIVE22_R {
        ACTIVE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn active23(&self) -> ACTIVE23_R {
        ACTIVE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn active24(&self) -> ACTIVE24_R {
        ACTIVE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn active25(&self) -> ACTIVE25_R {
        ACTIVE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn active26(&self) -> ACTIVE26_R {
        ACTIVE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn active27(&self) -> ACTIVE27_R {
        ACTIVE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn active28(&self) -> ACTIVE28_R {
        ACTIVE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn active29(&self) -> ACTIVE29_R {
        ACTIVE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn active30(&self) -> ACTIVE30_R {
        ACTIVE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn active31(&self) -> ACTIVE31_R {
        ACTIVE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active0(&mut self) -> ACTIVE0_W<0> {
        ACTIVE0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active1(&mut self) -> ACTIVE1_W<1> {
        ACTIVE1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active2(&mut self) -> ACTIVE2_W<2> {
        ACTIVE2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active3(&mut self) -> ACTIVE3_W<3> {
        ACTIVE3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active4(&mut self) -> ACTIVE4_W<4> {
        ACTIVE4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active5(&mut self) -> ACTIVE5_W<5> {
        ACTIVE5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active6(&mut self) -> ACTIVE6_W<6> {
        ACTIVE6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active7(&mut self) -> ACTIVE7_W<7> {
        ACTIVE7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active8(&mut self) -> ACTIVE8_W<8> {
        ACTIVE8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active9(&mut self) -> ACTIVE9_W<9> {
        ACTIVE9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active10(&mut self) -> ACTIVE10_W<10> {
        ACTIVE10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active11(&mut self) -> ACTIVE11_W<11> {
        ACTIVE11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active12(&mut self) -> ACTIVE12_W<12> {
        ACTIVE12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active13(&mut self) -> ACTIVE13_W<13> {
        ACTIVE13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active14(&mut self) -> ACTIVE14_W<14> {
        ACTIVE14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active15(&mut self) -> ACTIVE15_W<15> {
        ACTIVE15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active16(&mut self) -> ACTIVE16_W<16> {
        ACTIVE16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active17(&mut self) -> ACTIVE17_W<17> {
        ACTIVE17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active18(&mut self) -> ACTIVE18_W<18> {
        ACTIVE18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active19(&mut self) -> ACTIVE19_W<19> {
        ACTIVE19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active20(&mut self) -> ACTIVE20_W<20> {
        ACTIVE20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active21(&mut self) -> ACTIVE21_W<21> {
        ACTIVE21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active22(&mut self) -> ACTIVE22_W<22> {
        ACTIVE22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active23(&mut self) -> ACTIVE23_W<23> {
        ACTIVE23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active24(&mut self) -> ACTIVE24_W<24> {
        ACTIVE24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active25(&mut self) -> ACTIVE25_W<25> {
        ACTIVE25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active26(&mut self) -> ACTIVE26_W<26> {
        ACTIVE26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active27(&mut self) -> ACTIVE27_W<27> {
        ACTIVE27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active28(&mut self) -> ACTIVE28_W<28> {
        ACTIVE28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active29(&mut self) -> ACTIVE29_W<29> {
        ACTIVE29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active30(&mut self) -> ACTIVE30_W<30> {
        ACTIVE30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active31(&mut self) -> ACTIVE31_W<31> {
        ACTIVE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iabr0](index.html) module"]
pub struct NVIC_IABR0_SPEC;
impl crate::RegisterSpec for NVIC_IABR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iabr0::R](R) reader structure"]
impl crate::Readable for NVIC_IABR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iabr0::W](W) writer structure"]
impl crate::Writable for NVIC_IABR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IABR0 to value 0"]
impl crate::Resettable for NVIC_IABR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
