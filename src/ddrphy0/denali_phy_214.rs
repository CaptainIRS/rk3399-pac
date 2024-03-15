#[doc = "Register `DENALI_PHY_214` reader"]
pub type R = crate::R<DenaliPhy214Spec>;
#[doc = "Register `DENALI_PHY_214` writer"]
pub type W = crate::W<DenaliPhy214Spec>;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_1` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyRddataEnTselDly1R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_1` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
pub type PhyRddataEnTselDly1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SW_MASTER_MODE_1` reader - Master delay line override settings for slice 1. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode1R = crate::FieldReader;
#[doc = "Field `PHY_SW_MASTER_MODE_1` writer - Master delay line override settings for slice 1. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_MASTER_DELAY_START_1` reader - Start value for master delay line locking algorithm for slice 1."]
pub type PhyMasterDelayStart1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DELAY_START_1` writer - Start value for master delay line locking algorithm for slice 1."]
pub type PhyMasterDelayStart1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_rddata_en_tsel_dly_1(&self) -> PhyRddataEnTselDly1R {
        PhyRddataEnTselDly1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 1. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_sw_master_mode_1(&self) -> PhySwMasterMode1R {
        PhySwMasterMode1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 1."]
    #[inline(always)]
    pub fn phy_master_delay_start_1(&self) -> PhyMasterDelayStart1R {
        PhyMasterDelayStart1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_tsel_dly_1(&mut self) -> PhyRddataEnTselDly1W<DenaliPhy214Spec> {
        PhyRddataEnTselDly1W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 1. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_master_mode_1(&mut self) -> PhySwMasterMode1W<DenaliPhy214Spec> {
        PhySwMasterMode1W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_start_1(&mut self) -> PhyMasterDelayStart1W<DenaliPhy214Spec> {
        PhyMasterDelayStart1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_214::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_214::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy214Spec;
impl crate::RegisterSpec for DenaliPhy214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_214::R`](R) reader structure"]
impl crate::Readable for DenaliPhy214Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_214::W`](W) writer structure"]
impl crate::Writable for DenaliPhy214Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_214 to value 0"]
impl crate::Resettable for DenaliPhy214Spec {
    const RESET_VALUE: u32 = 0;
}
