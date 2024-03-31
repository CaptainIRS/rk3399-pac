#[doc = "Register `SIG_DETECT_STATUS` reader"]
pub type R = crate::R<SigDetectStatusSpec>;
#[doc = "Register `SIG_DETECT_STATUS` writer"]
pub type W = crate::W<SigDetectStatusSpec>;
#[doc = "Field `SDMMC_CARD_RISE_EDGE` reader - sdmmc card rise edge detect status"]
pub type SdmmcCardRiseEdgeR = crate::BitReader;
#[doc = "Field `SDMMC_CARD_RISE_EDGE` writer - sdmmc card rise edge detect status"]
pub type SdmmcCardRiseEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC_CARD_FALL_EDGE` reader - sdmmc card fall edge detect status"]
pub type SdmmcCardFallEdgeR = crate::BitReader;
#[doc = "Field `SDMMC_CARD_FALL_EDGE` writer - sdmmc card fall edge detect status"]
pub type SdmmcCardFallEdgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY0_OTG_LINESTATE_CHANGE` reader - cphy0_otg_linestate_change detect status"]
pub type Cphy0OtgLinestateChangeR = crate::BitReader;
#[doc = "Field `CPHY0_OTG_LINESTATE_CHANGE` writer - cphy0_otg_linestate_change detect status"]
pub type Cphy0OtgLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY0_OTG_BVALID_RISE` reader - cphy0_otg_bvalid_rise detect status"]
pub type Cphy0OtgBvalidRiseR = crate::BitReader;
#[doc = "Field `CPHY0_OTG_BVALID_RISE` writer - cphy0_otg_bvalid_rise detect status"]
pub type Cphy0OtgBvalidRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY0_OTG_ID_RISE` reader - cphy0_otg_id_rise detect status"]
pub type Cphy0OtgIdRiseR = crate::BitReader;
#[doc = "Field `CPHY0_OTG_ID_RISE` writer - cphy0_otg_id_rise detect status"]
pub type Cphy0OtgIdRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY0_OTG_ID_FALL` reader - cphy0_otg_id_fall detect status"]
pub type Cphy0OtgIdFallR = crate::BitReader;
#[doc = "Field `CPHY0_OTG_ID_FALL` writer - cphy0_otg_id_fall detect status"]
pub type Cphy0OtgIdFallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY0_HOST_LINESTATE_CHANGE` reader - cphy0_host_linestate_change detect status"]
pub type Cphy0HostLinestateChangeR = crate::BitReader;
#[doc = "Field `CPHY0_HOST_LINESTATE_CHANGE` writer - cphy0_host_linestate_change detect status"]
pub type Cphy0HostLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY1_OTG_LINESTATE_CHANGE` reader - cphy1_otg_linestate_change detect status"]
pub type Cphy1OtgLinestateChangeR = crate::BitReader;
#[doc = "Field `CPHY1_OTG_LINESTATE_CHANGE` writer - cphy1_otg_linestate_change detect status"]
pub type Cphy1OtgLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY1_OTG_BVALID_RISE` reader - cphy1_otg_bvalid_rise detect status"]
pub type Cphy1OtgBvalidRiseR = crate::BitReader;
#[doc = "Field `CPHY1_OTG_BVALID_RISE` writer - cphy1_otg_bvalid_rise detect status"]
pub type Cphy1OtgBvalidRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY1_OTG_ID_RISE` reader - cphy1_otg_id_rise detect status"]
pub type Cphy1OtgIdRiseR = crate::BitReader;
#[doc = "Field `CPHY1_OTG_ID_RISE` writer - cphy1_otg_id_rise detect status"]
pub type Cphy1OtgIdRiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY1_OTG_ID_FALL` reader - cphy1_otg_id_fall detect status"]
pub type Cphy1OtgIdFallR = crate::BitReader;
#[doc = "Field `CPHY1_OTG_ID_FALL` writer - cphy1_otg_id_fall detect status"]
pub type Cphy1OtgIdFallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHY1_HOST_LINESTATE_CHANGE` reader - cphy1_host_linestate_change detect status"]
pub type Cphy1HostLinestateChangeR = crate::BitReader;
#[doc = "Field `CPHY1_HOST_LINESTATE_CHANGE` writer - cphy1_host_linestate_change detect status"]
pub type Cphy1HostLinestateChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY0_RXDET_CHANGE` reader - uphy0_rxdet_change detect status"]
pub type Uphy0RxdetChangeR = crate::BitReader;
#[doc = "Field `UPHY0_RXDET_CHANGE` writer - uphy0_rxdet_change detect status"]
pub type Uphy0RxdetChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY1_RXDET_CHANGE` reader - uphy1_rxdet_change detect status"]
pub type Uphy1RxdetChangeR = crate::BitReader;
#[doc = "Field `UPHY1_RXDET_CHANGE` writer - uphy1_rxdet_change detect status"]
pub type Uphy1RxdetChangeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - sdmmc card rise edge detect status"]
    #[inline(always)]
    pub fn sdmmc_card_rise_edge(&self) -> SdmmcCardRiseEdgeR {
        SdmmcCardRiseEdgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sdmmc card fall edge detect status"]
    #[inline(always)]
    pub fn sdmmc_card_fall_edge(&self) -> SdmmcCardFallEdgeR {
        SdmmcCardFallEdgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - cphy0_otg_linestate_change detect status"]
    #[inline(always)]
    pub fn cphy0_otg_linestate_change(&self) -> Cphy0OtgLinestateChangeR {
        Cphy0OtgLinestateChangeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - cphy0_otg_bvalid_rise detect status"]
    #[inline(always)]
    pub fn cphy0_otg_bvalid_rise(&self) -> Cphy0OtgBvalidRiseR {
        Cphy0OtgBvalidRiseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - cphy0_otg_id_rise detect status"]
    #[inline(always)]
    pub fn cphy0_otg_id_rise(&self) -> Cphy0OtgIdRiseR {
        Cphy0OtgIdRiseR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - cphy0_otg_id_fall detect status"]
    #[inline(always)]
    pub fn cphy0_otg_id_fall(&self) -> Cphy0OtgIdFallR {
        Cphy0OtgIdFallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - cphy0_host_linestate_change detect status"]
    #[inline(always)]
    pub fn cphy0_host_linestate_change(&self) -> Cphy0HostLinestateChangeR {
        Cphy0HostLinestateChangeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - cphy1_otg_linestate_change detect status"]
    #[inline(always)]
    pub fn cphy1_otg_linestate_change(&self) -> Cphy1OtgLinestateChangeR {
        Cphy1OtgLinestateChangeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - cphy1_otg_bvalid_rise detect status"]
    #[inline(always)]
    pub fn cphy1_otg_bvalid_rise(&self) -> Cphy1OtgBvalidRiseR {
        Cphy1OtgBvalidRiseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cphy1_otg_id_rise detect status"]
    #[inline(always)]
    pub fn cphy1_otg_id_rise(&self) -> Cphy1OtgIdRiseR {
        Cphy1OtgIdRiseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cphy1_otg_id_fall detect status"]
    #[inline(always)]
    pub fn cphy1_otg_id_fall(&self) -> Cphy1OtgIdFallR {
        Cphy1OtgIdFallR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - cphy1_host_linestate_change detect status"]
    #[inline(always)]
    pub fn cphy1_host_linestate_change(&self) -> Cphy1HostLinestateChangeR {
        Cphy1HostLinestateChangeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - uphy0_rxdet_change detect status"]
    #[inline(always)]
    pub fn uphy0_rxdet_change(&self) -> Uphy0RxdetChangeR {
        Uphy0RxdetChangeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - uphy1_rxdet_change detect status"]
    #[inline(always)]
    pub fn uphy1_rxdet_change(&self) -> Uphy1RxdetChangeR {
        Uphy1RxdetChangeR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - sdmmc card rise edge detect status"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_card_rise_edge(&mut self) -> SdmmcCardRiseEdgeW<SigDetectStatusSpec> {
        SdmmcCardRiseEdgeW::new(self, 0)
    }
    #[doc = "Bit 1 - sdmmc card fall edge detect status"]
    #[inline(always)]
    #[must_use]
    pub fn sdmmc_card_fall_edge(&mut self) -> SdmmcCardFallEdgeW<SigDetectStatusSpec> {
        SdmmcCardFallEdgeW::new(self, 1)
    }
    #[doc = "Bit 2 - cphy0_otg_linestate_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_linestate_change(&mut self) -> Cphy0OtgLinestateChangeW<SigDetectStatusSpec> {
        Cphy0OtgLinestateChangeW::new(self, 2)
    }
    #[doc = "Bit 3 - cphy0_otg_bvalid_rise detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_bvalid_rise(&mut self) -> Cphy0OtgBvalidRiseW<SigDetectStatusSpec> {
        Cphy0OtgBvalidRiseW::new(self, 3)
    }
    #[doc = "Bit 4 - cphy0_otg_id_rise detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_id_rise(&mut self) -> Cphy0OtgIdRiseW<SigDetectStatusSpec> {
        Cphy0OtgIdRiseW::new(self, 4)
    }
    #[doc = "Bit 5 - cphy0_otg_id_fall detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_otg_id_fall(&mut self) -> Cphy0OtgIdFallW<SigDetectStatusSpec> {
        Cphy0OtgIdFallW::new(self, 5)
    }
    #[doc = "Bit 6 - cphy0_host_linestate_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy0_host_linestate_change(
        &mut self,
    ) -> Cphy0HostLinestateChangeW<SigDetectStatusSpec> {
        Cphy0HostLinestateChangeW::new(self, 6)
    }
    #[doc = "Bit 7 - cphy1_otg_linestate_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_linestate_change(&mut self) -> Cphy1OtgLinestateChangeW<SigDetectStatusSpec> {
        Cphy1OtgLinestateChangeW::new(self, 7)
    }
    #[doc = "Bit 8 - cphy1_otg_bvalid_rise detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_bvalid_rise(&mut self) -> Cphy1OtgBvalidRiseW<SigDetectStatusSpec> {
        Cphy1OtgBvalidRiseW::new(self, 8)
    }
    #[doc = "Bit 9 - cphy1_otg_id_rise detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_id_rise(&mut self) -> Cphy1OtgIdRiseW<SigDetectStatusSpec> {
        Cphy1OtgIdRiseW::new(self, 9)
    }
    #[doc = "Bit 10 - cphy1_otg_id_fall detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_otg_id_fall(&mut self) -> Cphy1OtgIdFallW<SigDetectStatusSpec> {
        Cphy1OtgIdFallW::new(self, 10)
    }
    #[doc = "Bit 11 - cphy1_host_linestate_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn cphy1_host_linestate_change(
        &mut self,
    ) -> Cphy1HostLinestateChangeW<SigDetectStatusSpec> {
        Cphy1HostLinestateChangeW::new(self, 11)
    }
    #[doc = "Bit 12 - uphy0_rxdet_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn uphy0_rxdet_change(&mut self) -> Uphy0RxdetChangeW<SigDetectStatusSpec> {
        Uphy0RxdetChangeW::new(self, 12)
    }
    #[doc = "Bit 13 - uphy1_rxdet_change detect status"]
    #[inline(always)]
    #[must_use]
    pub fn uphy1_rxdet_change(&mut self) -> Uphy1RxdetChangeW<SigDetectStatusSpec> {
        Uphy1RxdetChangeW::new(self, 13)
    }
}
#[doc = "Signal detect status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sig_detect_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sig_detect_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SigDetectStatusSpec;
impl crate::RegisterSpec for SigDetectStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sig_detect_status::R`](R) reader structure"]
impl crate::Readable for SigDetectStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`sig_detect_status::W`](W) writer structure"]
impl crate::Writable for SigDetectStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIG_DETECT_STATUS to value 0"]
impl crate::Resettable for SigDetectStatusSpec {
    const RESET_VALUE: u32 = 0;
}
