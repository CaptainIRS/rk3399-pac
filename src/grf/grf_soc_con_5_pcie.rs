#[doc = "Register `GRF_SOC_CON_5_PCIE` reader"]
pub type R = crate::R<GrfSocCon5PcieSpec>;
#[doc = "Register `GRF_SOC_CON_5_PCIE` writer"]
pub type W = crate::W<GrfSocCon5PcieSpec>;
#[doc = "Field `PCIE_TX_ELEC_IDLE_SEL` reader - pcie_tx_elec_idle_sel port control"]
pub type PcieTxElecIdleSelR = crate::BitReader;
#[doc = "Field `PCIE_TX_ELEC_IDLE_SEL` writer - pcie_tx_elec_idle_sel port control"]
pub type PcieTxElecIdleSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE_TX_ELEC_IDLE_SET` reader - pcie_tx_elec_idle_set port control"]
pub type PcieTxElecIdleSetR = crate::BitReader;
#[doc = "Field `PCIE_TX_ELEC_IDLE_SET` writer - pcie_tx_elec_idle_set port control"]
pub type PcieTxElecIdleSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE_RX_ELEC_IDLE_IRQ_EN` reader - pcie_rx_elec_idle_irq_en port control"]
pub type PcieRxElecIdleIrqEnR = crate::BitReader;
#[doc = "Field `PCIE_RX_ELEC_IDLE_IRQ_EN` writer - pcie_rx_elec_idle_irq_en port control"]
pub type PcieRxElecIdleIrqEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCIE_TX_ELEC_IDLE_OFF` reader - pcie_tx_elec_idle_off\\[3:0\\]
port control"]
pub type PcieTxElecIdleOffR = crate::FieldReader;
#[doc = "Field `PCIE_TX_ELEC_IDLE_OFF` writer - pcie_tx_elec_idle_off\\[3:0\\]
port control"]
pub type PcieTxElecIdleOffW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pcie_tx_elec_idle_sel port control"]
    #[inline(always)]
    pub fn pcie_tx_elec_idle_sel(&self) -> PcieTxElecIdleSelR {
        PcieTxElecIdleSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pcie_tx_elec_idle_set port control"]
    #[inline(always)]
    pub fn pcie_tx_elec_idle_set(&self) -> PcieTxElecIdleSetR {
        PcieTxElecIdleSetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pcie_rx_elec_idle_irq_en port control"]
    #[inline(always)]
    pub fn pcie_rx_elec_idle_irq_en(&self) -> PcieRxElecIdleIrqEnR {
        PcieRxElecIdleIrqEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - pcie_tx_elec_idle_off\\[3:0\\]
port control"]
    #[inline(always)]
    pub fn pcie_tx_elec_idle_off(&self) -> PcieTxElecIdleOffR {
        PcieTxElecIdleOffR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pcie_tx_elec_idle_sel port control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_tx_elec_idle_sel(&mut self) -> PcieTxElecIdleSelW<GrfSocCon5PcieSpec> {
        PcieTxElecIdleSelW::new(self, 0)
    }
    #[doc = "Bit 1 - pcie_tx_elec_idle_set port control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_tx_elec_idle_set(&mut self) -> PcieTxElecIdleSetW<GrfSocCon5PcieSpec> {
        PcieTxElecIdleSetW::new(self, 1)
    }
    #[doc = "Bit 2 - pcie_rx_elec_idle_irq_en port control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_rx_elec_idle_irq_en(&mut self) -> PcieRxElecIdleIrqEnW<GrfSocCon5PcieSpec> {
        PcieRxElecIdleIrqEnW::new(self, 2)
    }
    #[doc = "Bits 3:6 - pcie_tx_elec_idle_off\\[3:0\\]
port control"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_tx_elec_idle_off(&mut self) -> PcieTxElecIdleOffW<GrfSocCon5PcieSpec> {
        PcieTxElecIdleOffW::new(self, 3)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon5PcieSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con_5_pcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con_5_pcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon5PcieSpec;
impl crate::RegisterSpec for GrfSocCon5PcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con_5_pcie::R`](R) reader structure"]
impl crate::Readable for GrfSocCon5PcieSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con_5_pcie::W`](W) writer structure"]
impl crate::Writable for GrfSocCon5PcieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON_5_PCIE to value 0x02"]
impl crate::Resettable for GrfSocCon5PcieSpec {
    const RESET_VALUE: u32 = 0x02;
}
