#[doc = "Register `NVIC_ISER0` reader"]
pub type R = crate::R<NvicIser0Spec>;
#[doc = "Register `NVIC_ISER0` writer"]
pub type W = crate::W<NvicIser0Spec>;
#[doc = "Field `SETENA0` reader - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type Setena0R = crate::BitReader;
#[doc = "Field `SETENA0` writer - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
pub type Setena0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA1` reader - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type Setena1R = crate::BitReader;
#[doc = "Field `SETENA1` writer - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
pub type Setena1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA2` reader - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type Setena2R = crate::BitReader;
#[doc = "Field `SETENA2` writer - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
pub type Setena2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA3` reader - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type Setena3R = crate::BitReader;
#[doc = "Field `SETENA3` writer - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
pub type Setena3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA4` reader - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type Setena4R = crate::BitReader;
#[doc = "Field `SETENA4` writer - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
pub type Setena4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA5` reader - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type Setena5R = crate::BitReader;
#[doc = "Field `SETENA5` writer - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
pub type Setena5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA6` reader - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type Setena6R = crate::BitReader;
#[doc = "Field `SETENA6` writer - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
pub type Setena6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA7` reader - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type Setena7R = crate::BitReader;
#[doc = "Field `SETENA7` writer - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
pub type Setena7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA8` reader - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type Setena8R = crate::BitReader;
#[doc = "Field `SETENA8` writer - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
pub type Setena8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA9` reader - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type Setena9R = crate::BitReader;
#[doc = "Field `SETENA9` writer - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
pub type Setena9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA10` reader - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type Setena10R = crate::BitReader;
#[doc = "Field `SETENA10` writer - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
pub type Setena10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA11` reader - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type Setena11R = crate::BitReader;
#[doc = "Field `SETENA11` writer - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
pub type Setena11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA12` reader - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type Setena12R = crate::BitReader;
#[doc = "Field `SETENA12` writer - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
pub type Setena12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA13` reader - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type Setena13R = crate::BitReader;
#[doc = "Field `SETENA13` writer - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
pub type Setena13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA14` reader - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type Setena14R = crate::BitReader;
#[doc = "Field `SETENA14` writer - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
pub type Setena14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA15` reader - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type Setena15R = crate::BitReader;
#[doc = "Field `SETENA15` writer - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
pub type Setena15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA16` reader - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type Setena16R = crate::BitReader;
#[doc = "Field `SETENA16` writer - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
pub type Setena16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA17` reader - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type Setena17R = crate::BitReader;
#[doc = "Field `SETENA17` writer - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
pub type Setena17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA18` reader - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type Setena18R = crate::BitReader;
#[doc = "Field `SETENA18` writer - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
pub type Setena18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA19` reader - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type Setena19R = crate::BitReader;
#[doc = "Field `SETENA19` writer - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
pub type Setena19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA20` reader - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type Setena20R = crate::BitReader;
#[doc = "Field `SETENA20` writer - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
pub type Setena20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA21` reader - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type Setena21R = crate::BitReader;
#[doc = "Field `SETENA21` writer - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
pub type Setena21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA22` reader - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type Setena22R = crate::BitReader;
#[doc = "Field `SETENA22` writer - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
pub type Setena22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA23` reader - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type Setena23R = crate::BitReader;
#[doc = "Field `SETENA23` writer - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
pub type Setena23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA24` reader - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type Setena24R = crate::BitReader;
#[doc = "Field `SETENA24` writer - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
pub type Setena24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA25` reader - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type Setena25R = crate::BitReader;
#[doc = "Field `SETENA25` writer - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
pub type Setena25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA26` reader - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type Setena26R = crate::BitReader;
#[doc = "Field `SETENA26` writer - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
pub type Setena26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA27` reader - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type Setena27R = crate::BitReader;
#[doc = "Field `SETENA27` writer - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
pub type Setena27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA28` reader - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type Setena28R = crate::BitReader;
#[doc = "Field `SETENA28` writer - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
pub type Setena28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA29` reader - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type Setena29R = crate::BitReader;
#[doc = "Field `SETENA29` writer - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
pub type Setena29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA30` reader - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type Setena30R = crate::BitReader;
#[doc = "Field `SETENA30` writer - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
pub type Setena30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETENA31` reader - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type Setena31R = crate::BitReader;
#[doc = "Field `SETENA31` writer - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
pub type Setena31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena0(&self) -> Setena0R {
        Setena0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena1(&self) -> Setena1R {
        Setena1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena2(&self) -> Setena2R {
        Setena2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena3(&self) -> Setena3R {
        Setena3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena4(&self) -> Setena4R {
        Setena4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena5(&self) -> Setena5R {
        Setena5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena6(&self) -> Setena6R {
        Setena6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena7(&self) -> Setena7R {
        Setena7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena8(&self) -> Setena8R {
        Setena8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena9(&self) -> Setena9R {
        Setena9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena10(&self) -> Setena10R {
        Setena10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena11(&self) -> Setena11R {
        Setena11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena12(&self) -> Setena12R {
        Setena12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena13(&self) -> Setena13R {
        Setena13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena14(&self) -> Setena14R {
        Setena14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena15(&self) -> Setena15R {
        Setena15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena16(&self) -> Setena16R {
        Setena16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena17(&self) -> Setena17R {
        Setena17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena18(&self) -> Setena18R {
        Setena18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena19(&self) -> Setena19R {
        Setena19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena20(&self) -> Setena20R {
        Setena20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena21(&self) -> Setena21R {
        Setena21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena22(&self) -> Setena22R {
        Setena22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena23(&self) -> Setena23R {
        Setena23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena24(&self) -> Setena24R {
        Setena24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena25(&self) -> Setena25R {
        Setena25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena26(&self) -> Setena26R {
        Setena26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena27(&self) -> Setena27R {
        Setena27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena28(&self) -> Setena28R {
        Setena28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena29(&self) -> Setena29R {
        Setena29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena30(&self) -> Setena30R {
        Setena30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    pub fn setena31(&self) -> Setena31R {
        Setena31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 0 (See EVENT:CPUIRQSEL0.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena0(&mut self) -> Setena0W<NvicIser0Spec> {
        Setena0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 1 (See EVENT:CPUIRQSEL1.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena1(&mut self) -> Setena1W<NvicIser0Spec> {
        Setena1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 2 (See EVENT:CPUIRQSEL2.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena2(&mut self) -> Setena2W<NvicIser0Spec> {
        Setena2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 3 (See EVENT:CPUIRQSEL3.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena3(&mut self) -> Setena3W<NvicIser0Spec> {
        Setena3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 4 (See EVENT:CPUIRQSEL4.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena4(&mut self) -> Setena4W<NvicIser0Spec> {
        Setena4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 5 (See EVENT:CPUIRQSEL5.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena5(&mut self) -> Setena5W<NvicIser0Spec> {
        Setena5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 6 (See EVENT:CPUIRQSEL6.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena6(&mut self) -> Setena6W<NvicIser0Spec> {
        Setena6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 7 (See EVENT:CPUIRQSEL7.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena7(&mut self) -> Setena7W<NvicIser0Spec> {
        Setena7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 8 (See EVENT:CPUIRQSEL8.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena8(&mut self) -> Setena8W<NvicIser0Spec> {
        Setena8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 9 (See EVENT:CPUIRQSEL9.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena9(&mut self) -> Setena9W<NvicIser0Spec> {
        Setena9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 10 (See EVENT:CPUIRQSEL10.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena10(&mut self) -> Setena10W<NvicIser0Spec> {
        Setena10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 11 (See EVENT:CPUIRQSEL11.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena11(&mut self) -> Setena11W<NvicIser0Spec> {
        Setena11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 12 (See EVENT:CPUIRQSEL12.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena12(&mut self) -> Setena12W<NvicIser0Spec> {
        Setena12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 13 (See EVENT:CPUIRQSEL13.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena13(&mut self) -> Setena13W<NvicIser0Spec> {
        Setena13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 14 (See EVENT:CPUIRQSEL14.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena14(&mut self) -> Setena14W<NvicIser0Spec> {
        Setena14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 15 (See EVENT:CPUIRQSEL15.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena15(&mut self) -> Setena15W<NvicIser0Spec> {
        Setena15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 16 (See EVENT:CPUIRQSEL16.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena16(&mut self) -> Setena16W<NvicIser0Spec> {
        Setena16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 17 (See EVENT:CPUIRQSEL17.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena17(&mut self) -> Setena17W<NvicIser0Spec> {
        Setena17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 18 (See EVENT:CPUIRQSEL18.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena18(&mut self) -> Setena18W<NvicIser0Spec> {
        Setena18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 19 (See EVENT:CPUIRQSEL19.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena19(&mut self) -> Setena19W<NvicIser0Spec> {
        Setena19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 20 (See EVENT:CPUIRQSEL20.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena20(&mut self) -> Setena20W<NvicIser0Spec> {
        Setena20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 21 (See EVENT:CPUIRQSEL21.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena21(&mut self) -> Setena21W<NvicIser0Spec> {
        Setena21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 22 (See EVENT:CPUIRQSEL22.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena22(&mut self) -> Setena22W<NvicIser0Spec> {
        Setena22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 23 (See EVENT:CPUIRQSEL23.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena23(&mut self) -> Setena23W<NvicIser0Spec> {
        Setena23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 24 (See EVENT:CPUIRQSEL24.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena24(&mut self) -> Setena24W<NvicIser0Spec> {
        Setena24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 25 (See EVENT:CPUIRQSEL25.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena25(&mut self) -> Setena25W<NvicIser0Spec> {
        Setena25W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 26 (See EVENT:CPUIRQSEL26.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena26(&mut self) -> Setena26W<NvicIser0Spec> {
        Setena26W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 27 (See EVENT:CPUIRQSEL27.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena27(&mut self) -> Setena27W<NvicIser0Spec> {
        Setena27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 28 (See EVENT:CPUIRQSEL28.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena28(&mut self) -> Setena28W<NvicIser0Spec> {
        Setena28W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 29 (See EVENT:CPUIRQSEL29.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena29(&mut self) -> Setena29W<NvicIser0Spec> {
        Setena29W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 30 (See EVENT:CPUIRQSEL30.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena30(&mut self) -> Setena30W<NvicIser0Spec> {
        Setena30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Writing 0 to this bit has no effect, writing 1 to this bit enables the interrupt number 31 (See EVENT:CPUIRQSEL31.EV for details). Reading the bit returns its current enable state."]
    #[inline(always)]
    #[must_use]
    pub fn setena31(&mut self) -> Setena31W<NvicIser0Spec> {
        Setena31W::new(self, 31)
    }
}
#[doc = "Irq 0 to 31 Set Enable This register is used to enable interrupts and determine which interrupts are currently enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iser0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iser0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIser0Spec;
impl crate::RegisterSpec for NvicIser0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iser0::R`](R) reader structure"]
impl crate::Readable for NvicIser0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iser0::W`](W) writer structure"]
impl crate::Writable for NvicIser0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_ISER0 to value 0"]
impl crate::Resettable for NvicIser0Spec {
    const RESET_VALUE: u32 = 0;
}
