#[doc = "Register `DIN31_0` reader"]
pub type R = crate::R<Din31_0Spec>;
#[doc = "Register `DIN31_0` writer"]
pub type W = crate::W<Din31_0Spec>;
#[doc = "Field `DIO0` reader - 0:0\\]
Data input from DIO 0"]
pub type Dio0R = crate::BitReader;
#[doc = "Field `DIO1` reader - 1:1\\]
Data input from DIO 1"]
pub type Dio1R = crate::BitReader;
#[doc = "Field `DIO2` reader - 2:2\\]
Data input from DIO 2"]
pub type Dio2R = crate::BitReader;
#[doc = "Field `DIO3` reader - 3:3\\]
Data input from DIO 3"]
pub type Dio3R = crate::BitReader;
#[doc = "Field `DIO4` reader - 4:4\\]
Data input from DIO 4"]
pub type Dio4R = crate::BitReader;
#[doc = "Field `DIO5` reader - 5:5\\]
Data input from DIO 5"]
pub type Dio5R = crate::BitReader;
#[doc = "Field `DIO6` reader - 6:6\\]
Data input from DIO 6"]
pub type Dio6R = crate::BitReader;
#[doc = "Field `DIO7` reader - 7:7\\]
Data input from DIO 7"]
pub type Dio7R = crate::BitReader;
#[doc = "Field `DIO8` reader - 8:8\\]
Data input from DIO 8"]
pub type Dio8R = crate::BitReader;
#[doc = "Field `DIO9` reader - 9:9\\]
Data input from DIO 9"]
pub type Dio9R = crate::BitReader;
#[doc = "Field `DIO10` reader - 10:10\\]
Data input from DIO 10"]
pub type Dio10R = crate::BitReader;
#[doc = "Field `DIO11` reader - 11:11\\]
Data input from DIO 11"]
pub type Dio11R = crate::BitReader;
#[doc = "Field `DIO12` reader - 12:12\\]
Data input from DIO 12"]
pub type Dio12R = crate::BitReader;
#[doc = "Field `DIO13` reader - 13:13\\]
Data input from DIO 13"]
pub type Dio13R = crate::BitReader;
#[doc = "Field `DIO14` reader - 14:14\\]
Data input from DIO 14"]
pub type Dio14R = crate::BitReader;
#[doc = "Field `DIO15` reader - 15:15\\]
Data input from DIO 15"]
pub type Dio15R = crate::BitReader;
#[doc = "Field `DIO16` reader - 16:16\\]
Data input from DIO 16"]
pub type Dio16R = crate::BitReader;
#[doc = "Field `DIO17` reader - 17:17\\]
Data input from DIO 17"]
pub type Dio17R = crate::BitReader;
#[doc = "Field `DIO18` reader - 18:18\\]
Data input from DIO 18"]
pub type Dio18R = crate::BitReader;
#[doc = "Field `DIO19` reader - 19:19\\]
Data input from DIO 19"]
pub type Dio19R = crate::BitReader;
#[doc = "Field `DIO20` reader - 20:20\\]
Data input from DIO 20"]
pub type Dio20R = crate::BitReader;
#[doc = "Field `DIO21` reader - 21:21\\]
Data input from DIO 21"]
pub type Dio21R = crate::BitReader;
#[doc = "Field `DIO22` reader - 22:22\\]
Data input from DIO 22"]
pub type Dio22R = crate::BitReader;
#[doc = "Field `DIO23` reader - 23:23\\]
Data input from DIO 23"]
pub type Dio23R = crate::BitReader;
#[doc = "Field `DIO24` reader - 24:24\\]
Data input from DIO 24"]
pub type Dio24R = crate::BitReader;
#[doc = "Field `DIO25` reader - 25:25\\]
Data input from DIO 25"]
pub type Dio25R = crate::BitReader;
#[doc = "Field `DIO26` reader - 26:26\\]
Data input from DIO 26"]
pub type Dio26R = crate::BitReader;
#[doc = "Field `DIO27` reader - 27:27\\]
Data input from DIO 27"]
pub type Dio27R = crate::BitReader;
#[doc = "Field `DIO28` reader - 28:28\\]
Data input from DIO 28"]
pub type Dio28R = crate::BitReader;
#[doc = "Field `DIO29` reader - 29:29\\]
Data input from DIO 29"]
pub type Dio29R = crate::BitReader;
#[doc = "Field `DIO30` reader - 30:30\\]
Data input from DIO 30"]
pub type Dio30R = crate::BitReader;
#[doc = "Field `DIO31` reader - 31:31\\]
Data input from DIO 31"]
pub type Dio31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Data input from DIO 0"]
    #[inline(always)]
    pub fn dio0(&self) -> Dio0R {
        Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Data input from DIO 1"]
    #[inline(always)]
    pub fn dio1(&self) -> Dio1R {
        Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Data input from DIO 2"]
    #[inline(always)]
    pub fn dio2(&self) -> Dio2R {
        Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Data input from DIO 3"]
    #[inline(always)]
    pub fn dio3(&self) -> Dio3R {
        Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Data input from DIO 4"]
    #[inline(always)]
    pub fn dio4(&self) -> Dio4R {
        Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Data input from DIO 5"]
    #[inline(always)]
    pub fn dio5(&self) -> Dio5R {
        Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Data input from DIO 6"]
    #[inline(always)]
    pub fn dio6(&self) -> Dio6R {
        Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Data input from DIO 7"]
    #[inline(always)]
    pub fn dio7(&self) -> Dio7R {
        Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Data input from DIO 8"]
    #[inline(always)]
    pub fn dio8(&self) -> Dio8R {
        Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Data input from DIO 9"]
    #[inline(always)]
    pub fn dio9(&self) -> Dio9R {
        Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Data input from DIO 10"]
    #[inline(always)]
    pub fn dio10(&self) -> Dio10R {
        Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Data input from DIO 11"]
    #[inline(always)]
    pub fn dio11(&self) -> Dio11R {
        Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Data input from DIO 12"]
    #[inline(always)]
    pub fn dio12(&self) -> Dio12R {
        Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Data input from DIO 13"]
    #[inline(always)]
    pub fn dio13(&self) -> Dio13R {
        Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Data input from DIO 14"]
    #[inline(always)]
    pub fn dio14(&self) -> Dio14R {
        Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Data input from DIO 15"]
    #[inline(always)]
    pub fn dio15(&self) -> Dio15R {
        Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Data input from DIO 16"]
    #[inline(always)]
    pub fn dio16(&self) -> Dio16R {
        Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Data input from DIO 17"]
    #[inline(always)]
    pub fn dio17(&self) -> Dio17R {
        Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Data input from DIO 18"]
    #[inline(always)]
    pub fn dio18(&self) -> Dio18R {
        Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Data input from DIO 19"]
    #[inline(always)]
    pub fn dio19(&self) -> Dio19R {
        Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Data input from DIO 20"]
    #[inline(always)]
    pub fn dio20(&self) -> Dio20R {
        Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Data input from DIO 21"]
    #[inline(always)]
    pub fn dio21(&self) -> Dio21R {
        Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Data input from DIO 22"]
    #[inline(always)]
    pub fn dio22(&self) -> Dio22R {
        Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Data input from DIO 23"]
    #[inline(always)]
    pub fn dio23(&self) -> Dio23R {
        Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Data input from DIO 24"]
    #[inline(always)]
    pub fn dio24(&self) -> Dio24R {
        Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Data input from DIO 25"]
    #[inline(always)]
    pub fn dio25(&self) -> Dio25R {
        Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
Data input from DIO 26"]
    #[inline(always)]
    pub fn dio26(&self) -> Dio26R {
        Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Data input from DIO 27"]
    #[inline(always)]
    pub fn dio27(&self) -> Dio27R {
        Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Data input from DIO 28"]
    #[inline(always)]
    pub fn dio28(&self) -> Dio28R {
        Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Data input from DIO 29"]
    #[inline(always)]
    pub fn dio29(&self) -> Dio29R {
        Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Data input from DIO 30"]
    #[inline(always)]
    pub fn dio30(&self) -> Dio30R {
        Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Data input from DIO 31"]
    #[inline(always)]
    pub fn dio31(&self) -> Dio31R {
        Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Data Input from DIO 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din31_0Spec;
impl crate::RegisterSpec for Din31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din31_0::R`](R) reader structure"]
impl crate::Readable for Din31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`din31_0::W`](W) writer structure"]
impl crate::Writable for Din31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIN31_0 to value 0"]
impl crate::Resettable for Din31_0Spec {
    const RESET_VALUE: u32 = 0;
}
