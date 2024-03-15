#[doc = "Register `DENALI_CTL_201` reader"]
pub type R = crate::R<DenaliCtl201Spec>;
#[doc = "Register `DENALI_CTL_201` writer"]
pub type W = crate::W<DenaliCtl201Spec>;
#[doc = "Field `RD_PREAMBLE_TRAINING_EN` reader - Enable read preamble training during gate training. Set to 1 to enable."]
pub type RdPreambleTrainingEnR = crate::BitReader;
#[doc = "Field `RD_PREAMBLE_TRAINING_EN` writer - Enable read preamble training during gate training. Set to 1 to enable."]
pub type RdPreambleTrainingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DBI_EN` reader - Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
pub type WrDbiEnR = crate::BitReader;
#[doc = "Field `WR_DBI_EN` writer - Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
pub type WrDbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DBI_EN` reader - Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
pub type RdDbiEnR = crate::BitReader;
#[doc = "Field `RD_DBI_EN` writer - Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
pub type RdDbiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_ERROR` reader - Indicates that the DFI error flag has been asserted. READ-ONLY"]
pub type DfiErrorR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rd_preamble_training_en(&self) -> RdPreambleTrainingEnR {
        RdPreambleTrainingEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    pub fn wr_dbi_en(&self) -> WrDbiEnR {
        WrDbiEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    pub fn rd_dbi_en(&self) -> RdDbiEnR {
        RdDbiEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Indicates that the DFI error flag has been asserted. READ-ONLY"]
    #[inline(always)]
    pub fn dfi_error(&self) -> DfiErrorR {
        DfiErrorR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable read preamble training during gate training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_preamble_training_en(&mut self) -> RdPreambleTrainingEnW<DenaliCtl201Spec> {
        RdPreambleTrainingEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enables controller support of DRAM DBI feature for write data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn wr_dbi_en(&mut self) -> WrDbiEnW<DenaliCtl201Spec> {
        WrDbiEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enables controller support of DRAM DBI feature for read data with DDR4 devices. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rd_dbi_en(&mut self) -> RdDbiEnW<DenaliCtl201Spec> {
        RdDbiEnW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_201::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_201::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl201Spec;
impl crate::RegisterSpec for DenaliCtl201Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_201::R`](R) reader structure"]
impl crate::Readable for DenaliCtl201Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_201::W`](W) writer structure"]
impl crate::Writable for DenaliCtl201Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_201 to value 0"]
impl crate::Resettable for DenaliCtl201Spec {
    const RESET_VALUE: u32 = 0;
}
