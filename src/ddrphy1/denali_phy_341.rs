#[doc = "Register `DENALI_PHY_341` reader"]
pub type R = crate::R<DenaliPhy341Spec>;
#[doc = "Register `DENALI_PHY_341` writer"]
pub type W = crate::W<DenaliPhy341Spec>;
#[doc = "Field `PHY_DQS_IE_TIMING_2` reader - Start/end timing values for DQS input enable signals for slice 2."]
pub type PhyDqsIeTiming2R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_2` writer - Start/end timing values for DQS input enable signals for slice 2."]
pub type PhyDqsIeTiming2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_2` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
pub type PhyRddataEnIeDly2R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_2` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
pub type PhyRddataEnIeDly2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_2` reader - Input enable mode bits for slice 2. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode2R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_2` writer - Input enable mode bits for slice 2. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDDATA_EN_DLY_2` reader - Number of cycles that the dfi_rddata_en signal is early for slice 2."]
pub type PhyRddataEnDly2R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_DLY_2` writer - Number of cycles that the dfi_rddata_en signal is early for slice 2."]
pub type PhyRddataEnDly2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 2."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_2(&self) -> PhyDqsIeTiming2R {
        PhyDqsIeTiming2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_2(&self) -> PhyRddataEnIeDly2R {
        PhyRddataEnIeDly2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 2. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_2(&self) -> PhyIeMode2R {
        PhyIeMode2R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 2."]
    #[inline(always)]
    pub fn phy_rddata_en_dly_2(&self) -> PhyRddataEnDly2R {
        PhyRddataEnDly2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_2(&mut self) -> PhyDqsIeTiming2W<DenaliPhy341Spec> {
        PhyDqsIeTiming2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_2(&mut self) -> PhyRddataEnIeDly2W<DenaliPhy341Spec> {
        PhyRddataEnIeDly2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 2. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_2(&mut self) -> PhyIeMode2W<DenaliPhy341Spec> {
        PhyIeMode2W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_dly_2(&mut self) -> PhyRddataEnDly2W<DenaliPhy341Spec> {
        PhyRddataEnDly2W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_341::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_341::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy341Spec;
impl crate::RegisterSpec for DenaliPhy341Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_341::R`](R) reader structure"]
impl crate::Readable for DenaliPhy341Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_341::W`](W) writer structure"]
impl crate::Writable for DenaliPhy341Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_341 to value 0"]
impl crate::Resettable for DenaliPhy341Spec {
    const RESET_VALUE: u32 = 0;
}
