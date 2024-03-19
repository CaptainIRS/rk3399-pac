#[doc = "Register `DDR_DENALI_PHY_941` reader"]
pub type R = crate::R<DdrDenaliPhy941Spec>;
#[doc = "Register `DDR_DENALI_PHY_941` writer"]
pub type W = crate::W<DdrDenaliPhy941Spec>;
#[doc = "Field `PHY_ADRCTL_RX_CAL` reader - PHY address/control RX calibration controls."]
pub type PhyAdrctlRxCalR = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADRCTL_RX_CAL` writer - PHY address/control RX calibration controls."]
pub type PhyAdrctlRxCalW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PHY_ADRCTL_LP3_RX_CAL` reader - PHY CKE/RESET_N RX calibration controls."]
pub type PhyAdrctlLp3RxCalR = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADRCTL_LP3_RX_CAL` writer - PHY CKE/RESET_N RX calibration controls."]
pub type PhyAdrctlLp3RxCalW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - PHY address/control RX calibration controls."]
    #[inline(always)]
    pub fn phy_adrctl_rx_cal(&self) -> PhyAdrctlRxCalR {
        PhyAdrctlRxCalR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - PHY CKE/RESET_N RX calibration controls."]
    #[inline(always)]
    pub fn phy_adrctl_lp3_rx_cal(&self) -> PhyAdrctlLp3RxCalR {
        PhyAdrctlLp3RxCalR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - PHY address/control RX calibration controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_rx_cal(&mut self) -> PhyAdrctlRxCalW<DdrDenaliPhy941Spec> {
        PhyAdrctlRxCalW::new(self, 0)
    }
    #[doc = "Bits 16:28 - PHY CKE/RESET_N RX calibration controls."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_lp3_rx_cal(&mut self) -> PhyAdrctlLp3RxCalW<DdrDenaliPhy941Spec> {
        PhyAdrctlLp3RxCalW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_941::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_941::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy941Spec;
impl crate::RegisterSpec for DdrDenaliPhy941Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_941::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy941Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_941::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy941Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_941 to value 0"]
impl crate::Resettable for DdrDenaliPhy941Spec {
    const RESET_VALUE: u32 = 0;
}
