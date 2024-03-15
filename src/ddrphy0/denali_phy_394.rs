#[doc = "Register `DENALI_PHY_394` reader"]
pub type R = crate::R<DenaliPhy394Spec>;
#[doc = "Register `DENALI_PHY_394` writer"]
pub type W = crate::W<DenaliPhy394Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_3` reader - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
pub type PhyLp4BootRddqsLatencyAdjust3R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_3` writer - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
pub type PhyLp4BootRddqsLatencyAdjust3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LPBK_CONTROL_3` reader - Loopback control bits for slice 3."]
pub type PhyLpbkControl3R = crate::FieldReader;
#[doc = "Field `PHY_LPBK_CONTROL_3` writer - Loopback control bits for slice 3."]
pub type PhyLpbkControl3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_3` reader - data slice power reduction disable for slice 3."]
pub type PhySlicePwrRdcDisable3R = crate::BitReader;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_3` writer - data slice power reduction disable for slice 3."]
pub type PhySlicePwrRdcDisable3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_3(&self) -> PhyLp4BootRddqsLatencyAdjust3R {
        PhyLp4BootRddqsLatencyAdjust3R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 3."]
    #[inline(always)]
    pub fn phy_lpbk_control_3(&self) -> PhyLpbkControl3R {
        PhyLpbkControl3R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 3."]
    #[inline(always)]
    pub fn phy_slice_pwr_rdc_disable_3(&self) -> PhySlicePwrRdcDisable3R {
        PhySlicePwrRdcDisable3R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_3(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust3W<DenaliPhy394Spec> {
        PhyLp4BootRddqsLatencyAdjust3W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_3(&mut self) -> PhyLpbkControl3W<DenaliPhy394Spec> {
        PhyLpbkControl3W::new(self, 8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slice_pwr_rdc_disable_3(&mut self) -> PhySlicePwrRdcDisable3W<DenaliPhy394Spec> {
        PhySlicePwrRdcDisable3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_394::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_394::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy394Spec;
impl crate::RegisterSpec for DenaliPhy394Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_394::R`](R) reader structure"]
impl crate::Readable for DenaliPhy394Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_394::W`](W) writer structure"]
impl crate::Writable for DenaliPhy394Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_394 to value 0"]
impl crate::Resettable for DenaliPhy394Spec {
    const RESET_VALUE: u32 = 0;
}
