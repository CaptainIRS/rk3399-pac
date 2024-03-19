#[doc = "Register `PHY_POL0` reader"]
pub type R = crate::R<PhyPol0Spec>;
#[doc = "Register `PHY_POL0` writer"]
pub type W = crate::W<PhyPol0Spec>;
#[doc = "Field `TX_PHY_LOCK` reader - Polarity bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
pub type TxPhyLockR = crate::BitReader;
#[doc = "Field `TX_PHY_LOCK` writer - Polarity bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
pub type TxPhyLockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPD` reader - Polarity bit for PHY_INT0.HPD interrupt bit"]
pub type HpdR = crate::BitReader;
#[doc = "Field `HPD` writer - Polarity bit for PHY_INT0.HPD interrupt bit"]
pub type HpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_0` reader - Polarity bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
pub type RxSense0R = crate::BitReader;
#[doc = "Field `RX_SENSE_0` writer - Polarity bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
pub type RxSense0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_1` reader - Polarity bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
pub type RxSense1R = crate::BitReader;
#[doc = "Field `RX_SENSE_1` writer - Polarity bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
pub type RxSense1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_2` reader - Polarity bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
pub type RxSense2R = crate::BitReader;
#[doc = "Field `RX_SENSE_2` writer - Polarity bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
pub type RxSense2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_SENSE_3` reader - Polarity bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
pub type RxSense3R = crate::BitReader;
#[doc = "Field `RX_SENSE_3` writer - Polarity bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
pub type RxSense3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Polarity bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
    #[inline(always)]
    pub fn tx_phy_lock(&self) -> TxPhyLockR {
        TxPhyLockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity bit for PHY_INT0.HPD interrupt bit"]
    #[inline(always)]
    pub fn hpd(&self) -> HpdR {
        HpdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Polarity bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_0(&self) -> RxSense0R {
        RxSense0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_1(&self) -> RxSense1R {
        RxSense1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Polarity bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_2(&self) -> RxSense2R {
        RxSense2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Polarity bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
    #[inline(always)]
    pub fn rx_sense_3(&self) -> RxSense3R {
        RxSense3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Polarity bit for PHY_INT0.TX_PHY_LOCK interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn tx_phy_lock(&mut self) -> TxPhyLockW<PhyPol0Spec> {
        TxPhyLockW::new(self, 0)
    }
    #[doc = "Bit 1 - Polarity bit for PHY_INT0.HPD interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn hpd(&mut self) -> HpdW<PhyPol0Spec> {
        HpdW::new(self, 1)
    }
    #[doc = "Bit 4 - Polarity bit for PHY_INT0.RX_SENSE\\[0\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_0(&mut self) -> RxSense0W<PhyPol0Spec> {
        RxSense0W::new(self, 4)
    }
    #[doc = "Bit 5 - Polarity bit for PHY_INT0.RX_SENSE\\[1\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_1(&mut self) -> RxSense1W<PhyPol0Spec> {
        RxSense1W::new(self, 5)
    }
    #[doc = "Bit 6 - Polarity bit for PHY_INT0.RX_SENSE\\[2\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_2(&mut self) -> RxSense2W<PhyPol0Spec> {
        RxSense2W::new(self, 6)
    }
    #[doc = "Bit 7 - Polarity bit for PHY_INT0.RX_SENSE\\[3\\]
interrupt bit"]
    #[inline(always)]
    #[must_use]
    pub fn rx_sense_3(&mut self) -> RxSense3W<PhyPol0Spec> {
        RxSense3W::new(self, 7)
    }
}
#[doc = "PHY RXSENSE, PLL Lock, and HPD Polarity Register Polarity register for\n\ngeneration of PHY_INT0 interrupts.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_pol0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_pol0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyPol0Spec;
impl crate::RegisterSpec for PhyPol0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_pol0::R`](R) reader structure"]
impl crate::Readable for PhyPol0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_pol0::W`](W) writer structure"]
impl crate::Writable for PhyPol0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_POL0 to value 0xf3"]
impl crate::Resettable for PhyPol0Spec {
    const RESET_VALUE: u8 = 0xf3;
}
