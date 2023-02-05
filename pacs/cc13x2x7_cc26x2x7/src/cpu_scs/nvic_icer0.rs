#[doc = "Register `NVIC_ICER0` reader"]
pub struct R(crate::R<NVIC_ICER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_ICER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_ICER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_ICER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_ICER0` writer"]
pub struct W(crate::W<NVIC_ICER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_ICER0_SPEC>;
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
impl From<crate::W<NVIC_ICER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_ICER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRENA0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA0_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA1_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA2_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA3_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA4_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA5_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA6_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA7_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA8_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA8_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA9_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA9_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA10_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA10_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA11_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA11_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA12_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA12_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA13_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA13_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA14_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA14_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA15_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA15_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA16_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA16_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA17_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA17_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA18_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA18_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA19_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA19_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA20_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA20_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA21_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA21_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA22_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA22_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA23_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA23_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA24_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA24_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA25_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA25_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA26_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA26_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA27_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA27_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA28_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA28_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA29_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA29_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA30_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA30_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
#[doc = "Field `CLRENA31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA31_R = crate::BitReader<bool>;
#[doc = "Field `CLRENA31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type CLRENA31_W<'a, const O: u8> = crate::BitWriter<'a, u32, NVIC_ICER0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena0(&self) -> CLRENA0_R {
        CLRENA0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena1(&self) -> CLRENA1_R {
        CLRENA1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena2(&self) -> CLRENA2_R {
        CLRENA2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena3(&self) -> CLRENA3_R {
        CLRENA3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena4(&self) -> CLRENA4_R {
        CLRENA4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena5(&self) -> CLRENA5_R {
        CLRENA5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena6(&self) -> CLRENA6_R {
        CLRENA6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena7(&self) -> CLRENA7_R {
        CLRENA7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena8(&self) -> CLRENA8_R {
        CLRENA8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena9(&self) -> CLRENA9_R {
        CLRENA9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena10(&self) -> CLRENA10_R {
        CLRENA10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena11(&self) -> CLRENA11_R {
        CLRENA11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena12(&self) -> CLRENA12_R {
        CLRENA12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena13(&self) -> CLRENA13_R {
        CLRENA13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena14(&self) -> CLRENA14_R {
        CLRENA14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena15(&self) -> CLRENA15_R {
        CLRENA15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena16(&self) -> CLRENA16_R {
        CLRENA16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena17(&self) -> CLRENA17_R {
        CLRENA17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena18(&self) -> CLRENA18_R {
        CLRENA18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena19(&self) -> CLRENA19_R {
        CLRENA19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena20(&self) -> CLRENA20_R {
        CLRENA20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena21(&self) -> CLRENA21_R {
        CLRENA21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena22(&self) -> CLRENA22_R {
        CLRENA22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena23(&self) -> CLRENA23_R {
        CLRENA23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena24(&self) -> CLRENA24_R {
        CLRENA24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena25(&self) -> CLRENA25_R {
        CLRENA25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena26(&self) -> CLRENA26_R {
        CLRENA26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena27(&self) -> CLRENA27_R {
        CLRENA27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena28(&self) -> CLRENA28_R {
        CLRENA28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena29(&self) -> CLRENA29_R {
        CLRENA29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena30(&self) -> CLRENA30_R {
        CLRENA30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn clrena31(&self) -> CLRENA31_R {
        CLRENA31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena0(&mut self) -> CLRENA0_W<0> {
        CLRENA0_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena1(&mut self) -> CLRENA1_W<1> {
        CLRENA1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena2(&mut self) -> CLRENA2_W<2> {
        CLRENA2_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena3(&mut self) -> CLRENA3_W<3> {
        CLRENA3_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena4(&mut self) -> CLRENA4_W<4> {
        CLRENA4_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena5(&mut self) -> CLRENA5_W<5> {
        CLRENA5_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena6(&mut self) -> CLRENA6_W<6> {
        CLRENA6_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena7(&mut self) -> CLRENA7_W<7> {
        CLRENA7_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena8(&mut self) -> CLRENA8_W<8> {
        CLRENA8_W::new(self)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena9(&mut self) -> CLRENA9_W<9> {
        CLRENA9_W::new(self)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena10(&mut self) -> CLRENA10_W<10> {
        CLRENA10_W::new(self)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena11(&mut self) -> CLRENA11_W<11> {
        CLRENA11_W::new(self)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena12(&mut self) -> CLRENA12_W<12> {
        CLRENA12_W::new(self)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena13(&mut self) -> CLRENA13_W<13> {
        CLRENA13_W::new(self)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena14(&mut self) -> CLRENA14_W<14> {
        CLRENA14_W::new(self)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena15(&mut self) -> CLRENA15_W<15> {
        CLRENA15_W::new(self)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena16(&mut self) -> CLRENA16_W<16> {
        CLRENA16_W::new(self)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena17(&mut self) -> CLRENA17_W<17> {
        CLRENA17_W::new(self)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena18(&mut self) -> CLRENA18_W<18> {
        CLRENA18_W::new(self)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena19(&mut self) -> CLRENA19_W<19> {
        CLRENA19_W::new(self)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena20(&mut self) -> CLRENA20_W<20> {
        CLRENA20_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena21(&mut self) -> CLRENA21_W<21> {
        CLRENA21_W::new(self)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena22(&mut self) -> CLRENA22_W<22> {
        CLRENA22_W::new(self)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena23(&mut self) -> CLRENA23_W<23> {
        CLRENA23_W::new(self)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena24(&mut self) -> CLRENA24_W<24> {
        CLRENA24_W::new(self)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena25(&mut self) -> CLRENA25_W<25> {
        CLRENA25_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena26(&mut self) -> CLRENA26_W<26> {
        CLRENA26_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena27(&mut self) -> CLRENA27_W<27> {
        CLRENA27_W::new(self)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena28(&mut self) -> CLRENA28_W<28> {
        CLRENA28_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena29(&mut self) -> CLRENA29_W<29> {
        CLRENA29_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena30(&mut self) -> CLRENA30_W<30> {
        CLRENA30_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit disables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn clrena31(&mut self) -> CLRENA31_W<31> {
        CLRENA31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Irq 0 to 31 Clear Enable This register is used to disable interrupts and determine which interrupts are currently enabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvic_icer0](index.html) module"]
pub struct NVIC_ICER0_SPEC;
impl crate::RegisterSpec for NVIC_ICER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_icer0::R](R) reader structure"]
impl crate::Readable for NVIC_ICER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_icer0::W](W) writer structure"]
impl crate::Writable for NVIC_ICER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_ICER0 to value 0"]
impl crate::Resettable for NVIC_ICER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
