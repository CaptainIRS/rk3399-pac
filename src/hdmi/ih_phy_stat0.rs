#[doc = "Register `IH_PHY_STAT0` reader"]
pub type R = crate::R<IhPhyStat0Spec>;
#[doc = "Register `IH_PHY_STAT0` writer"]
pub type W = crate::W<IhPhyStat0Spec>;
#[doc = "Field `HPD` reader - HDMI Hot Plug Detect indication. You may need to\n\nmask or change polarity of this interrupt after it\n\nhas become active."]
pub type HpdR = crate::BitReader;
#[doc = "Field `HPD` writer - HDMI Hot Plug Detect indication. You may need to\n\nmask or change polarity of this interrupt after it\n\nhas become active."]
pub type HpdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_PHY_LOCK` reader - TX PHY PLL lock indication."]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `TX_PHY_LOCK` writer - TX PHY PLL lock indication."]
pub type TxPhyLockW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_SENSE_0` reader - TX PHY RX_SENSE indication for driver 0. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_0` writer - TX PHY RX_SENSE indication for driver 0. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_SENSE_1` reader - TX PHY RX_SENSE indication for driver 1. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` writer - TX PHY RX_SENSE indication for driver 1. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_SENSE_2` reader - TX PHY RX_SENSE indication for driver 2. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` writer - TX PHY RX_SENSE indication for driver 2. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_SENSE_3` reader - TX PHY RX_SENSE indication for driver 3. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense3R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` writer - TX PHY RX_SENSE indication for driver 3. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
pub type RxSense3W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - HDMI Hot Plug Detect indication. You may need to\n\nmask or change polarity of this interrupt after it\n\nhas become active."]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX PHY PLL lock indication."]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX PHY RX_SENSE indication for driver 0. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX PHY RX_SENSE indication for driver 1. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX PHY RX_SENSE indication for driver 2. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX PHY RX_SENSE indication for driver 3. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HDMI Hot Plug Detect indication. You may need to\n\nmask or change polarity of this interrupt after it\n\nhas become active."]
    #[inline(always)]
    #[must_use]
    pub fn hpd(&mut self) -> HpdW<IhPhyStat0Spec> {
        HpdW::new(self, 0)
    }
    #[doc = "Bit 1 - TX PHY PLL lock indication."]
    #[inline(always)]
    #[must_use]
    pub fn tx_phy_lock(&mut self) -> TxPhyLockW<IhPhyStat0Spec> {
        TxPhyLockW::new(self, 1)
    }
    #[doc = "Bit 2 - TX PHY RX_SENSE indication for driver 0. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_0(&mut self) -> RxSense0W<IhPhyStat0Spec> {
        RxSense0W::new(self, 2)
    }
    #[doc = "Bit 3 - TX PHY RX_SENSE indication for driver 1. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_1(&mut self) -> RxSense1W<IhPhyStat0Spec> {
        RxSense1W::new(self, 3)
    }
    #[doc = "Bit 4 - TX PHY RX_SENSE indication for driver 2. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_2(&mut self) -> RxSense2W<IhPhyStat0Spec> {
        RxSense2W::new(self, 4)
    }
    #[doc = "Bit 5 - TX PHY RX_SENSE indication for driver 3. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has become active."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_3(&mut self) -> RxSense3W<IhPhyStat0Spec> {
        RxSense3W::new(self, 5)
    }
}
#[doc = "PHY Interface Interrupt Status Register (RXSENSE, PLL Lock and HPD\n\nInterrupts)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_phy_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_phy_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhPhyStat0Spec;
impl crate::RegisterSpec for IhPhyStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_phy_stat0::R`](R) reader structure"]
impl crate::Readable for IhPhyStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_phy_stat0::W`](W) writer structure"]
impl crate::Writable for IhPhyStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x3f;
}
#[doc = "`reset()` method sets IH_PHY_STAT0 to value 0"]
impl crate::Resettable for IhPhyStat0Spec {
    const RESET_VALUE: u8 = 0;
}
