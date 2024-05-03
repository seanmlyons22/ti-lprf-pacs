#[doc = "Register `RFHWIFG` reader"]
pub type R = crate::R<RfhwifgSpec>;
#[doc = "Register `RFHWIFG` writer"]
pub type W = crate::W<RfhwifgSpec>;
#[doc = "Field `RESERVED0` reader - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `RESERVED0` writer - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSCA` reader - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type FscaR = crate::BitReader;
#[doc = "Field `FSCA` writer - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type FscaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMDONE` reader - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdmdoneR = crate::BitReader;
#[doc = "Field `MDMDONE` writer - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdmdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMIN` reader - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdminR = crate::BitReader;
#[doc = "Field `MDMIN` writer - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdminW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMOUT` reader - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdmoutR = crate::BitReader;
#[doc = "Field `MDMOUT` writer - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type MdmoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDMSOFT` reader - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
pub type MdmsoftR = crate::BitReader;
#[doc = "Field `MDMSOFT` writer - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
pub type MdmsoftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRCTK` reader - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type TrctkR = crate::BitReader;
#[doc = "Field `TRCTK` writer - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type TrctkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED7` reader - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `RESERVED7` writer - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFEDONE` reader - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RfedoneR = crate::BitReader;
#[doc = "Field `RFEDONE` writer - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type RfedoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT0` reader - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft0R = crate::BitReader;
#[doc = "Field `RFESOFT0` writer - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT1` reader - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft1R = crate::BitReader;
#[doc = "Field `RFESOFT1` writer - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFESOFT2` reader - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft2R = crate::BitReader;
#[doc = "Field `RFESOFT2` writer - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
pub type Rfesoft2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH0` reader - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch0R = crate::BitReader;
#[doc = "Field `RATCH0` writer - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH1` reader - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch1R = crate::BitReader;
#[doc = "Field `RATCH1` writer - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH2` reader - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch2R = crate::BitReader;
#[doc = "Field `RATCH2` writer - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH3` reader - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch3R = crate::BitReader;
#[doc = "Field `RATCH3` writer - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH4` reader - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch4R = crate::BitReader;
#[doc = "Field `RATCH4` writer - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH5` reader - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch5R = crate::BitReader;
#[doc = "Field `RATCH5` writer - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH6` reader - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch6R = crate::BitReader;
#[doc = "Field `RATCH6` writer - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RATCH7` reader - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch7R = crate::BitReader;
#[doc = "Field `RATCH7` writer - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
pub type Ratch7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved20W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn fsca(&self) -> FscaR {
        FscaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmdone(&self) -> MdmdoneR {
        MdmdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmin(&self) -> MdminR {
        MdminR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmout(&self) -> MdmoutR {
        MdmoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn mdmsoft(&self) -> MdmsoftR {
        MdmsoftR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
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
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfedone(&self) -> RfedoneR {
        RfedoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft0(&self) -> Rfesoft0R {
        Rfesoft0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft1(&self) -> Rfesoft1R {
        Rfesoft1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn rfesoft2(&self) -> Rfesoft2R {
        Rfesoft2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch0(&self) -> Ratch0R {
        Ratch0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch1(&self) -> Ratch1R {
        Ratch1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch2(&self) -> Ratch2R {
        Ratch2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch3(&self) -> Ratch3R {
        Ratch3R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch4(&self) -> Ratch4R {
        Ratch4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch5(&self) -> Ratch5R {
        Ratch5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    pub fn ratch6(&self) -> Ratch6R {
        Ratch6R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
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
    pub fn reserved0(&mut self) -> Reserved0W<RfhwifgSpec> {
        Reserved0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Frequency synthesizer calibration accelerator interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn fsca(&mut self) -> FscaW<RfhwifgSpec> {
        FscaW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Modem command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmdone(&mut self) -> MdmdoneW<RfhwifgSpec> {
        MdmdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Modem FIFO input interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmin(&mut self) -> MdminW<RfhwifgSpec> {
        MdminW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Modem FIFO output interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmout(&mut self) -> MdmoutW<RfhwifgSpec> {
        MdmoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Modem synchronization word detection interrupt flag. This interrupt will be raised by modem when the synchronization word is received. The CPE may decide to reject the packet based on its header (protocol specific). Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn mdmsoft(&mut self) -> MdmsoftW<RfhwifgSpec> {
        MdmsoftW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Debug tracer system tick interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn trctk(&mut self) -> TrctkW<RfhwifgSpec> {
        TrctkW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved7(&mut self) -> Reserved7W<RfhwifgSpec> {
        Reserved7W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
RF engine command done interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfedone(&mut self) -> RfedoneW<RfhwifgSpec> {
        RfedoneW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
RF engine software defined interrupt 0 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft0(&mut self) -> Rfesoft0W<RfhwifgSpec> {
        Rfesoft0W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
RF engine software defined interrupt 1 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft1(&mut self) -> Rfesoft1W<RfhwifgSpec> {
        Rfesoft1W::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
RF engine software defined interrupt 2 flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn rfesoft2(&mut self) -> Rfesoft2W<RfhwifgSpec> {
        Rfesoft2W::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Radio timer channel 0 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch0(&mut self) -> Ratch0W<RfhwifgSpec> {
        Ratch0W::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
Radio timer channel 1 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch1(&mut self) -> Ratch1W<RfhwifgSpec> {
        Ratch1W::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
Radio timer channel 2 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch2(&mut self) -> Ratch2W<RfhwifgSpec> {
        Ratch2W::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Radio timer channel 3 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch3(&mut self) -> Ratch3W<RfhwifgSpec> {
        Ratch3W::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Radio timer channel 4 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch4(&mut self) -> Ratch4W<RfhwifgSpec> {
        Ratch4W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Radio timer channel 5 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch5(&mut self) -> Ratch5W<RfhwifgSpec> {
        Ratch5W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Radio timer channel 6 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch6(&mut self) -> Ratch6W<RfhwifgSpec> {
        Ratch6W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Radio timer channel 7 interrupt flag. Write zero to clear flag. Write to one has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn ratch7(&mut self) -> Ratch7W<RfhwifgSpec> {
        Ratch7W::new(self, 19)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> Reserved20W<RfhwifgSpec> {
        Reserved20W::new(self, 20)
    }
}
#[doc = "Interrupt Flags From RF Hardware Modules\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfhwifg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfhwifg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfhwifgSpec;
impl crate::RegisterSpec for RfhwifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfhwifg::R`](R) reader structure"]
impl crate::Readable for RfhwifgSpec {}
#[doc = "`write(|w| ..)` method takes [`rfhwifg::W`](W) writer structure"]
impl crate::Writable for RfhwifgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFHWIFG to value 0"]
impl crate::Resettable for RfhwifgSpec {
    const RESET_VALUE: u32 = 0;
}
