#[doc = "Register `DDR_DENALI_PHY_950` reader"]
pub type R = crate::R<DdrDenaliPhy950Spec>;
#[doc = "Register `DDR_DENALI_PHY_950` writer"]
pub type W = crate::W<DdrDenaliPhy950Spec>;
#[doc = "Field `PHY_PAD_ATB_CTRL` reader - Pad ATB control settings. Bit (0) is the enable signal. Bits (5:1) are the ATB data signals. Bits (15:8) are the 1 hot select for which pad is selected."]
pub type PhyPadAtbCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_ATB_CTRL` writer - Pad ATB control settings. Bit (0) is the enable signal. Bits (5:1) are the ATB data signals. Bits (15:8) are the 1 hot select for which pad is selected."]
pub type PhyPadAtbCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ADRCTL_MANUAL_UPDATE` writer - Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
pub type PhyAdrctlManualUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_AC_LPBK_ERR_CLEAR` writer - Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
pub type PhyAcLpbkErrClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Pad ATB control settings. Bit (0) is the enable signal. Bits (5:1) are the ATB data signals. Bits (15:8) are the 1 hot select for which pad is selected."]
    #[inline(always)]
    pub fn phy_pad_atb_ctrl(&self) -> PhyPadAtbCtrlR {
        PhyPadAtbCtrlR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pad ATB control settings. Bit (0) is the enable signal. Bits (5:1) are the ATB data signals. Bits (15:8) are the 1 hot select for which pad is selected."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_atb_ctrl(&mut self) -> PhyPadAtbCtrlW<DdrDenaliPhy950Spec> {
        PhyPadAtbCtrlW::new(self, 0)
    }
    #[doc = "Bit 16 - Address/control manual update of slave delay lines. Set to 1 to update. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_manual_update(&mut self) -> PhyAdrctlManualUpdateW<DdrDenaliPhy950Spec> {
        PhyAdrctlManualUpdateW::new(self, 16)
    }
    #[doc = "Bit 24 - Address/control loopback error clear. Set to 1 to clear error. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_err_clear(&mut self) -> PhyAcLpbkErrClearW<DdrDenaliPhy950Spec> {
        PhyAcLpbkErrClearW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_950::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_950::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy950Spec;
impl crate::RegisterSpec for DdrDenaliPhy950Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_950::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy950Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_950::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy950Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_950 to value 0"]
impl crate::Resettable for DdrDenaliPhy950Spec {
    const RESET_VALUE: u32 = 0;
}
