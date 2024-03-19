#[doc = "Register `DDR_DENALI_PHY_10` reader"]
pub type R = crate::R<DdrDenaliPhy10Spec>;
#[doc = "Register `DDR_DENALI_PHY_10` writer"]
pub type W = crate::W<DdrDenaliPhy10Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_0` reader - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyLp4BootRddqsLatencyAdjust0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_0` writer - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
pub type PhyLp4BootRddqsLatencyAdjust0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LPBK_CONTROL_0` reader - Loopback control bits for slice 0."]
pub type PhyLpbkControl0R = crate::FieldReader;
#[doc = "Field `PHY_LPBK_CONTROL_0` writer - Loopback control bits for slice 0."]
pub type PhyLpbkControl0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_0` reader - data slice power reduction disable for slice 0."]
pub type PhySlicePwrRdcDisable0R = crate::BitReader;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_0` writer - data slice power reduction disable for slice 0."]
pub type PhySlicePwrRdcDisable0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_0(&self) -> PhyLp4BootRddqsLatencyAdjust0R {
        PhyLp4BootRddqsLatencyAdjust0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 0."]
    #[inline(always)]
    pub fn phy_lpbk_control_0(&self) -> PhyLpbkControl0R {
        PhyLpbkControl0R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 0."]
    #[inline(always)]
    pub fn phy_slice_pwr_rdc_disable_0(&self) -> PhySlicePwrRdcDisable0R {
        PhySlicePwrRdcDisable0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_0(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust0W<DdrDenaliPhy10Spec> {
        PhyLp4BootRddqsLatencyAdjust0W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_0(&mut self) -> PhyLpbkControl0W<DdrDenaliPhy10Spec> {
        PhyLpbkControl0W::new(self, 8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slice_pwr_rdc_disable_0(&mut self) -> PhySlicePwrRdcDisable0W<DdrDenaliPhy10Spec> {
        PhySlicePwrRdcDisable0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy10Spec;
impl crate::RegisterSpec for DdrDenaliPhy10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_10::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy10Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_10::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_10 to value 0"]
impl crate::Resettable for DdrDenaliPhy10Spec {
    const RESET_VALUE: u32 = 0;
}
