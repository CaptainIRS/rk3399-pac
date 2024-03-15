#[doc = "Register `GRF_USB3OTG0_CON0` reader"]
pub type R = crate::R<GrfUsb3otg0Con0Spec>;
#[doc = "Register `GRF_USB3OTG0_CON0` writer"]
pub type W = crate::W<GrfUsb3otg0Con0Spec>;
#[doc = "bus_filter_bypass It is expected that this signal is set or reset at power-on reset and is not changed during the normal operation of the core. The function of each bit is: bus_filter_bypass\\[3\\]: Bypass the filter for utmiotg_iddig bus_filter_bypass\\[2\\]: Bypass the filters for utmisrp_bvalid and utmisrp_sessend bus_filter_bypass\\[1\\]: Bypass the filter for pipe3_PowerPresent all U3 ports bus_filter_bypass\\[0\\]: Bypass the filter for utmiotg_vbusvalid all U2 ports In non-OTG Host-only mode, internal bus filters are not needed. Values:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusFilterBypass {
    #[doc = "0: Bus filter(s) disabled (bypassed)"]
    B0 = 0,
    #[doc = "1: Bus filter(s) disabled (bypassed)"]
    B1 = 1,
}
impl From<BusFilterBypass> for u8 {
    #[inline(always)]
    fn from(variant: BusFilterBypass) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusFilterBypass {
    type Ux = u8;
}
#[doc = "Field `BUS_FILTER_BYPASS` reader - bus_filter_bypass It is expected that this signal is set or reset at power-on reset and is not changed during the normal operation of the core. The function of each bit is: bus_filter_bypass\\[3\\]: Bypass the filter for utmiotg_iddig bus_filter_bypass\\[2\\]: Bypass the filters for utmisrp_bvalid and utmisrp_sessend bus_filter_bypass\\[1\\]: Bypass the filter for pipe3_PowerPresent all U3 ports bus_filter_bypass\\[0\\]: Bypass the filter for utmiotg_vbusvalid all U2 ports In non-OTG Host-only mode, internal bus filters are not needed. Values:"]
pub type BusFilterBypassR = crate::FieldReader<BusFilterBypass>;
impl BusFilterBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusFilterBypass> {
        match self.bits {
            0 => Some(BusFilterBypass::B0),
            1 => Some(BusFilterBypass::B1),
            _ => None,
        }
    }
    #[doc = "Bus filter(s) disabled (bypassed)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BusFilterBypass::B0
    }
    #[doc = "Bus filter(s) disabled (bypassed)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BusFilterBypass::B1
    }
}
#[doc = "Field `BUS_FILTER_BYPASS` writer - bus_filter_bypass It is expected that this signal is set or reset at power-on reset and is not changed during the normal operation of the core. The function of each bit is: bus_filter_bypass\\[3\\]: Bypass the filter for utmiotg_iddig bus_filter_bypass\\[2\\]: Bypass the filters for utmisrp_bvalid and utmisrp_sessend bus_filter_bypass\\[1\\]: Bypass the filter for pipe3_PowerPresent all U3 ports bus_filter_bypass\\[0\\]: Bypass the filter for utmiotg_vbusvalid all U2 ports In non-OTG Host-only mode, internal bus filters are not needed. Values:"]
pub type BusFilterBypassW<'a, REG> = crate::FieldWriter<'a, REG, 4, BusFilterBypass>;
impl<'a, REG> BusFilterBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus filter(s) disabled (bypassed)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(BusFilterBypass::B0)
    }
    #[doc = "Bus filter(s) disabled (bypassed)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BusFilterBypass::B1)
    }
}
#[doc = "hub_port_overcurrent This is the per port Overcurrent indication of the root-hub ports:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HubPortOvercurrent {
    #[doc = "0: Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    B0 = 0,
    #[doc = "1: Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    B1 = 1,
}
impl From<HubPortOvercurrent> for u8 {
    #[inline(always)]
    fn from(variant: HubPortOvercurrent) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HubPortOvercurrent {
    type Ux = u8;
}
#[doc = "Field `HUB_PORT_OVERCURRENT` reader - hub_port_overcurrent This is the per port Overcurrent indication of the root-hub ports:"]
pub type HubPortOvercurrentR = crate::FieldReader<HubPortOvercurrent>;
impl HubPortOvercurrentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HubPortOvercurrent> {
        match self.bits {
            0 => Some(HubPortOvercurrent::B0),
            1 => Some(HubPortOvercurrent::B1),
            _ => None,
        }
    }
    #[doc = "Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HubPortOvercurrent::B0
    }
    #[doc = "Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HubPortOvercurrent::B1
    }
}
#[doc = "Field `HUB_PORT_OVERCURRENT` writer - hub_port_overcurrent This is the per port Overcurrent indication of the root-hub ports:"]
pub type HubPortOvercurrentW<'a, REG> = crate::FieldWriter<'a, REG, 2, HubPortOvercurrent>;
impl<'a, REG> HubPortOvercurrentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HubPortOvercurrent::B0)
    }
    #[doc = "Overcurrent Bit0 is for USB 2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HubPortOvercurrent::B1)
    }
}
#[doc = "hub_port_perm_attach Indicates if the device attached to a downstream port is permanently attached or not.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HubPortPermAttach {
    #[doc = "0: Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    B0 = 0,
    #[doc = "1: Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    B1 = 1,
}
impl From<HubPortPermAttach> for u8 {
    #[inline(always)]
    fn from(variant: HubPortPermAttach) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HubPortPermAttach {
    type Ux = u8;
}
#[doc = "Field `HUB_PORT_PERM_ATTACH` reader - hub_port_perm_attach Indicates if the device attached to a downstream port is permanently attached or not."]
pub type HubPortPermAttachR = crate::FieldReader<HubPortPermAttach>;
impl HubPortPermAttachR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HubPortPermAttach> {
        match self.bits {
            0 => Some(HubPortPermAttach::B0),
            1 => Some(HubPortPermAttach::B1),
            _ => None,
        }
    }
    #[doc = "Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HubPortPermAttach::B0
    }
    #[doc = "Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HubPortPermAttach::B1
    }
}
#[doc = "Field `HUB_PORT_PERM_ATTACH` writer - hub_port_perm_attach Indicates if the device attached to a downstream port is permanently attached or not."]
pub type HubPortPermAttachW<'a, REG> = crate::FieldWriter<'a, REG, 2, HubPortPermAttach>;
impl<'a, REG> HubPortPermAttachW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HubPortPermAttach::B0)
    }
    #[doc = "Permanently attached Bit0 is for USB2.0 port and bit1 are for USB 3.0 SS port."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HubPortPermAttach::B1)
    }
}
#[doc = "Field `FLADJ_30MHZ_REG` reader - fladj_30mhz_reg HS Jitter Adjustment. Indicates the correction required to accommodate mac3 clock and utmi clock jitter to measure 125 's duration. With fladj_30mhz_reg tied to zero, the high speed 125us micro-frame is counted for 123933ns. You must program the value in terms of high speed bit times in a 30 MHz cycle. The default value that must be driven is 32 (assuming 30 MHz perfect clock)."]
pub type Fladj30mhzRegR = crate::FieldReader;
#[doc = "Field `FLADJ_30MHZ_REG` writer - fladj_30mhz_reg HS Jitter Adjustment. Indicates the correction required to accommodate mac3 clock and utmi clock jitter to measure 125 's duration. With fladj_30mhz_reg tied to zero, the high speed 125us micro-frame is counted for 123933ns. You must program the value in terms of high speed bit times in a 30 MHz cycle. The default value that must be driven is 32 (assuming 30 MHz perfect clock)."]
pub type Fladj30mhzRegW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "host_port_power_control_present This indicates whether the host controller implementation includes port power control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostPortPowerControlPresent {
    #[doc = "0: Indicates that the port has port power switches"]
    B0 = 0,
    #[doc = "1: Indicates that the port has port power switches"]
    B1 = 1,
}
impl From<HostPortPowerControlPresent> for bool {
    #[inline(always)]
    fn from(variant: HostPortPowerControlPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_PORT_POWER_CONTROL_PRESENT` reader - host_port_power_control_present This indicates whether the host controller implementation includes port power control."]
pub type HostPortPowerControlPresentR = crate::BitReader<HostPortPowerControlPresent>;
impl HostPortPowerControlPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostPortPowerControlPresent {
        match self.bits {
            false => HostPortPowerControlPresent::B0,
            true => HostPortPowerControlPresent::B1,
        }
    }
    #[doc = "Indicates that the port has port power switches"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HostPortPowerControlPresent::B0
    }
    #[doc = "Indicates that the port has port power switches"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HostPortPowerControlPresent::B1
    }
}
#[doc = "Field `HOST_PORT_POWER_CONTROL_PRESENT` writer - host_port_power_control_present This indicates whether the host controller implementation includes port power control."]
pub type HostPortPowerControlPresentW<'a, REG> =
    crate::BitWriter<'a, REG, HostPortPowerControlPresent>;
impl<'a, REG> HostPortPowerControlPresentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Indicates that the port has port power switches"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HostPortPowerControlPresent::B0)
    }
    #[doc = "Indicates that the port has port power switches"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HostPortPowerControlPresent::B1)
    }
}
#[doc = "host_u2_port_disable USB2.0 Port Disable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostU2PortDisable {
    #[doc = "0: Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    B0 = 0,
    #[doc = "1: Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    B1 = 1,
}
impl From<HostU2PortDisable> for bool {
    #[inline(always)]
    fn from(variant: HostU2PortDisable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_U2_PORT_DISABLE` reader - host_u2_port_disable USB2.0 Port Disable control."]
pub type HostU2PortDisableR = crate::BitReader<HostU2PortDisable>;
impl HostU2PortDisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostU2PortDisable {
        match self.bits {
            false => HostU2PortDisable::B0,
            true => HostU2PortDisable::B1,
        }
    }
    #[doc = "Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HostU2PortDisable::B0
    }
    #[doc = "Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HostU2PortDisable::B1
    }
}
#[doc = "Field `HOST_U2_PORT_DISABLE` writer - host_u2_port_disable USB2.0 Port Disable control."]
pub type HostU2PortDisableW<'a, REG> = crate::BitWriter<'a, REG, HostU2PortDisable>;
impl<'a, REG> HostU2PortDisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HostU2PortDisable::B0)
    }
    #[doc = "Port Disabled When 1, this signal stops reporting connect/disconnect events the port and keeps the port in disabled state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HostU2PortDisable::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - bus_filter_bypass It is expected that this signal is set or reset at power-on reset and is not changed during the normal operation of the core. The function of each bit is: bus_filter_bypass\\[3\\]: Bypass the filter for utmiotg_iddig bus_filter_bypass\\[2\\]: Bypass the filters for utmisrp_bvalid and utmisrp_sessend bus_filter_bypass\\[1\\]: Bypass the filter for pipe3_PowerPresent all U3 ports bus_filter_bypass\\[0\\]: Bypass the filter for utmiotg_vbusvalid all U2 ports In non-OTG Host-only mode, internal bus filters are not needed. Values:"]
    #[inline(always)]
    pub fn bus_filter_bypass(&self) -> BusFilterBypassR {
        BusFilterBypassR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - hub_port_overcurrent This is the per port Overcurrent indication of the root-hub ports:"]
    #[inline(always)]
    pub fn hub_port_overcurrent(&self) -> HubPortOvercurrentR {
        HubPortOvercurrentR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - hub_port_perm_attach Indicates if the device attached to a downstream port is permanently attached or not."]
    #[inline(always)]
    pub fn hub_port_perm_attach(&self) -> HubPortPermAttachR {
        HubPortPermAttachR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - fladj_30mhz_reg HS Jitter Adjustment. Indicates the correction required to accommodate mac3 clock and utmi clock jitter to measure 125 's duration. With fladj_30mhz_reg tied to zero, the high speed 125us micro-frame is counted for 123933ns. You must program the value in terms of high speed bit times in a 30 MHz cycle. The default value that must be driven is 32 (assuming 30 MHz perfect clock)."]
    #[inline(always)]
    pub fn fladj_30mhz_reg(&self) -> Fladj30mhzRegR {
        Fladj30mhzRegR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - host_port_power_control_present This indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    pub fn host_port_power_control_present(&self) -> HostPortPowerControlPresentR {
        HostPortPowerControlPresentR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - host_u2_port_disable USB2.0 Port Disable control."]
    #[inline(always)]
    pub fn host_u2_port_disable(&self) -> HostU2PortDisableR {
        HostU2PortDisableR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - bus_filter_bypass It is expected that this signal is set or reset at power-on reset and is not changed during the normal operation of the core. The function of each bit is: bus_filter_bypass\\[3\\]: Bypass the filter for utmiotg_iddig bus_filter_bypass\\[2\\]: Bypass the filters for utmisrp_bvalid and utmisrp_sessend bus_filter_bypass\\[1\\]: Bypass the filter for pipe3_PowerPresent all U3 ports bus_filter_bypass\\[0\\]: Bypass the filter for utmiotg_vbusvalid all U2 ports In non-OTG Host-only mode, internal bus filters are not needed. Values:"]
    #[inline(always)]
    #[must_use]
    pub fn bus_filter_bypass(&mut self) -> BusFilterBypassW<GrfUsb3otg0Con0Spec> {
        BusFilterBypassW::new(self, 0)
    }
    #[doc = "Bits 4:5 - hub_port_overcurrent This is the per port Overcurrent indication of the root-hub ports:"]
    #[inline(always)]
    #[must_use]
    pub fn hub_port_overcurrent(&mut self) -> HubPortOvercurrentW<GrfUsb3otg0Con0Spec> {
        HubPortOvercurrentW::new(self, 4)
    }
    #[doc = "Bits 6:7 - hub_port_perm_attach Indicates if the device attached to a downstream port is permanently attached or not."]
    #[inline(always)]
    #[must_use]
    pub fn hub_port_perm_attach(&mut self) -> HubPortPermAttachW<GrfUsb3otg0Con0Spec> {
        HubPortPermAttachW::new(self, 6)
    }
    #[doc = "Bits 8:13 - fladj_30mhz_reg HS Jitter Adjustment. Indicates the correction required to accommodate mac3 clock and utmi clock jitter to measure 125 's duration. With fladj_30mhz_reg tied to zero, the high speed 125us micro-frame is counted for 123933ns. You must program the value in terms of high speed bit times in a 30 MHz cycle. The default value that must be driven is 32 (assuming 30 MHz perfect clock)."]
    #[inline(always)]
    #[must_use]
    pub fn fladj_30mhz_reg(&mut self) -> Fladj30mhzRegW<GrfUsb3otg0Con0Spec> {
        Fladj30mhzRegW::new(self, 8)
    }
    #[doc = "Bit 14 - host_port_power_control_present This indicates whether the host controller implementation includes port power control."]
    #[inline(always)]
    #[must_use]
    pub fn host_port_power_control_present(
        &mut self,
    ) -> HostPortPowerControlPresentW<GrfUsb3otg0Con0Spec> {
        HostPortPowerControlPresentW::new(self, 14)
    }
    #[doc = "Bit 15 - host_u2_port_disable USB2.0 Port Disable control."]
    #[inline(always)]
    #[must_use]
    pub fn host_u2_port_disable(&mut self) -> HostU2PortDisableW<GrfUsb3otg0Con0Spec> {
        HostU2PortDisableW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb3otg0Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "USB3 OTG0 GRF Register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3otg0_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3otg0_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3otg0Con0Spec;
impl crate::RegisterSpec for GrfUsb3otg0Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3otg0_con0::R`](R) reader structure"]
impl crate::Readable for GrfUsb3otg0Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3otg0_con0::W`](W) writer structure"]
impl crate::Writable for GrfUsb3otg0Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3OTG0_CON0 to value 0x2000"]
impl crate::Resettable for GrfUsb3otg0Con0Spec {
    const RESET_VALUE: u32 = 0x2000;
}
