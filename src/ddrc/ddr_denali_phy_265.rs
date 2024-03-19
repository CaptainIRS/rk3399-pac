#[doc = "Register `DDR_DENALI_PHY_265` reader"]
pub type R = crate::R<DdrDenaliPhy265Spec>;
#[doc = "Register `DDR_DENALI_PHY_265` writer"]
pub type W = crate::W<DdrDenaliPhy265Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_2` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
pub type PhyLp4BootRddataEnIeDly2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_2` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
pub type PhyLp4BootRddataEnIeDly2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_2` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 2."]
pub type PhyLp4BootRddataEnDly2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_2` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 2."]
pub type PhyLp4BootRddataEnDly2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_2` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
pub type PhyLp4BootRddataEnTselDly2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_2` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
pub type PhyLp4BootRddataEnTselDly2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_2` reader - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
pub type PhyLp4BootRptrUpdate2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_2` writer - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
pub type PhyLp4BootRptrUpdate2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_2(&self) -> PhyLp4BootRddataEnIeDly2R {
        PhyLp4BootRddataEnIeDly2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_2(&self) -> PhyLp4BootRddataEnDly2R {
        PhyLp4BootRddataEnDly2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_2(&self) -> PhyLp4BootRddataEnTselDly2R {
        PhyLp4BootRddataEnTselDly2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_2(&self) -> PhyLp4BootRptrUpdate2R {
        PhyLp4BootRptrUpdate2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_2(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly2W<DdrDenaliPhy265Spec> {
        PhyLp4BootRddataEnIeDly2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_2(&mut self) -> PhyLp4BootRddataEnDly2W<DdrDenaliPhy265Spec> {
        PhyLp4BootRddataEnDly2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_2(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly2W<DdrDenaliPhy265Spec> {
        PhyLp4BootRddataEnTselDly2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_2(&mut self) -> PhyLp4BootRptrUpdate2W<DdrDenaliPhy265Spec> {
        PhyLp4BootRptrUpdate2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_265::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_265::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy265Spec;
impl crate::RegisterSpec for DdrDenaliPhy265Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_265::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy265Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_265::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy265Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_265 to value 0"]
impl crate::Resettable for DdrDenaliPhy265Spec {
    const RESET_VALUE: u32 = 0;
}
