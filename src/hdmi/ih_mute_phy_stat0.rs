#[doc = "Register `IH_MUTE_PHY_STAT0` reader"]
pub type R = crate::R<IhMutePhyStat0Spec>;
#[doc = "Register `IH_MUTE_PHY_STAT0` writer"]
pub type W = crate::W<IhMutePhyStat0Spec>;
#[doc = "Field `HPD` reader - When set to 1, mutes ih_phy_stat0\\[0\\]"]
pub type HpdR = crate::BitReader;
#[doc = "Field `HPD` writer - When set to 1, mutes ih_phy_stat0\\[0\\]"]
pub type HpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PHY_LOCK` reader - When set to 1, mutes ih_phy_stat0\\[1\\]"]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `TX_PHY_LOCK` writer - When set to 1, mutes ih_phy_stat0\\[1\\]"]
pub type TxPhyLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_0` reader - When set to 1, mutes ih_phy_stat0\\[2\\]"]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_0` writer - When set to 1, mutes ih_phy_stat0\\[2\\]"]
pub type RxSense0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_1` reader - When set to 1, mutes ih_phy_stat0\\[3\\]"]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` writer - When set to 1, mutes ih_phy_stat0\\[3\\]"]
pub type RxSense1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_2` reader - When set to 1, mutes ih_phy_stat0\\[4\\]"]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` writer - When set to 1, mutes ih_phy_stat0\\[4\\]"]
pub type RxSense2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_3` reader - When set to 1, mutes ih_phy_stat0\\[5\\]"]
pub type RxSense3R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` writer - When set to 1, mutes ih_phy_stat0\\[5\\]"]
pub type RxSense3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set to 1, mutes ih_phy_stat0\\[0\\]"]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_phy_stat0\\[1\\]"]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_phy_stat0\\[2\\]"]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_phy_stat0\\[3\\]"]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_phy_stat0\\[4\\]"]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_phy_stat0\\[5\\]"]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, mutes ih_phy_stat0\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn hpd(&mut self) -> HpdW<IhMutePhyStat0Spec> {
        HpdW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1, mutes ih_phy_stat0\\[1\\]"]
    #[inline(always)]
    #[must_use]
    pub fn tx_phy_lock(&mut self) -> TxPhyLockW<IhMutePhyStat0Spec> {
        TxPhyLockW::new(self, 1)
    }
    #[doc = "Bit 2 - When set to 1, mutes ih_phy_stat0\\[2\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_0(&mut self) -> RxSense0W<IhMutePhyStat0Spec> {
        RxSense0W::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, mutes ih_phy_stat0\\[3\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_1(&mut self) -> RxSense1W<IhMutePhyStat0Spec> {
        RxSense1W::new(self, 3)
    }
    #[doc = "Bit 4 - When set to 1, mutes ih_phy_stat0\\[4\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_2(&mut self) -> RxSense2W<IhMutePhyStat0Spec> {
        RxSense2W::new(self, 4)
    }
    #[doc = "Bit 5 - When set to 1, mutes ih_phy_stat0\\[5\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_3(&mut self) -> RxSense3W<IhMutePhyStat0Spec> {
        RxSense3W::new(self, 5)
    }
}
#[doc = "PHY Interface Interrupt Mute Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ih_mute_phy_stat0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ih_mute_phy_stat0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhMutePhyStat0Spec;
impl crate::RegisterSpec for IhMutePhyStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ih_mute_phy_stat0::R`](R) reader structure"]
impl crate::Readable for IhMutePhyStat0Spec {}
#[doc = "`write(|w| ..)` method takes [`ih_mute_phy_stat0::W`](W) writer structure"]
impl crate::Writable for IhMutePhyStat0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IH_MUTE_PHY_STAT0 to value 0"]
impl crate::Resettable for IhMutePhyStat0Spec {
    const RESET_VALUE: u8 = 0;
}
