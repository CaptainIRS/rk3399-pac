#[doc = "Register `DDR_PI_REG_90` reader"]
pub type R = crate::R<DdrPiReg90Spec>;
#[doc = "Register `DDR_PI_REG_90` writer"]
pub type W = crate::W<DdrPiReg90Spec>;
#[doc = "Field `PI_RDLVL_ADJ_F2` reader - Indicates the adjustment value for PHY read timing. The suffix \"_f2\"\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF2R = crate::FieldReader;
#[doc = "Field `PI_RDLVL_ADJ_F2` writer - Indicates the adjustment value for PHY read timing. The suffix \"_f2\"\n\nof the parameter name is omitted when in non-DFS mode."]
pub type PiRdlvlAdjF2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_TDFI_RDDATA_EN` reader - Holds the calculated DFI tRDDATA_EN timing parameter (in DFI\n\nPHY clocks), the maximum cycles between a read command and a\n\ndfi_rddata_en assertion."]
pub type PiTdfiRddataEnR = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F0` reader - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF0R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F0` writer - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PI_WRLAT_ADJ_F1` reader - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF1R = crate::FieldReader;
#[doc = "Field `PI_WRLAT_ADJ_F1` writer - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
pub type PiWrlatAdjF1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates the adjustment value for PHY read timing. The suffix \"_f2\"\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_rdlvl_adj_f2(&self) -> PiRdlvlAdjF2R {
        PiRdlvlAdjF2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Holds the calculated DFI tRDDATA_EN timing parameter (in DFI\n\nPHY clocks), the maximum cycles between a read command and a\n\ndfi_rddata_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_rddata_en(&self) -> PiTdfiRddataEnR {
        PiTdfiRddataEnR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f0(&self) -> PiWrlatAdjF0R {
        PiWrlatAdjF0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    pub fn pi_wrlat_adj_f1(&self) -> PiWrlatAdjF1R {
        PiWrlatAdjF1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates the adjustment value for PHY read timing. The suffix \"_f2\"\n\nof the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_rdlvl_adj_f2(&mut self) -> PiRdlvlAdjF2W<DdrPiReg90Spec> {
        PiRdlvlAdjF2W::new(self, 0)
    }
    #[doc = "Bits 16:23 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f0\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f0(&mut self) -> PiWrlatAdjF0W<DdrPiReg90Spec> {
        PiWrlatAdjF0W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Indicates the adjustment value for PHY write timing. The suffix\n\n\"_f1\" of the parameter name is omitted when in non-DFS mode."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlat_adj_f1(&mut self) -> PiWrlatAdjF1W<DdrPiReg90Spec> {
        PiWrlatAdjF1W::new(self, 24)
    }
}
#[doc = "DDR PHY Independent Register 90\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_90::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_90::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg90Spec;
impl crate::RegisterSpec for DdrPiReg90Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_90::R`](R) reader structure"]
impl crate::Readable for DdrPiReg90Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_90::W`](W) writer structure"]
impl crate::Writable for DdrPiReg90Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_90 to value 0"]
impl crate::Resettable for DdrPiReg90Spec {
    const RESET_VALUE: u32 = 0;
}
