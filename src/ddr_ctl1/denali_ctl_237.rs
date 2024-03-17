#[doc = "Register `DENALI_CTL_237` reader"]
pub type R = crate::R<DenaliCtl237Spec>;
#[doc = "Register `DENALI_CTL_237` writer"]
pub type W = crate::W<DenaliCtl237Spec>;
#[doc = "Field `RDLVL_GATE_SEQ_EN` reader - Specifies the pattern, format and MPR for gate training."]
pub type RdlvlGateSeqEnR = crate::FieldReader;
#[doc = "Field `RDLVL_GATE_SEQ_EN` writer - Specifies the pattern, format and MPR for gate training."]
pub type RdlvlGateSeqEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFI_PHY_RDLVL_MODE` reader - Specifies the PHY support for DFI data eye training. Set to 1 for supported."]
pub type DfiPhyRdlvlModeR = crate::BitReader;
#[doc = "Field `DFI_PHY_RDLVL_MODE` writer - Specifies the PHY support for DFI data eye training. Set to 1 for supported."]
pub type DfiPhyRdlvlModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFI_PHY_RDLVL_GATE_MODE` reader - Specifies the PHY support for DFI gate training. Set to 1 for supported."]
pub type DfiPhyRdlvlGateModeR = crate::BitReader;
#[doc = "Field `DFI_PHY_RDLVL_GATE_MODE` writer - Specifies the PHY support for DFI gate training. Set to 1 for supported."]
pub type DfiPhyRdlvlGateModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDLVL_PERIODIC` reader - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
pub type RdlvlPeriodicR = crate::BitReader;
#[doc = "Field `RDLVL_PERIODIC` writer - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
pub type RdlvlPeriodicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Specifies the pattern, format and MPR for gate training."]
    #[inline(always)]
    pub fn rdlvl_gate_seq_en(&self) -> RdlvlGateSeqEnR {
        RdlvlGateSeqEnR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Specifies the PHY support for DFI data eye training. Set to 1 for supported."]
    #[inline(always)]
    pub fn dfi_phy_rdlvl_mode(&self) -> DfiPhyRdlvlModeR {
        DfiPhyRdlvlModeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI gate training. Set to 1 for supported."]
    #[inline(always)]
    pub fn dfi_phy_rdlvl_gate_mode(&self) -> DfiPhyRdlvlGateModeR {
        DfiPhyRdlvlGateModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
    #[inline(always)]
    pub fn rdlvl_periodic(&self) -> RdlvlPeriodicR {
        RdlvlPeriodicR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Specifies the pattern, format and MPR for gate training."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_gate_seq_en(&mut self) -> RdlvlGateSeqEnW<DenaliCtl237Spec> {
        RdlvlGateSeqEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Specifies the PHY support for DFI data eye training. Set to 1 for supported."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phy_rdlvl_mode(&mut self) -> DfiPhyRdlvlModeW<DenaliCtl237Spec> {
        DfiPhyRdlvlModeW::new(self, 8)
    }
    #[doc = "Bit 16 - Specifies the PHY support for DFI gate training. Set to 1 for supported."]
    #[inline(always)]
    #[must_use]
    pub fn dfi_phy_rdlvl_gate_mode(&mut self) -> DfiPhyRdlvlGateModeW<DenaliCtl237Spec> {
        DfiPhyRdlvlGateModeW::new(self, 16)
    }
    #[doc = "Bit 24 - Enables the use of the dfi_lvl_periodic signal during data eye training. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn rdlvl_periodic(&mut self) -> RdlvlPeriodicW<DenaliCtl237Spec> {
        RdlvlPeriodicW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_237::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_237::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl237Spec;
impl crate::RegisterSpec for DenaliCtl237Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_237::R`](R) reader structure"]
impl crate::Readable for DenaliCtl237Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_237::W`](W) writer structure"]
impl crate::Writable for DenaliCtl237Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_237 to value 0"]
impl crate::Resettable for DenaliCtl237Spec {
    const RESET_VALUE: u32 = 0;
}
