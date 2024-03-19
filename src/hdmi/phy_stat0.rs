#[doc = "Register `PHY_STAT0` reader"]
pub type R = crate::R<PhyStat0Spec>;
#[doc = "Field `TX_PHY_LOCK` reader - Status bit. TX PHY PLL lock indication. You may need\n\nto mask or change polarity of this interrupt after it\n\nhas became active."]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `HPD` reader - Status bit. HDMI Hot Plug Detect indication. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has became active."]
pub type HpdR = crate::BitReader;
#[doc = "Field `RX_SENSE_0` reader - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 0 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` reader - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 1 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` reader - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 2 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` reader - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 3 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
pub type RxSense3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status bit. TX PHY PLL lock indication. You may need\n\nto mask or change polarity of this interrupt after it\n\nhas became active."]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status bit. HDMI Hot Plug Detect indication. You may\n\nneed to mask or change polarity of this interrupt\n\nafter it has became active."]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 0 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 1 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 2 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status bit. TX PHY RX_SENSE indication for TMDS\n\nchannel 3 driver. You may need to mask or change\n\npolarity of this interrupt after it has became active."]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PHY RXSENSE, PLL Lock, and HPD Status Register\n\nThis register contains the following active high packet sent status indications.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_stat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyStat0Spec;
impl crate::RegisterSpec for PhyStat0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_stat0::R`](R) reader structure"]
impl crate::Readable for PhyStat0Spec {}
#[doc = "`reset()` method sets PHY_STAT0 to value 0"]
impl crate::Resettable for PhyStat0Spec {
    const RESET_VALUE: u8 = 0;
}
