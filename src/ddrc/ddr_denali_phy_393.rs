#[doc = "Register `DDR_DENALI_PHY_393` reader"]
pub type R = crate::R<DdrDenaliPhy393Spec>;
#[doc = "Register `DDR_DENALI_PHY_393` writer"]
pub type W = crate::W<DdrDenaliPhy393Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_3` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
pub type PhyLp4BootRddataEnIeDly3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_3` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
pub type PhyLp4BootRddataEnIeDly3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_3` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 3."]
pub type PhyLp4BootRddataEnDly3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_3` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 3."]
pub type PhyLp4BootRddataEnDly3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_3` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 3."]
pub type PhyLp4BootRddataEnTselDly3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_3` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 3."]
pub type PhyLp4BootRddataEnTselDly3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_3` reader - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
pub type PhyLp4BootRptrUpdate3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_3` writer - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
pub type PhyLp4BootRptrUpdate3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_3(&self) -> PhyLp4BootRddataEnIeDly3R {
        PhyLp4BootRddataEnIeDly3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_3(&self) -> PhyLp4BootRddataEnDly3R {
        PhyLp4BootRddataEnDly3R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_3(&self) -> PhyLp4BootRddataEnTselDly3R {
        PhyLp4BootRddataEnTselDly3R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_3(&self) -> PhyLp4BootRptrUpdate3R {
        PhyLp4BootRptrUpdate3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_3(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly3W<DdrDenaliPhy393Spec> {
        PhyLp4BootRddataEnIeDly3W::new(self, 0)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_3(&mut self) -> PhyLp4BootRddataEnDly3W<DdrDenaliPhy393Spec> {
        PhyLp4BootRddataEnDly3W::new(self, 8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_3(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly3W<DdrDenaliPhy393Spec> {
        PhyLp4BootRddataEnTselDly3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_3(&mut self) -> PhyLp4BootRptrUpdate3W<DdrDenaliPhy393Spec> {
        PhyLp4BootRptrUpdate3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_393::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_393::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy393Spec;
impl crate::RegisterSpec for DdrDenaliPhy393Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_393::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy393Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_393::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy393Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_393 to value 0"]
impl crate::Resettable for DdrDenaliPhy393Spec {
    const RESET_VALUE: u32 = 0;
}
