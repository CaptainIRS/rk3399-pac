#[doc = "Register `SOC_CON_9_PCIE` reader"]
pub type R = crate::R<SocCon9PcieSpec>;
#[doc = "Register `SOC_CON_9_PCIE` writer"]
pub type W = crate::W<SocCon9PcieSpec>;
#[doc = "Field `PCIE_RC_MODE_IDLE_IRQ_CLR` reader - irq clear bit for pcie_rc_mode_idle_irq"]
pub type PcieRcModeIdleIrqClrR = crate::BitReader;
#[doc = "Field `PCIE_RC_MODE_IDLE_IRQ_CLR` writer - irq clear bit for pcie_rc_mode_idle_irq"]
pub type PcieRcModeIdleIrqClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - irq clear bit for pcie_rc_mode_idle_irq"]
    #[inline(always)]
    pub fn pcie_rc_mode_idle_irq_clr(&self) -> PcieRcModeIdleIrqClrR {
        PcieRcModeIdleIrqClrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - irq clear bit for pcie_rc_mode_idle_irq"]
    #[inline(always)]
    #[must_use]
    pub fn pcie_rc_mode_idle_irq_clr(&mut self) -> PcieRcModeIdleIrqClrW<SocCon9PcieSpec> {
        PcieRcModeIdleIrqClrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon9PcieSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 9 for PCIE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con_9_pcie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con_9_pcie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon9PcieSpec;
impl crate::RegisterSpec for SocCon9PcieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con_9_pcie::R`](R) reader structure"]
impl crate::Readable for SocCon9PcieSpec {}
#[doc = "`write(|w| ..)` method takes [`soc_con_9_pcie::W`](W) writer structure"]
impl crate::Writable for SocCon9PcieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON_9_PCIE to value 0"]
impl crate::Resettable for SocCon9PcieSpec {
    const RESET_VALUE: u32 = 0;
}
