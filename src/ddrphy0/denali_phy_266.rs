#[doc = "Register `DENALI_PHY_266` reader"]
pub type R = crate::R<DenaliPhy266Spec>;
#[doc = "Register `DENALI_PHY_266` writer"]
pub type W = crate::W<DenaliPhy266Spec>;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_2` reader - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
pub type PhyLp4BootRddqsLatencyAdjust2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_BOOT_RDDQS_LATENCY_ADJUST_2` writer - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
pub type PhyLp4BootRddqsLatencyAdjust2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_LPBK_CONTROL_2` reader - Loopback control bits for slice 2."]
pub type PhyLpbkControl2R = crate::FieldReader;
#[doc = "Field `PHY_LPBK_CONTROL_2` writer - Loopback control bits for slice 2."]
pub type PhyLpbkControl2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_2` reader - data slice power reduction disable for slice 2."]
pub type PhySlicePwrRdcDisable2R = crate::BitReader;
#[doc = "Field `PHY_SLICE_PWR_RDC_DISABLE_2` writer - data slice power reduction disable for slice 2."]
pub type PhySlicePwrRdcDisable2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_boot_rddqs_latency_adjust_2(&self) -> PhyLp4BootRddqsLatencyAdjust2R {
        PhyLp4BootRddqsLatencyAdjust2R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 2."]
    #[inline(always)]
    pub fn phy_lpbk_control_2(&self) -> PhyLpbkControl2R {
        PhyLpbkControl2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 2."]
    #[inline(always)]
    pub fn phy_slice_pwr_rdc_disable_2(&self) -> PhySlicePwrRdcDisable2R {
        PhySlicePwrRdcDisable2R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - For LPDDR4 boot frequency, the number of cycles to delay the incoming dfi_rddata_en for read DQS gate generation for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_rddqs_latency_adjust_2(
        &mut self,
    ) -> PhyLp4BootRddqsLatencyAdjust2W<DenaliPhy266Spec> {
        PhyLp4BootRddqsLatencyAdjust2W::new(self, 0)
    }
    #[doc = "Bits 8:14 - Loopback control bits for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lpbk_control_2(&mut self) -> PhyLpbkControl2W<DenaliPhy266Spec> {
        PhyLpbkControl2W::new(self, 8)
    }
    #[doc = "Bit 16 - data slice power reduction disable for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_slice_pwr_rdc_disable_2(&mut self) -> PhySlicePwrRdcDisable2W<DenaliPhy266Spec> {
        PhySlicePwrRdcDisable2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_266::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_266::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy266Spec;
impl crate::RegisterSpec for DenaliPhy266Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_266::R`](R) reader structure"]
impl crate::Readable for DenaliPhy266Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_266::W`](W) writer structure"]
impl crate::Writable for DenaliPhy266Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_266 to value 0"]
impl crate::Resettable for DenaliPhy266Spec {
    const RESET_VALUE: u32 = 0;
}
