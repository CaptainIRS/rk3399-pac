#[doc = "Register `DENALI_PHY_15` reader"]
pub type R = crate::R<DenaliPhy15Spec>;
#[doc = "Register `DENALI_PHY_15` writer"]
pub type W = crate::W<DenaliPhy15Spec>;
#[doc = "Field `PHY_DFI40_POLARITY_0` reader - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity0R = crate::BitReader;
#[doc = "Field `PHY_DFI40_POLARITY_0` writer - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_PST_AMBLE_0` reader - Controls the read postamble extension for LPDDR4 for slice 0."]
pub type PhyLp4PstAmble0R = crate::FieldReader;
#[doc = "Field `PHY_LP4_PST_AMBLE_0` writer - Controls the read postamble extension for LPDDR4 for slice 0."]
pub type PhyLp4PstAmble0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    pub fn phy_dfi40_polarity_0(&self) -> PhyDfi40Polarity0R {
        PhyDfi40Polarity0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 0."]
    #[inline(always)]
    pub fn phy_lp4_pst_amble_0(&self) -> PhyLp4PstAmble0R {
        PhyLp4PstAmble0R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi40_polarity_0(&mut self) -> PhyDfi40Polarity0W<DenaliPhy15Spec> {
        PhyDfi40Polarity0W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_pst_amble_0(&mut self) -> PhyLp4PstAmble0W<DenaliPhy15Spec> {
        PhyLp4PstAmble0W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy15Spec;
impl crate::RegisterSpec for DenaliPhy15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_15::R`](R) reader structure"]
impl crate::Readable for DenaliPhy15Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_15::W`](W) writer structure"]
impl crate::Writable for DenaliPhy15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_15 to value 0"]
impl crate::Resettable for DenaliPhy15Spec {
    const RESET_VALUE: u32 = 0;
}
