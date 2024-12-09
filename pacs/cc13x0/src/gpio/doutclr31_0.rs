#[doc = "Register `DOUTCLR31_0` reader"]
pub type R = crate::R<Doutclr31_0Spec>;
#[doc = "Register `DOUTCLR31_0` writer"]
pub type W = crate::W<Doutclr31_0Spec>;
#[doc = "Field `DIO0` writer - 0:0\\]
Clears bit 0"]
pub type Dio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO1` writer - 1:1\\]
Clears bit 1"]
pub type Dio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO2` writer - 2:2\\]
Clears bit 2"]
pub type Dio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO3` writer - 3:3\\]
Clears bit 3"]
pub type Dio3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO4` writer - 4:4\\]
Clears bit 4"]
pub type Dio4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO5` writer - 5:5\\]
Clears bit 5"]
pub type Dio5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO6` writer - 6:6\\]
Clears bit 6"]
pub type Dio6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO7` writer - 7:7\\]
Clears bit 7"]
pub type Dio7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO8` writer - 8:8\\]
Clears bit 8"]
pub type Dio8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO9` writer - 9:9\\]
Clears bit 9"]
pub type Dio9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO10` writer - 10:10\\]
Clears bit 10"]
pub type Dio10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO11` writer - 11:11\\]
Clears bit 11"]
pub type Dio11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO12` writer - 12:12\\]
Clears bit 12"]
pub type Dio12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO13` writer - 13:13\\]
Clears bit 13"]
pub type Dio13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO14` writer - 14:14\\]
Clears bit 14"]
pub type Dio14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO15` writer - 15:15\\]
Clears bit 15"]
pub type Dio15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO16` writer - 16:16\\]
Clears bit 16"]
pub type Dio16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO17` writer - 17:17\\]
Clears bit 17"]
pub type Dio17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO18` writer - 18:18\\]
Clears bit 18"]
pub type Dio18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO19` writer - 19:19\\]
Clears bit 19"]
pub type Dio19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO20` writer - 20:20\\]
Clears bit 20"]
pub type Dio20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO21` writer - 21:21\\]
Clears bit 21"]
pub type Dio21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO22` writer - 22:22\\]
Clears bit 22"]
pub type Dio22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO23` writer - 23:23\\]
Clears bit 23"]
pub type Dio23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO24` writer - 24:24\\]
Clears bit 24"]
pub type Dio24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO25` writer - 25:25\\]
Clears bit 25"]
pub type Dio25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO26` writer - 26:26\\]
Clears bit 26"]
pub type Dio26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO27` writer - 27:27\\]
Clears bit 27"]
pub type Dio27W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO28` writer - 28:28\\]
Clears bit 28"]
pub type Dio28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO29` writer - 29:29\\]
Clears bit 29"]
pub type Dio29W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO30` writer - 30:30\\]
Clears bit 30"]
pub type Dio30W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIO31` writer - 31:31\\]
Clears bit 31"]
pub type Dio31W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clears bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dio0(&mut self) -> Dio0W<Doutclr31_0Spec> {
        Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clears bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn dio1(&mut self) -> Dio1W<Doutclr31_0Spec> {
        Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clears bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn dio2(&mut self) -> Dio2W<Doutclr31_0Spec> {
        Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clears bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn dio3(&mut self) -> Dio3W<Doutclr31_0Spec> {
        Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clears bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn dio4(&mut self) -> Dio4W<Doutclr31_0Spec> {
        Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clears bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn dio5(&mut self) -> Dio5W<Doutclr31_0Spec> {
        Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clears bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn dio6(&mut self) -> Dio6W<Doutclr31_0Spec> {
        Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clears bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn dio7(&mut self) -> Dio7W<Doutclr31_0Spec> {
        Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Clears bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn dio8(&mut self) -> Dio8W<Doutclr31_0Spec> {
        Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Clears bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn dio9(&mut self) -> Dio9W<Doutclr31_0Spec> {
        Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Clears bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn dio10(&mut self) -> Dio10W<Doutclr31_0Spec> {
        Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Clears bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn dio11(&mut self) -> Dio11W<Doutclr31_0Spec> {
        Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Clears bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn dio12(&mut self) -> Dio12W<Doutclr31_0Spec> {
        Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Clears bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn dio13(&mut self) -> Dio13W<Doutclr31_0Spec> {
        Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Clears bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn dio14(&mut self) -> Dio14W<Doutclr31_0Spec> {
        Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Clears bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn dio15(&mut self) -> Dio15W<Doutclr31_0Spec> {
        Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Clears bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn dio16(&mut self) -> Dio16W<Doutclr31_0Spec> {
        Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Clears bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn dio17(&mut self) -> Dio17W<Doutclr31_0Spec> {
        Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Clears bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn dio18(&mut self) -> Dio18W<Doutclr31_0Spec> {
        Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Clears bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn dio19(&mut self) -> Dio19W<Doutclr31_0Spec> {
        Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Clears bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn dio20(&mut self) -> Dio20W<Doutclr31_0Spec> {
        Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Clears bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn dio21(&mut self) -> Dio21W<Doutclr31_0Spec> {
        Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Clears bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn dio22(&mut self) -> Dio22W<Doutclr31_0Spec> {
        Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Clears bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn dio23(&mut self) -> Dio23W<Doutclr31_0Spec> {
        Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Clears bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn dio24(&mut self) -> Dio24W<Doutclr31_0Spec> {
        Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Clears bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn dio25(&mut self) -> Dio25W<Doutclr31_0Spec> {
        Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
Clears bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn dio26(&mut self) -> Dio26W<Doutclr31_0Spec> {
        Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
Clears bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn dio27(&mut self) -> Dio27W<Doutclr31_0Spec> {
        Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
Clears bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn dio28(&mut self) -> Dio28W<Doutclr31_0Spec> {
        Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Clears bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn dio29(&mut self) -> Dio29W<Doutclr31_0Spec> {
        Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Clears bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn dio30(&mut self) -> Dio30W<Doutclr31_0Spec> {
        Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Clears bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn dio31(&mut self) -> Dio31W<Doutclr31_0Spec> {
        Dio31W::new(self, 31)
    }
}
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutclr31_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutclr31_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doutclr31_0Spec;
impl crate::RegisterSpec for Doutclr31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doutclr31_0::R`](R) reader structure"]
impl crate::Readable for Doutclr31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`doutclr31_0::W`](W) writer structure"]
impl crate::Writable for Doutclr31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTCLR31_0 to value 0"]
impl crate::Resettable for Doutclr31_0Spec {
    const RESET_VALUE: u32 = 0;
}
