#[doc = "Register `PHY_INT0` reader"]
pub type R = crate::R<PhyInt0Spec>;
#[doc = "Field `TX_PHY_LOCK` reader - Interrupt indication bit. TX PHY PLL lock indication\n\ninterrupt."]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `HPD` reader - Interrupt indication bit. HDMI Hot Plug Detect\n\nindication interrupt."]
pub type HpdR = crate::BitReader;
#[doc = "Field `RX_SENSE_0` reader - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 0 driver."]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` reader - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 1 driver."]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` reader - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 2 driver."]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` reader - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS CLK driver."]
pub type RxSense3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt indication bit. TX PHY PLL lock indication\n\ninterrupt."]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt indication bit. HDMI Hot Plug Detect\n\nindication interrupt."]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 0 driver."]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 1 driver."]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS channel 2 driver."]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt indication bit. TX PHY RX_SENSE indication\n\ninterruption for TMDS CLK driver."]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PHY RXSENSE, PLL Lock, and HPD Interrupt Register\n\nThis register contains the interrupt indication of the PHY_STAT0 status interrupts. Interrupt\n\ngeneration is accomplished in the following way:\n\ninterrupt = (mask == 1'b0) &amp;&amp; (polarity == status);\n\nAll these interrupts are forwarded to the Interrupt Handler sticky bit register ih_phy_stat0\n\nand after ORed to a single main interrupt line to micro controller. Assertion of this interrupt\n\nimplies that data related with the corresponding packet has been sent through the HDMI\n\ninterface.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_int0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyInt0Spec;
impl crate::RegisterSpec for PhyInt0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_int0::R`](R) reader structure"]
impl crate::Readable for PhyInt0Spec {}
#[doc = "`reset()` method sets PHY_INT0 to value 0"]
impl crate::Resettable for PhyInt0Spec {
    const RESET_VALUE: u8 = 0;
}
