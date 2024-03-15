#[doc = "Register `DENALI_PHY_469` reader"]
pub type R = crate::R<DenaliPhy469Spec>;
#[doc = "Register `DENALI_PHY_469` writer"]
pub type W = crate::W<DenaliPhy469Spec>;
#[doc = "Field `PHY_DQS_IE_TIMING_3` reader - Start/end timing values for DQS input enable signals for slice 3."]
pub type PhyDqsIeTiming3R = crate::FieldReader;
#[doc = "Field `PHY_DQS_IE_TIMING_3` writer - Start/end timing values for DQS input enable signals for slice 3."]
pub type PhyDqsIeTiming3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_3` reader - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
pub type PhyRddataEnIeDly3R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_IE_DLY_3` writer - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
pub type PhyRddataEnIeDly3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_IE_MODE_3` reader - Input enable mode bits for slice 3. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode3R = crate::FieldReader;
#[doc = "Field `PHY_IE_MODE_3` writer - Input enable mode bits for slice 3. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
pub type PhyIeMode3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_RDDATA_EN_DLY_3` reader - Number of cycles that the dfi_rddata_en signal is early for slice 3."]
pub type PhyRddataEnDly3R = crate::FieldReader;
#[doc = "Field `PHY_RDDATA_EN_DLY_3` writer - Number of cycles that the dfi_rddata_en signal is early for slice 3."]
pub type PhyRddataEnDly3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 3."]
    #[inline(always)]
    pub fn phy_dqs_ie_timing_3(&self) -> PhyDqsIeTiming3R {
        PhyDqsIeTiming3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
    #[inline(always)]
    pub fn phy_rddata_en_ie_dly_3(&self) -> PhyRddataEnIeDly3R {
        PhyRddataEnIeDly3R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 3. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    pub fn phy_ie_mode_3(&self) -> PhyIeMode3R {
        PhyIeMode3R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 3."]
    #[inline(always)]
    pub fn phy_rddata_en_dly_3(&self) -> PhyRddataEnDly3R {
        PhyRddataEnDly3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Start/end timing values for DQS input enable signals for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dqs_ie_timing_3(&mut self) -> PhyDqsIeTiming3W<DenaliPhy469Spec> {
        PhyDqsIeTiming3W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Number of cycles that the dfi_rddata_en signal is earlier than necessary for input enable generation for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_ie_dly_3(&mut self) -> PhyRddataEnIeDly3W<DenaliPhy469Spec> {
        PhyRddataEnIeDly3W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Input enable mode bits for slice 3. Bit (0) enables the mode where the input enables are always on; set to 1 to enable. Bit (1) disables the input enable on the DM signal; set to 1 to disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ie_mode_3(&mut self) -> PhyIeMode3W<DenaliPhy469Spec> {
        PhyIeMode3W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Number of cycles that the dfi_rddata_en signal is early for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddata_en_dly_3(&mut self) -> PhyRddataEnDly3W<DenaliPhy469Spec> {
        PhyRddataEnDly3W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_469::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_469::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy469Spec;
impl crate::RegisterSpec for DenaliPhy469Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_469::R`](R) reader structure"]
impl crate::Readable for DenaliPhy469Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_469::W`](W) writer structure"]
impl crate::Writable for DenaliPhy469Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_469 to value 0"]
impl crate::Resettable for DenaliPhy469Spec {
    const RESET_VALUE: u32 = 0;
}
