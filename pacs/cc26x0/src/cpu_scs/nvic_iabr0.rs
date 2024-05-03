#[doc = "Register `NVIC_IABR0` reader"]
pub type R = crate::R<NvicIabr0Spec>;
#[doc = "Register `NVIC_IABR0` writer"]
pub type W = crate::W<NvicIabr0Spec>;
#[doc = "Field `ACTIVE0` reader - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub type Active0R = crate::BitReader;
#[doc = "Field `ACTIVE0` writer - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
pub type Active0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE1` reader - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub type Active1R = crate::BitReader;
#[doc = "Field `ACTIVE1` writer - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
pub type Active1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE2` reader - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub type Active2R = crate::BitReader;
#[doc = "Field `ACTIVE2` writer - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
pub type Active2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE3` reader - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub type Active3R = crate::BitReader;
#[doc = "Field `ACTIVE3` writer - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
pub type Active3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE4` reader - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub type Active4R = crate::BitReader;
#[doc = "Field `ACTIVE4` writer - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
pub type Active4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE5` reader - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub type Active5R = crate::BitReader;
#[doc = "Field `ACTIVE5` writer - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
pub type Active5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE6` reader - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub type Active6R = crate::BitReader;
#[doc = "Field `ACTIVE6` writer - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
pub type Active6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE7` reader - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub type Active7R = crate::BitReader;
#[doc = "Field `ACTIVE7` writer - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
pub type Active7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE8` reader - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub type Active8R = crate::BitReader;
#[doc = "Field `ACTIVE8` writer - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
pub type Active8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE9` reader - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub type Active9R = crate::BitReader;
#[doc = "Field `ACTIVE9` writer - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
pub type Active9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE10` reader - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub type Active10R = crate::BitReader;
#[doc = "Field `ACTIVE10` writer - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
pub type Active10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE11` reader - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub type Active11R = crate::BitReader;
#[doc = "Field `ACTIVE11` writer - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
pub type Active11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE12` reader - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub type Active12R = crate::BitReader;
#[doc = "Field `ACTIVE12` writer - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
pub type Active12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE13` reader - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub type Active13R = crate::BitReader;
#[doc = "Field `ACTIVE13` writer - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
pub type Active13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE14` reader - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub type Active14R = crate::BitReader;
#[doc = "Field `ACTIVE14` writer - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
pub type Active14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE15` reader - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub type Active15R = crate::BitReader;
#[doc = "Field `ACTIVE15` writer - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
pub type Active15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE16` reader - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub type Active16R = crate::BitReader;
#[doc = "Field `ACTIVE16` writer - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
pub type Active16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE17` reader - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub type Active17R = crate::BitReader;
#[doc = "Field `ACTIVE17` writer - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
pub type Active17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE18` reader - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub type Active18R = crate::BitReader;
#[doc = "Field `ACTIVE18` writer - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
pub type Active18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE19` reader - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub type Active19R = crate::BitReader;
#[doc = "Field `ACTIVE19` writer - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
pub type Active19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE20` reader - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub type Active20R = crate::BitReader;
#[doc = "Field `ACTIVE20` writer - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
pub type Active20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE21` reader - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub type Active21R = crate::BitReader;
#[doc = "Field `ACTIVE21` writer - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
pub type Active21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE22` reader - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub type Active22R = crate::BitReader;
#[doc = "Field `ACTIVE22` writer - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
pub type Active22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE23` reader - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub type Active23R = crate::BitReader;
#[doc = "Field `ACTIVE23` writer - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
pub type Active23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE24` reader - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub type Active24R = crate::BitReader;
#[doc = "Field `ACTIVE24` writer - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
pub type Active24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE25` reader - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub type Active25R = crate::BitReader;
#[doc = "Field `ACTIVE25` writer - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
pub type Active25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE26` reader - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub type Active26R = crate::BitReader;
#[doc = "Field `ACTIVE26` writer - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
pub type Active26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE27` reader - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub type Active27R = crate::BitReader;
#[doc = "Field `ACTIVE27` writer - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
pub type Active27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE28` reader - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub type Active28R = crate::BitReader;
#[doc = "Field `ACTIVE28` writer - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
pub type Active28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE29` reader - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub type Active29R = crate::BitReader;
#[doc = "Field `ACTIVE29` writer - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
pub type Active29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE30` reader - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub type Active30R = crate::BitReader;
#[doc = "Field `ACTIVE30` writer - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
pub type Active30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE31` reader - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub type Active31R = crate::BitReader;
#[doc = "Field `ACTIVE31` writer - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
pub type Active31W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    pub fn active0(&self) -> Active0R {
        Active0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    pub fn active1(&self) -> Active1R {
        Active1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    pub fn active2(&self) -> Active2R {
        Active2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    pub fn active3(&self) -> Active3R {
        Active3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    pub fn active4(&self) -> Active4R {
        Active4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    pub fn active5(&self) -> Active5R {
        Active5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    pub fn active6(&self) -> Active6R {
        Active6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    pub fn active7(&self) -> Active7R {
        Active7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    pub fn active8(&self) -> Active8R {
        Active8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    pub fn active9(&self) -> Active9R {
        Active9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    pub fn active10(&self) -> Active10R {
        Active10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    pub fn active11(&self) -> Active11R {
        Active11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    pub fn active12(&self) -> Active12R {
        Active12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    pub fn active13(&self) -> Active13R {
        Active13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    pub fn active14(&self) -> Active14R {
        Active14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    pub fn active15(&self) -> Active15R {
        Active15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    pub fn active16(&self) -> Active16R {
        Active16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    pub fn active17(&self) -> Active17R {
        Active17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    pub fn active18(&self) -> Active18R {
        Active18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    pub fn active19(&self) -> Active19R {
        Active19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    pub fn active20(&self) -> Active20R {
        Active20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    pub fn active21(&self) -> Active21R {
        Active21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    pub fn active22(&self) -> Active22R {
        Active22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    pub fn active23(&self) -> Active23R {
        Active23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    pub fn active24(&self) -> Active24R {
        Active24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    pub fn active25(&self) -> Active25R {
        Active25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    pub fn active26(&self) -> Active26R {
        Active26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    pub fn active27(&self) -> Active27R {
        Active27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    pub fn active28(&self) -> Active28R {
        Active28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    pub fn active29(&self) -> Active29R {
        Active29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    pub fn active30(&self) -> Active30R {
        Active30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    pub fn active31(&self) -> Active31R {
        Active31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reading 0 from this bit implies that interrupt line 0 is not active. Reading 1 from this bit implies that the interrupt line 0 is active (See EVENT:CPUIRQSEL0.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active0(&mut self) -> Active0W<NvicIabr0Spec> {
        Active0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Reading 0 from this bit implies that interrupt line 1 is not active. Reading 1 from this bit implies that the interrupt line 1 is active (See EVENT:CPUIRQSEL1.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active1(&mut self) -> Active1W<NvicIabr0Spec> {
        Active1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Reading 0 from this bit implies that interrupt line 2 is not active. Reading 1 from this bit implies that the interrupt line 2 is active (See EVENT:CPUIRQSEL2.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active2(&mut self) -> Active2W<NvicIabr0Spec> {
        Active2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Reading 0 from this bit implies that interrupt line 3 is not active. Reading 1 from this bit implies that the interrupt line 3 is active (See EVENT:CPUIRQSEL3.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active3(&mut self) -> Active3W<NvicIabr0Spec> {
        Active3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Reading 0 from this bit implies that interrupt line 4 is not active. Reading 1 from this bit implies that the interrupt line 4 is active (See EVENT:CPUIRQSEL4.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active4(&mut self) -> Active4W<NvicIabr0Spec> {
        Active4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Reading 0 from this bit implies that interrupt line 5 is not active. Reading 1 from this bit implies that the interrupt line 5 is active (See EVENT:CPUIRQSEL5.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active5(&mut self) -> Active5W<NvicIabr0Spec> {
        Active5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Reading 0 from this bit implies that interrupt line 6 is not active. Reading 1 from this bit implies that the interrupt line 6 is active (See EVENT:CPUIRQSEL6.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active6(&mut self) -> Active6W<NvicIabr0Spec> {
        Active6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Reading 0 from this bit implies that interrupt line 7 is not active. Reading 1 from this bit implies that the interrupt line 7 is active (See EVENT:CPUIRQSEL7.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active7(&mut self) -> Active7W<NvicIabr0Spec> {
        Active7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Reading 0 from this bit implies that interrupt line 8 is not active. Reading 1 from this bit implies that the interrupt line 8 is active (See EVENT:CPUIRQSEL8.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active8(&mut self) -> Active8W<NvicIabr0Spec> {
        Active8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Reading 0 from this bit implies that interrupt line 9 is not active. Reading 1 from this bit implies that the interrupt line 9 is active (See EVENT:CPUIRQSEL9.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active9(&mut self) -> Active9W<NvicIabr0Spec> {
        Active9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Reading 0 from this bit implies that interrupt line 10 is not active. Reading 1 from this bit implies that the interrupt line 10 is active (See EVENT:CPUIRQSEL10.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active10(&mut self) -> Active10W<NvicIabr0Spec> {
        Active10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Reading 0 from this bit implies that interrupt line 11 is not active. Reading 1 from this bit implies that the interrupt line 11 is active (See EVENT:CPUIRQSEL11.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active11(&mut self) -> Active11W<NvicIabr0Spec> {
        Active11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Reading 0 from this bit implies that interrupt line 12 is not active. Reading 1 from this bit implies that the interrupt line 12 is active (See EVENT:CPUIRQSEL12.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active12(&mut self) -> Active12W<NvicIabr0Spec> {
        Active12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Reading 0 from this bit implies that interrupt line 13 is not active. Reading 1 from this bit implies that the interrupt line 13 is active (See EVENT:CPUIRQSEL13.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active13(&mut self) -> Active13W<NvicIabr0Spec> {
        Active13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Reading 0 from this bit implies that interrupt line 14 is not active. Reading 1 from this bit implies that the interrupt line 14 is active (See EVENT:CPUIRQSEL14.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active14(&mut self) -> Active14W<NvicIabr0Spec> {
        Active14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Reading 0 from this bit implies that interrupt line 15 is not active. Reading 1 from this bit implies that the interrupt line 15 is active (See EVENT:CPUIRQSEL15.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active15(&mut self) -> Active15W<NvicIabr0Spec> {
        Active15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Reading 0 from this bit implies that interrupt line 16 is not active. Reading 1 from this bit implies that the interrupt line 16 is active (See EVENT:CPUIRQSEL16.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active16(&mut self) -> Active16W<NvicIabr0Spec> {
        Active16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Reading 0 from this bit implies that interrupt line 17 is not active. Reading 1 from this bit implies that the interrupt line 17 is active (See EVENT:CPUIRQSEL17.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active17(&mut self) -> Active17W<NvicIabr0Spec> {
        Active17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Reading 0 from this bit implies that interrupt line 18 is not active. Reading 1 from this bit implies that the interrupt line 18 is active (See EVENT:CPUIRQSEL18.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active18(&mut self) -> Active18W<NvicIabr0Spec> {
        Active18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Reading 0 from this bit implies that interrupt line 19 is not active. Reading 1 from this bit implies that the interrupt line 19 is active (See EVENT:CPUIRQSEL19.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active19(&mut self) -> Active19W<NvicIabr0Spec> {
        Active19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Reading 0 from this bit implies that interrupt line 20 is not active. Reading 1 from this bit implies that the interrupt line 20 is active (See EVENT:CPUIRQSEL20.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active20(&mut self) -> Active20W<NvicIabr0Spec> {
        Active20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Reading 0 from this bit implies that interrupt line 21 is not active. Reading 1 from this bit implies that the interrupt line 21 is active (See EVENT:CPUIRQSEL21.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active21(&mut self) -> Active21W<NvicIabr0Spec> {
        Active21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Reading 0 from this bit implies that interrupt line 22 is not active. Reading 1 from this bit implies that the interrupt line 22 is active (See EVENT:CPUIRQSEL22.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active22(&mut self) -> Active22W<NvicIabr0Spec> {
        Active22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Reading 0 from this bit implies that interrupt line 23 is not active. Reading 1 from this bit implies that the interrupt line 23 is active (See EVENT:CPUIRQSEL23.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active23(&mut self) -> Active23W<NvicIabr0Spec> {
        Active23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Reading 0 from this bit implies that interrupt line 24 is not active. Reading 1 from this bit implies that the interrupt line 24 is active (See EVENT:CPUIRQSEL24.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active24(&mut self) -> Active24W<NvicIabr0Spec> {
        Active24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Reading 0 from this bit implies that interrupt line 25 is not active. Reading 1 from this bit implies that the interrupt line 25 is active (See EVENT:CPUIRQSEL25.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active25(&mut self) -> Active25W<NvicIabr0Spec> {
        Active25W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Reading 0 from this bit implies that interrupt line 26 is not active. Reading 1 from this bit implies that the interrupt line 26 is active (See EVENT:CPUIRQSEL26.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active26(&mut self) -> Active26W<NvicIabr0Spec> {
        Active26W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Reading 0 from this bit implies that interrupt line 27 is not active. Reading 1 from this bit implies that the interrupt line 27 is active (See EVENT:CPUIRQSEL27.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active27(&mut self) -> Active27W<NvicIabr0Spec> {
        Active27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Reading 0 from this bit implies that interrupt line 28 is not active. Reading 1 from this bit implies that the interrupt line 28 is active (See EVENT:CPUIRQSEL28.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active28(&mut self) -> Active28W<NvicIabr0Spec> {
        Active28W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Reading 0 from this bit implies that interrupt line 29 is not active. Reading 1 from this bit implies that the interrupt line 29 is active (See EVENT:CPUIRQSEL29.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active29(&mut self) -> Active29W<NvicIabr0Spec> {
        Active29W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Reading 0 from this bit implies that interrupt line 30 is not active. Reading 1 from this bit implies that the interrupt line 30 is active (See EVENT:CPUIRQSEL30.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active30(&mut self) -> Active30W<NvicIabr0Spec> {
        Active30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Reading 0 from this bit implies that interrupt line 31 is not active. Reading 1 from this bit implies that the interrupt line 31 is active (See EVENT:CPUIRQSEL31.EV for details)."]
    #[inline(always)]
    #[must_use]
    pub fn active31(&mut self) -> Active31W<NvicIabr0Spec> {
        Active31W::new(self, 31)
    }
}
#[doc = "Irq 0 to 31 Active Bit This register is used to determine which interrupts are active. Each flag in the register corresponds to one interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nvic_iabr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nvic_iabr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NvicIabr0Spec;
impl crate::RegisterSpec for NvicIabr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nvic_iabr0::R`](R) reader structure"]
impl crate::Readable for NvicIabr0Spec {}
#[doc = "`write(|w| ..)` method takes [`nvic_iabr0::W`](W) writer structure"]
impl crate::Writable for NvicIabr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NVIC_IABR0 to value 0"]
impl crate::Resettable for NvicIabr0Spec {
    const RESET_VALUE: u32 = 0;
}
