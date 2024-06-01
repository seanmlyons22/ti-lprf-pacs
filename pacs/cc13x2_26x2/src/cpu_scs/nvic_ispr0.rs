#[doc = "Register `NVIC_ISPR0` reader"]
pub type R = crate::R<NvicIspr0Spec>;
#[doc = "Register `NVIC_ISPR0` writer"]
pub type W = crate::W<NvicIspr0Spec>;
#[doc = "Field `SETPEND0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type Setpend0R = crate::BitReader;
#[doc = "Field `SETPEND0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
pub type Setpend0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type Setpend1R = crate::BitReader;
#[doc = "Field `SETPEND1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
pub type Setpend1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type Setpend2R = crate::BitReader;
#[doc = "Field `SETPEND2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
pub type Setpend2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type Setpend3R = crate::BitReader;
#[doc = "Field `SETPEND3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
pub type Setpend3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type Setpend4R = crate::BitReader;
#[doc = "Field `SETPEND4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
pub type Setpend4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type Setpend5R = crate::BitReader;
#[doc = "Field `SETPEND5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
pub type Setpend5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type Setpend6R = crate::BitReader;
#[doc = "Field `SETPEND6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
pub type Setpend6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type Setpend7R = crate::BitReader;
#[doc = "Field `SETPEND7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
pub type Setpend7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type Setpend8R = crate::BitReader;
#[doc = "Field `SETPEND8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
pub type Setpend8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type Setpend9R = crate::BitReader;
#[doc = "Field `SETPEND9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
pub type Setpend9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type Setpend10R = crate::BitReader;
#[doc = "Field `SETPEND10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
pub type Setpend10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type Setpend11R = crate::BitReader;
#[doc = "Field `SETPEND11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
pub type Setpend11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type Setpend12R = crate::BitReader;
#[doc = "Field `SETPEND12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
pub type Setpend12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type Setpend13R = crate::BitReader;
#[doc = "Field `SETPEND13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
pub type Setpend13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type Setpend14R = crate::BitReader;
#[doc = "Field `SETPEND14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
pub type Setpend14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type Setpend15R = crate::BitReader;
#[doc = "Field `SETPEND15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
pub type Setpend15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type Setpend16R = crate::BitReader;
#[doc = "Field `SETPEND16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
pub type Setpend16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type Setpend17R = crate::BitReader;
#[doc = "Field `SETPEND17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
pub type Setpend17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type Setpend18R = crate::BitReader;
#[doc = "Field `SETPEND18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
pub type Setpend18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type Setpend19R = crate::BitReader;
#[doc = "Field `SETPEND19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
pub type Setpend19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type Setpend20R = crate::BitReader;
#[doc = "Field `SETPEND20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
pub type Setpend20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type Setpend21R = crate::BitReader;
#[doc = "Field `SETPEND21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
pub type Setpend21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type Setpend22R = crate::BitReader;
#[doc = "Field `SETPEND22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
pub type Setpend22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type Setpend23R = crate::BitReader;
#[doc = "Field `SETPEND23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
pub type Setpend23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type Setpend24R = crate::BitReader;
#[doc = "Field `SETPEND24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
pub type Setpend24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type Setpend25R = crate::BitReader;
#[doc = "Field `SETPEND25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
pub type Setpend25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type Setpend26R = crate::BitReader;
#[doc = "Field `SETPEND26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
pub type Setpend26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type Setpend27R = crate::BitReader;
#[doc = "Field `SETPEND27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
pub type Setpend27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type Setpend28R = crate::BitReader;
#[doc = "Field `SETPEND28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
pub type Setpend28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type Setpend29R = crate::BitReader;
#[doc = "Field `SETPEND29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
pub type Setpend29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type Setpend30R = crate::BitReader;
#[doc = "Field `SETPEND30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
pub type Setpend30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETPEND31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type Setpend31R = crate::BitReader;
#[doc = "Field `SETPEND31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
pub type Setpend31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend0(&self) -> Setpend0R {
        Setpend0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend1(&self) -> Setpend1R {
        Setpend1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend2(&self) -> Setpend2R {
        Setpend2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend3(&self) -> Setpend3R {
        Setpend3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend4(&self) -> Setpend4R {
        Setpend4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend5(&self) -> Setpend5R {
        Setpend5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend6(&self) -> Setpend6R {
        Setpend6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend7(&self) -> Setpend7R {
        Setpend7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend8(&self) -> Setpend8R {
        Setpend8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend9(&self) -> Setpend9R {
        Setpend9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend10(&self) -> Setpend10R {
        Setpend10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend11(&self) -> Setpend11R {
        Setpend11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend12(&self) -> Setpend12R {
        Setpend12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend13(&self) -> Setpend13R {
        Setpend13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend14(&self) -> Setpend14R {
        Setpend14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend15(&self) -> Setpend15R {
        Setpend15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend16(&self) -> Setpend16R {
        Setpend16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend17(&self) -> Setpend17R {
        Setpend17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend18(&self) -> Setpend18R {
        Setpend18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend19(&self) -> Setpend19R {
        Setpend19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend20(&self) -> Setpend20R {
        Setpend20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend21(&self) -> Setpend21R {
        Setpend21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend22(&self) -> Setpend22R {
        Setpend22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend23(&self) -> Setpend23R {
        Setpend23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend24(&self) -> Setpend24R {
        Setpend24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend25(&self) -> Setpend25R {
        Setpend25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend26(&self) -> Setpend26R {
        Setpend26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend27(&self) -> Setpend27R {
        Setpend27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend28(&self) -> Setpend28R {
        Setpend28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend29(&self) -> Setpend29R {
        Setpend29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend30(&self) -> Setpend30R {
        Setpend30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    pub fn setpend31(&self) -> Setpend31R {
        Setpend31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend0(&mut self) -> Setpend0W<NvicIspr0Spec> {
        Setpend0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend1(&mut self) -> Setpend1W<NvicIspr0Spec> {
        Setpend1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend2(&mut self) -> Setpend2W<NvicIspr0Spec> {
        Setpend2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend3(&mut self) -> Setpend3W<NvicIspr0Spec> {
        Setpend3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend4(&mut self) -> Setpend4W<NvicIspr0Spec> {
        Setpend4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend5(&mut self) -> Setpend5W<NvicIspr0Spec> {
        Setpend5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend6(&mut self) -> Setpend6W<NvicIspr0Spec> {
        Setpend6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend7(&mut self) -> Setpend7W<NvicIspr0Spec> {
        Setpend7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend8(&mut self) -> Setpend8W<NvicIspr0Spec> {
        Setpend8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend9(&mut self) -> Setpend9W<NvicIspr0Spec> {
        Setpend9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend10(&mut self) -> Setpend10W<NvicIspr0Spec> {
        Setpend10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend11(&mut self) -> Setpend11W<NvicIspr0Spec> {
        Setpend11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend12(&mut self) -> Setpend12W<NvicIspr0Spec> {
        Setpend12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend13(&mut self) -> Setpend13W<NvicIspr0Spec> {
        Setpend13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend14(&mut self) -> Setpend14W<NvicIspr0Spec> {
        Setpend14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend15(&mut self) -> Setpend15W<NvicIspr0Spec> {
        Setpend15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend16(&mut self) -> Setpend16W<NvicIspr0Spec> {
        Setpend16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend17(&mut self) -> Setpend17W<NvicIspr0Spec> {
        Setpend17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend18(&mut self) -> Setpend18W<NvicIspr0Spec> {
        Setpend18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend19(&mut self) -> Setpend19W<NvicIspr0Spec> {
        Setpend19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend20(&mut self) -> Setpend20W<NvicIspr0Spec> {
        Setpend20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend21(&mut self) -> Setpend21W<NvicIspr0Spec> {
        Setpend21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend22(&mut self) -> Setpend22W<NvicIspr0Spec> {
        Setpend22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend23(&mut self) -> Setpend23W<NvicIspr0Spec> {
        Setpend23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend24(&mut self) -> Setpend24W<NvicIspr0Spec> {
        Setpend24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend25(&mut self) -> Setpend25W<NvicIspr0Spec> {
        Setpend25W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend26(&mut self) -> Setpend26W<NvicIspr0Spec> {
        Setpend26W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend27(&mut self) -> Setpend27W<NvicIspr0Spec> {
        Setpend27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend28(&mut self) -> Setpend28W<NvicIspr0Spec> {
        Setpend28W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend29(&mut self) -> Setpend29W<NvicIspr0Spec> {
        Setpend29W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend30(&mut self) -> Setpend30W<NvicIspr0Spec> {
        Setpend30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit pends the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current state."]
    #[inline(always)]
    #[must_use]
    pub fn setpend31(&mut self) -> Setpend31W<NvicIspr0Spec> {
        Setpend31W::new(self, 31)
    }
}
#[doc = "Irq 0 to 31 Set Pending This register is used to force interrupts into the pending state and determine which interrupts are currently pending.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_ispr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_ispr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIspr0Spec;
impl crate::RegisterSpec for NvicIspr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_ispr0::R`](R) reader structure"]
impl crate::Readable for NvicIspr0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_ispr0::W`](W) writer structure"]
impl crate::Writable for NvicIspr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISPR0 to value 0"]
impl crate::Resettable for NvicIspr0Spec {
    const RESET_VALUE: u32 = 0;
}
