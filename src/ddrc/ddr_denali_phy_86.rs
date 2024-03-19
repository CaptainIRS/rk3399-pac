#[doc = "Register `DDR_DENALI_PHY_86` reader"]
pub type R = crate::R<DdrDenaliPhy86Spec>;
#[doc = "Register `DDR_DENALI_PHY_86` writer"]
pub type W = crate::W<DdrDenaliPhy86Spec>;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_0` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyRddataEnTselDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_0` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
pub type PhyRddataEnTselDly0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SW_MASTER_MODE_0` reader - Master delay line override settings for slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode0R = crate::FieldReader;
#[doc = "Field `PHY_SW_MASTER_MODE_0` writer - Master delay line override settings for slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_MASTER_DELAY_START_0` reader - Start value for master delay line locking algorithm for slice 0."]
pub type PhyMasterDelayStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DELAY_START_0` writer - Start value for master delay line locking algorithm for slice 0."]
pub type PhyMasterDelayStart0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_tsel_dly_0(&self) -> PhyRddataEnTselDly0R {
        PhyRddataEnTselDly0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_sw_master_mode_0(&self) -> PhySwMasterMode0R {
        PhySwMasterMode0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 0."]
    #[inline(always)]
    pub fn phy_master_delay_start_0(&self) -> PhyMasterDelayStart0R {
        PhyMasterDelayStart0R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_tsel_dly_0(&mut self) -> PhyRddataEnTselDly0W<DdrDenaliPhy86Spec> {
        PhyRddataEnTselDly0W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 0. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_master_mode_0(&mut self) -> PhySwMasterMode0W<DdrDenaliPhy86Spec> {
        PhySwMasterMode0W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_start_0(&mut self) -> PhyMasterDelayStart0W<DdrDenaliPhy86Spec> {
        PhyMasterDelayStart0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_86::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_86::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy86Spec;
impl crate::RegisterSpec for DdrDenaliPhy86Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_86::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy86Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_86::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy86Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_86 to value 0"]
impl crate::Resettable for DdrDenaliPhy86Spec {
    const RESET_VALUE: u32 = 0;
}
