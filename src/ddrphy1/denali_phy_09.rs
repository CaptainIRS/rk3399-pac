#[doc = "Register `DENALI_PHY_09` reader"]
pub type R = crate::R<DenaliPhy09Spec>;
#[doc = "Register `DENALI_PHY_09` writer"]
pub type W = crate::W<DenaliPhy09Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_0` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyLp4BootRddataEnIeDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_IE_DLY_0` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyLp4BootRddataEnIeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_0` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyLp4BootRddataEnDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_DLY_0` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyLp4BootRddataEnDly0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_0` reader - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyLp4BootRddataEnTselDly0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDATA_EN_TSEL_DLY_0` writer - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyLp4BootRddataEnTselDly0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_0` reader - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyLp4BootRptrUpdate0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RPTR_UPDATE_0` writer - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
pub type PhyLp4BootRptrUpdate0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_ie_dly_0(&self) -> PhyLp4BootRddataEnIeDly0R {
        PhyLp4BootRddataEnIeDly0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_dly_0(&self) -> PhyLp4BootRddataEnDly0R {
        PhyLp4BootRddataEnDly0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_0(&self) -> PhyLp4BootRddataEnTselDly0R {
        PhyLp4BootRddataEnTselDly0R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rptr_update_0(&self) -> PhyLp4BootRptrUpdate0R {
        PhyLp4BootRptrUpdate0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_ie_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnIeDly0W<DenaliPhy09Spec> {
        PhyLp4BootRddataEnIeDly0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_dly_0(&mut self) -> PhyLp4BootRddataEnDly0W<DenaliPhy09Spec> {
        PhyLp4BootRddataEnDly0W::new(self, 8)
    }
    #[doc = "Bits 16:19 - For LPDDR4 boot frequency, the number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddata_en_tsel_dly_0(
        &mut self,
    ) -> PhyLp4BootRddataEnTselDly0W<DenaliPhy09Spec> {
        PhyLp4BootRddataEnTselDly0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - For LPDDR4 boot frequency, the offset in cycles from the dfi_rddata_en signal to release data from the entry FIFO for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rptr_update_0(&mut self) -> PhyLp4BootRptrUpdate0W<DenaliPhy09Spec> {
        PhyLp4BootRptrUpdate0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_09::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_09::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy09Spec;
impl crate::RegisterSpec for DenaliPhy09Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_09::R`](R) reader structure"]
impl crate::Readable for DenaliPhy09Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_09::W`](W) writer structure"]
impl crate::Writable for DenaliPhy09Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_09 to value 0"]
impl crate::Resettable for DenaliPhy09Spec {
    const RESET_VALUE: u32 = 0;
}
