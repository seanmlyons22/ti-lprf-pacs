#[doc = "Register `RFHWIEN` reader"]
pub type R = crate::R<RfhwienSpec>;
#[doc = "Register `RFHWIEN` writer"]
pub type W = crate::W<RfhwienSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSCA` reader - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
pub type FscaR = crate::BitReader;
#[doc = "Field `FSCA` writer - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
pub type FscaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMDONE` reader - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
pub type MdmdoneR = crate::BitReader;
#[doc = "Field `MDMDONE` writer - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
pub type MdmdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMIN` reader - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
pub type MdminR = crate::BitReader;
#[doc = "Field `MDMIN` writer - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
pub type MdminW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMOUT` reader - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
pub type MdmoutR = crate::BitReader;
#[doc = "Field `MDMOUT` writer - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
pub type MdmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMSOFT` reader - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
pub type MdmsoftR = crate::BitReader;
#[doc = "Field `MDMSOFT` writer - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
pub type MdmsoftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCTK` reader - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
pub type TrctkR = crate::BitReader;
#[doc = "Field `TRCTK` writer - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
pub type TrctkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFEDONE` reader - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
pub type RfedoneR = crate::BitReader;
#[doc = "Field `RFEDONE` writer - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
pub type RfedoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT0` reader - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
pub type Rfesoft0R = crate::BitReader;
#[doc = "Field `RFESOFT0` writer - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
pub type Rfesoft0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT1` reader - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
pub type Rfesoft1R = crate::BitReader;
#[doc = "Field `RFESOFT1` writer - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
pub type Rfesoft1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT2` reader - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
pub type Rfesoft2R = crate::BitReader;
#[doc = "Field `RFESOFT2` writer - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
pub type Rfesoft2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH0` reader - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
pub type Ratch0R = crate::BitReader;
#[doc = "Field `RATCH0` writer - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
pub type Ratch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH1` reader - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
pub type Ratch1R = crate::BitReader;
#[doc = "Field `RATCH1` writer - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
pub type Ratch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH2` reader - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
pub type Ratch2R = crate::BitReader;
#[doc = "Field `RATCH2` writer - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
pub type Ratch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH3` reader - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
pub type Ratch3R = crate::BitReader;
#[doc = "Field `RATCH3` writer - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
pub type Ratch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH4` reader - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
pub type Ratch4R = crate::BitReader;
#[doc = "Field `RATCH4` writer - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
pub type Ratch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH5` reader - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
pub type Ratch5R = crate::BitReader;
#[doc = "Field `RATCH5` writer - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
pub type Ratch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH6` reader - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
pub type Ratch6R = crate::BitReader;
#[doc = "Field `RATCH6` writer - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
pub type Ratch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH7` reader - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
pub type Ratch7R = crate::BitReader;
#[doc = "Field `RATCH7` writer - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
pub type Ratch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
    #[inline(always)]
    pub fn fsca(&self) -> FscaR {
        FscaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
    #[inline(always)]
    pub fn mdmdone(&self) -> MdmdoneR {
        MdmdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
    #[inline(always)]
    pub fn mdmin(&self) -> MdminR {
        MdminR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
    #[inline(always)]
    pub fn mdmout(&self) -> MdmoutR {
        MdmoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
    #[inline(always)]
    pub fn mdmsoft(&self) -> MdmsoftR {
        MdmsoftR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
    #[inline(always)]
    pub fn trctk(&self) -> TrctkR {
        TrctkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
    #[inline(always)]
    pub fn rfedone(&self) -> RfedoneR {
        RfedoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
    #[inline(always)]
    pub fn rfesoft0(&self) -> Rfesoft0R {
        Rfesoft0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
    #[inline(always)]
    pub fn rfesoft1(&self) -> Rfesoft1R {
        Rfesoft1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
    #[inline(always)]
    pub fn rfesoft2(&self) -> Rfesoft2R {
        Rfesoft2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
    #[inline(always)]
    pub fn ratch0(&self) -> Ratch0R {
        Ratch0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
    #[inline(always)]
    pub fn ratch1(&self) -> Ratch1R {
        Ratch1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
    #[inline(always)]
    pub fn ratch2(&self) -> Ratch2R {
        Ratch2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
    #[inline(always)]
    pub fn ratch3(&self) -> Ratch3R {
        Ratch3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
    #[inline(always)]
    pub fn ratch4(&self) -> Ratch4R {
        Ratch4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
    #[inline(always)]
    pub fn ratch5(&self) -> Ratch5R {
        Ratch5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
    #[inline(always)]
    pub fn ratch6(&self) -> Ratch6R {
        Ratch6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
    #[inline(always)]
    pub fn ratch7(&self) -> Ratch7R {
        Ratch7R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> Reserved0W<RfhwienSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt enable for RFHWIFG.FSCA."]
    #[inline(always)]
    #[must_use]
    pub fn fsca(&mut self) -> FscaW<RfhwienSpec> {
        FscaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Interrupt enable for RFHWIFG.MDMDONE."]
    #[inline(always)]
    #[must_use]
    pub fn mdmdone(&mut self) -> MdmdoneW<RfhwienSpec> {
        MdmdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Interrupt enable for RFHWIFG.MDMIN."]
    #[inline(always)]
    #[must_use]
    pub fn mdmin(&mut self) -> MdminW<RfhwienSpec> {
        MdminW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Interrupt enable for RFHWIFG.MDMOUT."]
    #[inline(always)]
    #[must_use]
    pub fn mdmout(&mut self) -> MdmoutW<RfhwienSpec> {
        MdmoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Interrupt enable for RFHWIFG.MDMSOFT."]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft(&mut self) -> MdmsoftW<RfhwienSpec> {
        MdmsoftW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Interrupt enable for RFHWIFG.TRCTK."]
    #[inline(always)]
    #[must_use]
    pub fn trctk(&mut self) -> TrctkW<RfhwienSpec> {
        TrctkW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<RfhwienSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Interrupt enable for RFHWIFG.RFEDONE."]
    #[inline(always)]
    #[must_use]
    pub fn rfedone(&mut self) -> RfedoneW<RfhwienSpec> {
        RfedoneW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Interrupt enable for RFHWIFG.RFESOFT0."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft0(&mut self) -> Rfesoft0W<RfhwienSpec> {
        Rfesoft0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
Interrupt enable for RFHWIFG.RFESOFT1."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft1(&mut self) -> Rfesoft1W<RfhwienSpec> {
        Rfesoft1W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Interrupt enable for RFHWIFG.RFESOFT2."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft2(&mut self) -> Rfesoft2W<RfhwienSpec> {
        Rfesoft2W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Interrupt enable for RFHWIFG.RATCH0."]
    #[inline(always)]
    #[must_use]
    pub fn ratch0(&mut self) -> Ratch0W<RfhwienSpec> {
        Ratch0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Interrupt enable for RFHWIFG.RATCH1."]
    #[inline(always)]
    #[must_use]
    pub fn ratch1(&mut self) -> Ratch1W<RfhwienSpec> {
        Ratch1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Interrupt enable for RFHWIFG.RATCH2."]
    #[inline(always)]
    #[must_use]
    pub fn ratch2(&mut self) -> Ratch2W<RfhwienSpec> {
        Ratch2W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Interrupt enable for RFHWIFG.RATCH3."]
    #[inline(always)]
    #[must_use]
    pub fn ratch3(&mut self) -> Ratch3W<RfhwienSpec> {
        Ratch3W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Interrupt enable for RFHWIFG.RATCH4."]
    #[inline(always)]
    #[must_use]
    pub fn ratch4(&mut self) -> Ratch4W<RfhwienSpec> {
        Ratch4W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Interrupt enable for RFHWIFG.RATCH5."]
    #[inline(always)]
    #[must_use]
    pub fn ratch5(&mut self) -> Ratch5W<RfhwienSpec> {
        Ratch5W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Interrupt enable for RFHWIFG.RATCH6."]
    #[inline(always)]
    #[must_use]
    pub fn ratch6(&mut self) -> Ratch6W<RfhwienSpec> {
        Ratch6W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Interrupt enable for RFHWIFG.RATCH7."]
    #[inline(always)]
    #[must_use]
    pub fn ratch7(&mut self) -> Ratch7W<RfhwienSpec> {
        Ratch7W::new(self, 19)
    }
}
#[doc = "Interrupt Enable For RF Hardware Modules\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfhwien::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfhwien::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfhwienSpec;
impl crate::RegisterSpec for RfhwienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfhwien::R`](R) reader structure"]
impl crate::Readable for RfhwienSpec {}
#[doc = "`write(|w| ..)` method takes [`rfhwien::W`](W) writer structure"]
impl crate::Writable for RfhwienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFHWIEN to value 0"]
impl crate::Resettable for RfhwienSpec {
    const RESET_VALUE: u32 = 0;
}
