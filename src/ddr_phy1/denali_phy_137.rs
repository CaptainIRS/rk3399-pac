#[doc = "Register `DENALI_PHY_137` reader"]
pub type R = crate::R<DenaliPhy137Spec>;
#[doc = "Register `DENALI_PHY_137` writer"]
pub type W = crate::W<DenaliPhy137Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_1` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyLp4BootRddataEnIeDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_1` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyLp4BootRddataEnIeDly1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_1` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyLp4BootRddataEnDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_1` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyLp4BootRddataEnDly1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_1` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyLp4BootRddataEnTselDly1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_1` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyLp4BootRddataEnTselDly1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_1` reader - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyLp4BootRptrUpdate1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_1` writer - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
pub type PhyLp4BootRptrUpdate1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_1(&self) -> PhyLp4BootRddataEnIeDly1R {
        PhyLp4BootRddataEnIeDly1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_1(&self) -> PhyLp4BootRddataEnDly1R {
        PhyLp4BootRddataEnDly1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_1(&self) -> PhyLp4BootRddataEnTselDly1R {
        PhyLp4BootRddataEnTselDly1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_1(&self) -> PhyLp4BootRptrUpdate1R {
        PhyLp4BootRptrUpdate1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly1W<DenaliPhy137Spec> {
        PhyLp4BootRddataEnIeDly1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_1(&mut self) -> PhyLp4BootRddataEnDly1W<DenaliPhy137Spec> {
        PhyLp4BootRddataEnDly1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_1(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly1W<DenaliPhy137Spec> {
        PhyLp4BootRddataEnTselDly1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_1(&mut self) -> PhyLp4BootRptrUpdate1W<DenaliPhy137Spec> {
        PhyLp4BootRptrUpdate1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_137::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_137::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy137Spec;
impl crate::RegisterSpec for DenaliPhy137Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_137::R`](R) reader structure"]
impl crate::Readable for DenaliPhy137Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_137::W`](W) writer structure"]
impl crate::Writable for DenaliPhy137Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_137 to value 0"]
impl crate::Resettable for DenaliPhy137Spec {
    const RESET_VALUE: u32 = 0;
}
