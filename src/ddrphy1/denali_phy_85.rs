#[doc = "Register `DENALI_PHY_85` reader"]
pub type R = crate::R<DenaliPhy85Spec>;
#[doc = "Register `DENALI_PHY_85` writer"]
pub type W = crate::W<DenaliPhy85Spec>;
#[doc = "Field `PHY_DQS_IE_TIMING_0` reader - Start/end timing values for DQS input enable signals for slice 0."]
pub type PhyDqsIeTiming0R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_0` writer - Start/end timing values for DQS input enable signals for slice 0."]
pub type PhyDqsIeTiming0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_0` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyRddataEnIeDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_0` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
pub type PhyRddataEnIeDly0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_0` reader - Input enable mode bits for slice 0. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode0R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_0` writer - Input enable mode bits for slice 0. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDDATA_EN_DLY_0` reader - Number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyRddataEnDly0R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_DLY_0` writer - Number of cycles that the dfi_rddata_en signal is early for slice 0."]
pub type PhyRddataEnDly0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 0."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_0(&self) -> PhyDqsIeTiming0R {
        PhyDqsIeTiming0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_0(&self) -> PhyRddataEnIeDly0R {
        PhyRddataEnIeDly0R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 0. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_0(&self) -> PhyIeMode0R {
        PhyIeMode0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    pub fn phy_rddata_en_dly_0(&self) -> PhyRddataEnDly0R {
        PhyRddataEnDly0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_0(&mut self) -> PhyDqsIeTiming0W<DenaliPhy85Spec> {
        PhyDqsIeTiming0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_0(&mut self) -> PhyRddataEnIeDly0W<DenaliPhy85Spec> {
        PhyRddataEnIeDly0W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 0. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_0(&mut self) -> PhyIeMode0W<DenaliPhy85Spec> {
        PhyIeMode0W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_dly_0(&mut self) -> PhyRddataEnDly0W<DenaliPhy85Spec> {
        PhyRddataEnDly0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_85::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_85::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy85Spec;
impl crate::RegisterSpec for DenaliPhy85Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_85::R`](R) reader structure"]
impl crate::Readable for DenaliPhy85Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_85::W`](W) writer structure"]
impl crate::Writable for DenaliPhy85Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_85 to value 0"]
impl crate::Resettable for DenaliPhy85Spec {
    const RESET_VALUE: u32 = 0;
}
