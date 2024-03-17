#[doc = "Register `DENALI_PHY_271` reader"]
pub type R = crate::R<DenaliPhy271Spec>;
#[doc = "Register `DENALI_PHY_271` writer"]
pub type W = crate::W<DenaliPhy271Spec>;
#[doc = "Field `PHY_DFI40_POLARITY_2` reader - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity2R = crate::BitReader;
#[doc = "Field `PHY_DFI40_POLARITY_2` writer - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
pub type PhyDfi40Polarity2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_LP4_PST_AMBLE_2` reader - Controls the read postamble extension for LPDDR4 for slice 2."]
pub type PhyLp4PstAmble2R = crate::FieldReader;
#[doc = "Field `PHY_LP4_PST_AMBLE_2` writer - Controls the read postamble extension for LPDDR4 for slice 2."]
pub type PhyLp4PstAmble2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    pub fn phy_dfi40_polarity_2(&self) -> PhyDfi40Polarity2R {
        PhyDfi40Polarity2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 2."]
    #[inline(always)]
    pub fn phy_lp4_pst_amble_2(&self) -> PhyLp4PstAmble2R {
        PhyLp4PstAmble2R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates the dfi_wrdata_cs_n and dfi_rddata_cs_n is low active or high active."]
    #[inline(always)]
    #[must_use]
    pub fn phy_dfi40_polarity_2(&mut self) -> PhyDfi40Polarity2W<DenaliPhy271Spec> {
        PhyDfi40Polarity2W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Controls the read postamble extension for LPDDR4 for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_pst_amble_2(&mut self) -> PhyLp4PstAmble2W<DenaliPhy271Spec> {
        PhyLp4PstAmble2W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_271::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_271::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy271Spec;
impl crate::RegisterSpec for DenaliPhy271Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_271::R`](R) reader structure"]
impl crate::Readable for DenaliPhy271Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_271::W`](W) writer structure"]
impl crate::Writable for DenaliPhy271Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_271 to value 0"]
impl crate::Resettable for DenaliPhy271Spec {
    const RESET_VALUE: u32 = 0;
}
