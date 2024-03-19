#[doc = "Register `DDR_DENALI_PHY_138` reader"]
pub type R = crate::R<DdrDenaliPhy138Spec>;
#[doc = "Register `DDR_DENALI_PHY_138` writer"]
pub type W = crate::W<DdrDenaliPhy138Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_1` reader - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyLp4BootRddqsLatencyAdjust1R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_1` writer - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
pub type PhyLp4BootRddqsLatencyAdjust1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LPBK_CONTROL_1` reader - Loopback control bits for slice 1."]
pub type PhyLpbkControl1R = crate::FieldReader;
#[doc = "Field `PHY_LPBK_CONTROL_1` writer - Loopback control bits for slice 1."]
pub type PhyLpbkControl1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_1` reader - data slice power reduction disable for slice 1."]
pub type PhySlicePwrRdcDisable1R = crate::BitReader;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_1` writer - data slice power reduction disable for slice 1."]
pub type PhySlicePwrRdcDisable1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_1(&self) -> PhyLp4BootRddqsLatencyAdjust1R {
        PhyLp4BootRddqsLatencyAdjust1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 1."]
    #[inline(always)]
    pub fn phy_lpbk_control_1(&self) -> PhyLpbkControl1R {
        PhyLpbkControl1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 1."]
    #[inline(always)]
    pub fn phy_slice_pwr_rdc_disable_1(&self) -> PhySlicePwrRdcDisable1R {
        PhySlicePwrRdcDisable1R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_1(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust1W<DdrDenaliPhy138Spec> {
        PhyLp4BootRddqsLatencyAdjust1W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_1(&mut self) -> PhyLpbkControl1W<DdrDenaliPhy138Spec> {
        PhyLpbkControl1W::new(self, 8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slice_pwr_rdc_disable_1(&mut self) -> PhySlicePwrRdcDisable1W<DdrDenaliPhy138Spec> {
        PhySlicePwrRdcDisable1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_138::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_138::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy138Spec;
impl crate::RegisterSpec for DdrDenaliPhy138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_138::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy138Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_138::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy138Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_138 to value 0"]
impl crate::Resettable for DdrDenaliPhy138Spec {
    const RESET_VALUE: u32 = 0;
}
