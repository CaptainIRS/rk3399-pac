#[doc = "Register `GRF_USB3OTG0_CON1` reader"]
pub type R = crate::R<GrfUsb3otg0Con1Spec>;
#[doc = "Register `GRF_USB3OTG0_CON1` writer"]
pub type W = crate::W<GrfUsb3otg0Con1Spec>;
#[doc = "host_u3_port_disable USB 3.0 SS Port Disable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostU3PortDisable {
    #[doc = "0: Port Disabled"]
    B0 = 0,
    #[doc = "1: Port Disabled"]
    B1 = 1,
}
impl From<HostU3PortDisable> for bool {
    #[inline(always)]
    fn from(variant: HostU3PortDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_U3_PORT_DISABLE` reader - host_u3_port_disable USB 3.0 SS Port Disable control."]
pub type HostU3PortDisableR = crate::BitReader<HostU3PortDisable>;
impl HostU3PortDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostU3PortDisable {
        match self.bits {
            false => HostU3PortDisable::B0,
            true => HostU3PortDisable::B1,
        }
    }
    #[doc = "Port Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HostU3PortDisable::B0
    }
    #[doc = "Port Disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HostU3PortDisable::B1
    }
}
#[doc = "Field `HOST_U3_PORT_DISABLE` writer - host_u3_port_disable USB 3.0 SS Port Disable control."]
pub type HostU3PortDisableW<'a, REG> = crate::BitWriter<'a, REG, HostU3PortDisable>;
impl<'a, REG> HostU3PortDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HostU3PortDisable::B0)
    }
    #[doc = "Port Disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HostU3PortDisable::B1)
    }
}
#[doc = "Field `PME_EN` reader - pme_en Enable signal for the pme_generation. Enable the core to assert pme_generation."]
pub type PmeEnR = crate::BitReader;
#[doc = "Field `PME_EN` writer - pme_en Enable signal for the pme_generation. Enable the core to assert pme_generation."]
pub type PmeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_LEGACY_SMI_PCI_CMD` reader - host_legacy_smi_pci_cmd Use this register to support SMI on PCI Command defined in xHCI spec. SW must set this register, then clear this register to indicate PCI command register written."]
pub type HostLegacySmiPciCmdR = crate::BitReader;
#[doc = "Field `HOST_LEGACY_SMI_PCI_CMD` writer - host_legacy_smi_pci_cmd Use this register to support SMI on PCI Command defined in xHCI spec. SW must set this register, then clear this register to indicate PCI command register written."]
pub type HostLegacySmiPciCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_LEGACY_SMI_BAR` reader - host_legacy_smi_bar Use this register to support SMI on BAR defined in xHCI spec. SW must set this register, then clear this register to indicate Base Address Register written"]
pub type HostLegacySmiBarR = crate::BitReader;
#[doc = "Field `HOST_LEGACY_SMI_BAR` writer - host_legacy_smi_bar Use this register to support SMI on BAR defined in xHCI spec. SW must set this register, then clear this register to indicate Base Address Register written"]
pub type HostLegacySmiBarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_U2_PORT` reader - host_u2_port xHCI host USB2 Port number, default as 1."]
pub type HostU2PortR = crate::FieldReader;
#[doc = "Field `HOST_U2_PORT` writer - host_u2_port xHCI host USB2 Port number, default as 1."]
pub type HostU2PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOST_U3_PORT` reader - host_u3_port xHCI usb3 port number, default as 1."]
pub type HostU3PortR = crate::FieldReader;
#[doc = "Field `HOST_U3_PORT` writer - host_u3_port xHCI usb3 port number, default as 1."]
pub type HostU3PortW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - host_u3_port_disable USB 3.0 SS Port Disable control."]
    #[inline(always)]
    pub fn host_u3_port_disable(&self) -> HostU3PortDisableR {
        HostU3PortDisableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pme_en Enable signal for the pme_generation. Enable the core to assert pme_generation."]
    #[inline(always)]
    pub fn pme_en(&self) -> PmeEnR {
        PmeEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - host_legacy_smi_pci_cmd Use this register to support SMI on PCI Command defined in xHCI spec. SW must set this register, then clear this register to indicate PCI command register written."]
    #[inline(always)]
    pub fn host_legacy_smi_pci_cmd(&self) -> HostLegacySmiPciCmdR {
        HostLegacySmiPciCmdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - host_legacy_smi_bar Use this register to support SMI on BAR defined in xHCI spec. SW must set this register, then clear this register to indicate Base Address Register written"]
    #[inline(always)]
    pub fn host_legacy_smi_bar(&self) -> HostLegacySmiBarR {
        HostLegacySmiBarR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - host_u2_port xHCI host USB2 Port number, default as 1."]
    #[inline(always)]
    pub fn host_u2_port(&self) -> HostU2PortR {
        HostU2PortR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - host_u3_port xHCI usb3 port number, default as 1."]
    #[inline(always)]
    pub fn host_u3_port(&self) -> HostU3PortR {
        HostU3PortR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - host_u3_port_disable USB 3.0 SS Port Disable control."]
    #[inline(always)]
    #[must_use]
    pub fn host_u3_port_disable(&mut self) -> HostU3PortDisableW<GrfUsb3otg0Con1Spec> {
        HostU3PortDisableW::new(self, 0)
    }
    #[doc = "Bit 1 - pme_en Enable signal for the pme_generation. Enable the core to assert pme_generation."]
    #[inline(always)]
    #[must_use]
    pub fn pme_en(&mut self) -> PmeEnW<GrfUsb3otg0Con1Spec> {
        PmeEnW::new(self, 1)
    }
    #[doc = "Bit 4 - host_legacy_smi_pci_cmd Use this register to support SMI on PCI Command defined in xHCI spec. SW must set this register, then clear this register to indicate PCI command register written."]
    #[inline(always)]
    #[must_use]
    pub fn host_legacy_smi_pci_cmd(&mut self) -> HostLegacySmiPciCmdW<GrfUsb3otg0Con1Spec> {
        HostLegacySmiPciCmdW::new(self, 4)
    }
    #[doc = "Bit 5 - host_legacy_smi_bar Use this register to support SMI on BAR defined in xHCI spec. SW must set this register, then clear this register to indicate Base Address Register written"]
    #[inline(always)]
    #[must_use]
    pub fn host_legacy_smi_bar(&mut self) -> HostLegacySmiBarW<GrfUsb3otg0Con1Spec> {
        HostLegacySmiBarW::new(self, 5)
    }
    #[doc = "Bits 8:11 - host_u2_port xHCI host USB2 Port number, default as 1."]
    #[inline(always)]
    #[must_use]
    pub fn host_u2_port(&mut self) -> HostU2PortW<GrfUsb3otg0Con1Spec> {
        HostU2PortW::new(self, 8)
    }
    #[doc = "Bits 12:15 - host_u3_port xHCI usb3 port number, default as 1."]
    #[inline(always)]
    #[must_use]
    pub fn host_u3_port(&mut self) -> HostU3PortW<GrfUsb3otg0Con1Spec> {
        HostU3PortW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb3otg0Con1Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB3 OTG0 GRF Register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3otg0Con1Spec;
impl crate::RegisterSpec for GrfUsb3otg0Con1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3otg0_con1::R`](R) reader structure"]
impl crate::Readable for GrfUsb3otg0Con1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3otg0_con1::W`](W) writer structure"]
impl crate::Writable for GrfUsb3otg0Con1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3OTG0_CON1 to value 0x1100"]
impl crate::Resettable for GrfUsb3otg0Con1Spec {
    const RESET_VALUE: u32 = 0x1100;
}
