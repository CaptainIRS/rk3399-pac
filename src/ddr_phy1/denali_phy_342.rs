#[doc = "Register `DENALI_PHY_342` reader"]
pub type R = crate::R<DenaliPhy342Spec>;
#[doc = "Register `DENALI_PHY_342` writer"]
pub type W = crate::W<DenaliPhy342Spec>;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_2` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
pub type PhyRddataEnTselDly2R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_TSEL_DLY_2` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
pub type PhyRddataEnTselDly2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_SW_MASTER_MODE_2` reader - Master delay line override settings for slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode2R = crate::FieldReader;
#[doc = "Field `PHY_SW_MASTER_MODE_2` writer - Master delay line override settings for slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhySwMasterMode2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_MASTER_DELAY_START_2` reader - Start value for master delay line locking algorithm for slice 2."]
pub type PhyMasterDelayStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_MASTER_DELAY_START_2` writer - Start value for master delay line locking algorithm for slice 2."]
pub type PhyMasterDelayStart2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
    #[inline(always)]
    pub fn phy_rddata_en_tsel_dly_2(&self) -> PhyRddataEnTselDly2R {
        PhyRddataEnTselDly2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_sw_master_mode_2(&self) -> PhySwMasterMode2R {
        PhySwMasterMode2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 2."]
    #[inline(always)]
    pub fn phy_master_delay_start_2(&self) -> PhyMasterDelayStart2R {
        PhyMasterDelayStart2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for TSEL enable generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_tsel_dly_2(&mut self) -> PhyRddataEnTselDly2W<DenaliPhy342Spec> {
        PhyRddataEnTselDly2W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Master delay line override settings for slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_sw_master_mode_2(&mut self) -> PhySwMasterMode2W<DenaliPhy342Spec> {
        PhySwMasterMode2W::new(self, 8)
    }
    #[doc = "Bits 16:25 - Start value for master delay line locking algorithm for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_master_delay_start_2(&mut self) -> PhyMasterDelayStart2W<DenaliPhy342Spec> {
        PhyMasterDelayStart2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_342::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_342::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy342Spec;
impl crate::RegisterSpec for DenaliPhy342Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_342::R`](R) reader structure"]
impl crate::Readable for DenaliPhy342Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_342::W`](W) writer structure"]
impl crate::Writable for DenaliPhy342Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_342 to value 0"]
impl crate::Resettable for DenaliPhy342Spec {
    const RESET_VALUE: u32 = 0;
}
