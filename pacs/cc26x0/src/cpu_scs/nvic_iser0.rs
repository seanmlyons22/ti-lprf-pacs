#[doc = "Register `NVIC_ISER0` reader"]
pub struct R(crate::R<NVIC_ISER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ISER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ISER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ISER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ISER0` writer"]
pub struct W(crate::W<NVIC_ISER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ISER0_SPEC>;
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
impl From<crate::W<NVIC_ISER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ISER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETENA0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type SETENA0_R = crate::BitReader<bool>;
#[doc = "Field `SETENA0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type SETENA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type SETENA1_R = crate::BitReader<bool>;
#[doc = "Field `SETENA1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type SETENA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type SETENA2_R = crate::BitReader<bool>;
#[doc = "Field `SETENA2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type SETENA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type SETENA3_R = crate::BitReader<bool>;
#[doc = "Field `SETENA3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type SETENA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type SETENA4_R = crate::BitReader<bool>;
#[doc = "Field `SETENA4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type SETENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type SETENA5_R = crate::BitReader<bool>;
#[doc = "Field `SETENA5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type SETENA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type SETENA6_R = crate::BitReader<bool>;
#[doc = "Field `SETENA6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type SETENA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type SETENA7_R = crate::BitReader<bool>;
#[doc = "Field `SETENA7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type SETENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type SETENA8_R = crate::BitReader<bool>;
#[doc = "Field `SETENA8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type SETENA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type SETENA9_R = crate::BitReader<bool>;
#[doc = "Field `SETENA9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type SETENA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type SETENA10_R = crate::BitReader<bool>;
#[doc = "Field `SETENA10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type SETENA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type SETENA11_R = crate::BitReader<bool>;
#[doc = "Field `SETENA11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type SETENA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type SETENA12_R = crate::BitReader<bool>;
#[doc = "Field `SETENA12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type SETENA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type SETENA13_R = crate::BitReader<bool>;
#[doc = "Field `SETENA13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type SETENA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type SETENA14_R = crate::BitReader<bool>;
#[doc = "Field `SETENA14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type SETENA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type SETENA15_R = crate::BitReader<bool>;
#[doc = "Field `SETENA15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type SETENA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type SETENA16_R = crate::BitReader<bool>;
#[doc = "Field `SETENA16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type SETENA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type SETENA17_R = crate::BitReader<bool>;
#[doc = "Field `SETENA17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type SETENA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type SETENA18_R = crate::BitReader<bool>;
#[doc = "Field `SETENA18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type SETENA18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type SETENA19_R = crate::BitReader<bool>;
#[doc = "Field `SETENA19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type SETENA19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type SETENA20_R = crate::BitReader<bool>;
#[doc = "Field `SETENA20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type SETENA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type SETENA21_R = crate::BitReader<bool>;
#[doc = "Field `SETENA21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type SETENA21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type SETENA22_R = crate::BitReader<bool>;
#[doc = "Field `SETENA22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type SETENA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type SETENA23_R = crate::BitReader<bool>;
#[doc = "Field `SETENA23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type SETENA23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type SETENA24_R = crate::BitReader<bool>;
#[doc = "Field `SETENA24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type SETENA24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type SETENA25_R = crate::BitReader<bool>;
#[doc = "Field `SETENA25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type SETENA25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type SETENA26_R = crate::BitReader<bool>;
#[doc = "Field `SETENA26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type SETENA26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type SETENA27_R = crate::BitReader<bool>;
#[doc = "Field `SETENA27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type SETENA27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type SETENA28_R = crate::BitReader<bool>;
#[doc = "Field `SETENA28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type SETENA28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type SETENA29_R = crate::BitReader<bool>;
#[doc = "Field `SETENA29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type SETENA29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type SETENA30_R = crate::BitReader<bool>;
#[doc = "Field `SETENA30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type SETENA30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
#[doc = "Field `SETENA31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type SETENA31_R = crate::BitReader<bool>;
#[doc = "Field `SETENA31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type SETENA31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ISER0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena0(&self) -> SETENA0_R {
        SETENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena1(&self) -> SETENA1_R {
        SETENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena2(&self) -> SETENA2_R {
        SETENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena3(&self) -> SETENA3_R {
        SETENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena4(&self) -> SETENA4_R {
        SETENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena5(&self) -> SETENA5_R {
        SETENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena6(&self) -> SETENA6_R {
        SETENA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena7(&self) -> SETENA7_R {
        SETENA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena8(&self) -> SETENA8_R {
        SETENA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena9(&self) -> SETENA9_R {
        SETENA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena10(&self) -> SETENA10_R {
        SETENA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena11(&self) -> SETENA11_R {
        SETENA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena12(&self) -> SETENA12_R {
        SETENA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena13(&self) -> SETENA13_R {
        SETENA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena14(&self) -> SETENA14_R {
        SETENA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena15(&self) -> SETENA15_R {
        SETENA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena16(&self) -> SETENA16_R {
        SETENA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena17(&self) -> SETENA17_R {
        SETENA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena18(&self) -> SETENA18_R {
        SETENA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena19(&self) -> SETENA19_R {
        SETENA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena20(&self) -> SETENA20_R {
        SETENA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena21(&self) -> SETENA21_R {
        SETENA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena22(&self) -> SETENA22_R {
        SETENA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena23(&self) -> SETENA23_R {
        SETENA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena24(&self) -> SETENA24_R {
        SETENA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena25(&self) -> SETENA25_R {
        SETENA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena26(&self) -> SETENA26_R {
        SETENA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena27(&self) -> SETENA27_R {
        SETENA27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena28(&self) -> SETENA28_R {
        SETENA28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena29(&self) -> SETENA29_R {
        SETENA29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena30(&self) -> SETENA30_R {
        SETENA30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena31(&self) -> SETENA31_R {
        SETENA31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena0(&mut self) -> SETENA0_W<0> {
        SETENA0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena1(&mut self) -> SETENA1_W<1> {
        SETENA1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena2(&mut self) -> SETENA2_W<2> {
        SETENA2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena3(&mut self) -> SETENA3_W<3> {
        SETENA3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena4(&mut self) -> SETENA4_W<4> {
        SETENA4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena5(&mut self) -> SETENA5_W<5> {
        SETENA5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena6(&mut self) -> SETENA6_W<6> {
        SETENA6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena7(&mut self) -> SETENA7_W<7> {
        SETENA7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena8(&mut self) -> SETENA8_W<8> {
        SETENA8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena9(&mut self) -> SETENA9_W<9> {
        SETENA9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena10(&mut self) -> SETENA10_W<10> {
        SETENA10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena11(&mut self) -> SETENA11_W<11> {
        SETENA11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena12(&mut self) -> SETENA12_W<12> {
        SETENA12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena13(&mut self) -> SETENA13_W<13> {
        SETENA13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena14(&mut self) -> SETENA14_W<14> {
        SETENA14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena15(&mut self) -> SETENA15_W<15> {
        SETENA15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena16(&mut self) -> SETENA16_W<16> {
        SETENA16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena17(&mut self) -> SETENA17_W<17> {
        SETENA17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena18(&mut self) -> SETENA18_W<18> {
        SETENA18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena19(&mut self) -> SETENA19_W<19> {
        SETENA19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena20(&mut self) -> SETENA20_W<20> {
        SETENA20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena21(&mut self) -> SETENA21_W<21> {
        SETENA21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena22(&mut self) -> SETENA22_W<22> {
        SETENA22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena23(&mut self) -> SETENA23_W<23> {
        SETENA23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena24(&mut self) -> SETENA24_W<24> {
        SETENA24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena25(&mut self) -> SETENA25_W<25> {
        SETENA25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena26(&mut self) -> SETENA26_W<26> {
        SETENA26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena27(&mut self) -> SETENA27_W<27> {
        SETENA27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena28(&mut self) -> SETENA28_W<28> {
        SETENA28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena29(&mut self) -> SETENA29_W<29> {
        SETENA29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena30(&mut self) -> SETENA30_W<30> {
        SETENA30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena31(&mut self) -> SETENA31_W<31> {
        SETENA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_iser0](index.html) module"]
pub struct NVIC_ISER0_SPEC;
impl crate::RegisterSpec for NVIC_ISER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_iser0::R](R) reader structure"]
impl crate::Readable for NVIC_ISER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_iser0::W](W) writer structure"]
impl crate::Writable for NVIC_ISER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NVIC_ISER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
