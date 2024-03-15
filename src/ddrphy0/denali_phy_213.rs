#[doc = "Register `DENALI_PHY_213` reader"]
pub type R = crate::R<DenaliPhy213Spec>;
#[doc = "Register `DENALI_PHY_213` writer"]
pub type W = crate::W<DenaliPhy213Spec>;
#[doc = "Field `PHY_DQS_IE_TIMING_1` reader - Start/end timing values for DQS input enable signals for slice 1."]
pub type PhyDqsIeTiming1R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_1` writer - Start/end timing values for DQS input enable signals for slice 1."]
pub type PhyDqsIeTiming1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_1` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyRddataEnIeDly1R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_1` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
pub type PhyRddataEnIeDly1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_1` reader - Input enable mode bits for slice 1. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode1R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_1` writer - Input enable mode bits for slice 1. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDDATA_EN_DLY_1` reader - Number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyRddataEnDly1R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_DLY_1` writer - Number of cycles that the dfi_rddata_en signal is early for slice 1."]
pub type PhyRddataEnDly1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 1."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_1(&self) -> PhyDqsIeTiming1R {
        PhyDqsIeTiming1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_1(&self) -> PhyRddataEnIeDly1R {
        PhyRddataEnIeDly1R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 1. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_1(&self) -> PhyIeMode1R {
        PhyIeMode1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    pub fn phy_rddata_en_dly_1(&self) -> PhyRddataEnDly1R {
        PhyRddataEnDly1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_1(&mut self) -> PhyDqsIeTiming1W<DenaliPhy213Spec> {
        PhyDqsIeTiming1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_1(&mut self) -> PhyRddataEnIeDly1W<DenaliPhy213Spec> {
        PhyRddataEnIeDly1W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 1. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_1(&mut self) -> PhyIeMode1W<DenaliPhy213Spec> {
        PhyIeMode1W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_dly_1(&mut self) -> PhyRddataEnDly1W<DenaliPhy213Spec> {
        PhyRddataEnDly1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_213::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_213::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy213Spec;
impl crate::RegisterSpec for DenaliPhy213Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_213::R`](R) reader structure"]
impl crate::Readable for DenaliPhy213Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_213::W`](W) writer structure"]
impl crate::Writable for DenaliPhy213Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_213 to value 0"]
impl crate::Resettable for DenaliPhy213Spec {
    const RESET_VALUE: u32 = 0;
}
