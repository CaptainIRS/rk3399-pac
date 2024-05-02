#[doc = "Register `CONFIG_STANDARD_OUTPUT` reader"]
pub type R = crate::R<ConfigStandardOutputSpec>;
#[doc = "Register `CONFIG_STANDARD_OUTPUT` writer"]
pub type W = crate::W<ConfigStandardOutputSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectorOrientation {
    #[doc = "0: Normal (CC1=A5, CC2=B5,TX1=A2/A3, RX1=B10/B11)default"]
    B0 = 0,
    #[doc = "1: Flipped (CC2=A5, CC1=B5,TX1=B2/B3, RX1=A10/A11) Controlled by the TCPM The TCPC shall ignore writes to this bit if TCPC_CONTROL.DebugAccessoryControl =0"]
    B1 = 1,
}
impl From<ConnectorOrientation> for bool {
    #[inline(always)]
    fn from(variant: ConnectorOrientation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Connector_Orientation` reader - "]
pub type ConnectorOrientationR = crate::BitReader<ConnectorOrientation>;
impl ConnectorOrientationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConnectorOrientation {
        match self.bits {
            false => ConnectorOrientation::B0,
            true => ConnectorOrientation::B1,
        }
    }
    #[doc = "Normal (CC1=A5, CC2=B5,TX1=A2/A3, RX1=B10/B11)default"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ConnectorOrientation::B0
    }
    #[doc = "Flipped (CC2=A5, CC1=B5,TX1=B2/B3, RX1=A10/A11) Controlled by the TCPM The TCPC shall ignore writes to this bit if TCPC_CONTROL.DebugAccessoryControl =0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ConnectorOrientation::B1
    }
}
#[doc = "Field `Connector_Orientation` writer - "]
pub type ConnectorOrientationW<'a, REG> = crate::BitWriter<'a, REG, ConnectorOrientation>;
impl<'a, REG> ConnectorOrientationW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal (CC1=A5, CC2=B5,TX1=A2/A3, RX1=B10/B11)default"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectorOrientation::B0)
    }
    #[doc = "Flipped (CC2=A5, CC1=B5,TX1=B2/B3, RX1=A10/A11) Controlled by the TCPM The TCPC shall ignore writes to this bit if TCPC_CONTROL.DebugAccessoryControl =0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectorOrientation::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionPresent {
    #[doc = "0: No Connection (default)"]
    B0 = 0,
    #[doc = "1: Connection Controlled by the TCPM."]
    B1 = 1,
}
impl From<ConnectionPresent> for bool {
    #[inline(always)]
    fn from(variant: ConnectionPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Connection_Present` reader - "]
pub type ConnectionPresentR = crate::BitReader<ConnectionPresent>;
impl ConnectionPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ConnectionPresent {
        match self.bits {
            false => ConnectionPresent::B0,
            true => ConnectionPresent::B1,
        }
    }
    #[doc = "No Connection (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ConnectionPresent::B0
    }
    #[doc = "Connection Controlled by the TCPM."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ConnectionPresent::B1
    }
}
#[doc = "Field `Connection_Present` writer - "]
pub type ConnectionPresentW<'a, REG> = crate::BitWriter<'a, REG, ConnectionPresent>;
impl<'a, REG> ConnectionPresentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Connection (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectionPresent::B0)
    }
    #[doc = "Connection Controlled by the TCPM."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ConnectionPresent::B1)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MuxControl {
    #[doc = "0: No connection (default)"]
    B00 = 0,
    #[doc = "1: USB3.1Connected"]
    B01 = 1,
    #[doc = "2: DP Alternate Mode - 4lanes"]
    B10 = 2,
    #[doc = "3: USB3.1 + Display Port Lanes 0 &amp; 1 Controlled by the TCPM"]
    B11 = 3,
}
impl From<MuxControl> for u8 {
    #[inline(always)]
    fn from(variant: MuxControl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MuxControl {
    type Ux = u8;
}
#[doc = "Field `MUX_Control` reader - "]
pub type MuxControlR = crate::FieldReader<MuxControl>;
impl MuxControlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MuxControl {
        match self.bits {
            0 => MuxControl::B00,
            1 => MuxControl::B01,
            2 => MuxControl::B10,
            3 => MuxControl::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "No connection (default)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == MuxControl::B00
    }
    #[doc = "USB3.1Connected"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == MuxControl::B01
    }
    #[doc = "DP Alternate Mode - 4lanes"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == MuxControl::B10
    }
    #[doc = "USB3.1 + Display Port Lanes 0 &amp; 1 Controlled by the TCPM"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == MuxControl::B11
    }
}
#[doc = "Field `MUX_Control` writer - "]
pub type MuxControlW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, MuxControl>;
impl<'a, REG> MuxControlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No connection (default)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(MuxControl::B00)
    }
    #[doc = "USB3.1Connected"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(MuxControl::B01)
    }
    #[doc = "DP Alternate Mode - 4lanes"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(MuxControl::B10)
    }
    #[doc = "USB3.1 + Display Port Lanes 0 &amp; 1 Controlled by the TCPM"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(MuxControl::B11)
    }
}
#[doc = "Field `Active_Cable_Connected` reader - 0b: \n\nNo \n\nActive \n\nCable \n\nconnected \n\n(default) 1b: Active Cable connected \n\nControlled by the TCPM"]
pub type ActiveCableConnectedR = crate::BitReader;
#[doc = "Field `Active_Cable_Connected` writer - 0b: \n\nNo \n\nActive \n\nCable \n\nconnected \n\n(default) 1b: Active Cable connected \n\nControlled by the TCPM"]
pub type ActiveCableConnectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioAccessoryConnected {
    #[doc = "0: Audio Accessory connected"]
    B0 = 0,
    #[doc = "1: No Audio Accessory connected(default) Controlled by the TCPM"]
    B1 = 1,
}
impl From<AudioAccessoryConnected> for bool {
    #[inline(always)]
    fn from(variant: AudioAccessoryConnected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Audio_Accessory_Connected` reader - "]
pub type AudioAccessoryConnectedR = crate::BitReader<AudioAccessoryConnected>;
impl AudioAccessoryConnectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AudioAccessoryConnected {
        match self.bits {
            false => AudioAccessoryConnected::B0,
            true => AudioAccessoryConnected::B1,
        }
    }
    #[doc = "Audio Accessory connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == AudioAccessoryConnected::B0
    }
    #[doc = "No Audio Accessory connected(default) Controlled by the TCPM"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == AudioAccessoryConnected::B1
    }
}
#[doc = "Field `Audio_Accessory_Connected` writer - "]
pub type AudioAccessoryConnectedW<'a, REG> = crate::BitWriter<'a, REG, AudioAccessoryConnected>;
impl<'a, REG> AudioAccessoryConnectedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Audio Accessory connected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(AudioAccessoryConnected::B0)
    }
    #[doc = "No Audio Accessory connected(default) Controlled by the TCPM"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(AudioAccessoryConnected::B1)
    }
}
#[doc = "Field `Debug_Accessory_Connected` reader - 0b: Debug Accessory Connected# output \n\ndriven \n\nlow 1b: Debug Accessory Connected# \n\noutput is driven high(default) \n\nControlled by either the TCPM or TCPC. The \n\nTCPC \n\nshall \n\nignore writes \n\nto \n\nthis \n\nbit \n\nif \n\nTCPC_CONTROL.DebugAccessoryControl =0b."]
pub type DebugAccessoryConnectedR = crate::BitReader;
#[doc = "Field `Debug_Accessory_Connected` writer - 0b: Debug Accessory Connected# output \n\ndriven \n\nlow 1b: Debug Accessory Connected# \n\noutput is driven high(default) \n\nControlled by either the TCPM or TCPC. The \n\nTCPC \n\nshall \n\nignore writes \n\nto \n\nthis \n\nbit \n\nif \n\nTCPC_CONTROL.DebugAccessoryControl =0b."]
pub type DebugAccessoryConnectedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighImpedanceOutputs {
    #[doc = "0: Standard output control (default)"]
    B0 = 0,
    #[doc = "1: Force all outputs to high impedance May be used to save power in Sleep Controlled by the TCPM."]
    B1 = 1,
}
impl From<HighImpedanceOutputs> for bool {
    #[inline(always)]
    fn from(variant: HighImpedanceOutputs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `High_Impedance_outputs` reader - "]
pub type HighImpedanceOutputsR = crate::BitReader<HighImpedanceOutputs>;
impl HighImpedanceOutputsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HighImpedanceOutputs {
        match self.bits {
            false => HighImpedanceOutputs::B0,
            true => HighImpedanceOutputs::B1,
        }
    }
    #[doc = "Standard output control (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HighImpedanceOutputs::B0
    }
    #[doc = "Force all outputs to high impedance May be used to save power in Sleep Controlled by the TCPM."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HighImpedanceOutputs::B1
    }
}
#[doc = "Field `High_Impedance_outputs` writer - "]
pub type HighImpedanceOutputsW<'a, REG> = crate::BitWriter<'a, REG, HighImpedanceOutputs>;
impl<'a, REG> HighImpedanceOutputsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard output control (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HighImpedanceOutputs::B0)
    }
    #[doc = "Force all outputs to high impedance May be used to save power in Sleep Controlled by the TCPM."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HighImpedanceOutputs::B1)
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn connector_orientation(&self) -> ConnectorOrientationR {
        ConnectorOrientationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn connection_present(&self) -> ConnectionPresentR {
        ConnectionPresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn mux_control(&self) -> MuxControlR {
        MuxControlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 0b: \n\nNo \n\nActive \n\nCable \n\nconnected \n\n(default) 1b: Active Cable connected \n\nControlled by the TCPM"]
    #[inline(always)]
    pub fn active_cable_connected(&self) -> ActiveCableConnectedR {
        ActiveCableConnectedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn audio_accessory_connected(&self) -> AudioAccessoryConnectedR {
        AudioAccessoryConnectedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0b: Debug Accessory Connected# output \n\ndriven \n\nlow 1b: Debug Accessory Connected# \n\noutput is driven high(default) \n\nControlled by either the TCPM or TCPC. The \n\nTCPC \n\nshall \n\nignore writes \n\nto \n\nthis \n\nbit \n\nif \n\nTCPC_CONTROL.DebugAccessoryControl =0b."]
    #[inline(always)]
    pub fn debug_accessory_connected(&self) -> DebugAccessoryConnectedR {
        DebugAccessoryConnectedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn high_impedance_outputs(&self) -> HighImpedanceOutputsR {
        HighImpedanceOutputsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn connector_orientation(&mut self) -> ConnectorOrientationW<ConfigStandardOutputSpec> {
        ConnectorOrientationW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn connection_present(&mut self) -> ConnectionPresentW<ConfigStandardOutputSpec> {
        ConnectionPresentW::new(self, 1)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn mux_control(&mut self) -> MuxControlW<ConfigStandardOutputSpec> {
        MuxControlW::new(self, 2)
    }
    #[doc = "Bit 4 - 0b: \n\nNo \n\nActive \n\nCable \n\nconnected \n\n(default) 1b: Active Cable connected \n\nControlled by the TCPM"]
    #[inline(always)]
    #[must_use]
    pub fn active_cable_connected(&mut self) -> ActiveCableConnectedW<ConfigStandardOutputSpec> {
        ActiveCableConnectedW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn audio_accessory_connected(
        &mut self,
    ) -> AudioAccessoryConnectedW<ConfigStandardOutputSpec> {
        AudioAccessoryConnectedW::new(self, 5)
    }
    #[doc = "Bit 6 - 0b: Debug Accessory Connected# output \n\ndriven \n\nlow 1b: Debug Accessory Connected# \n\noutput is driven high(default) \n\nControlled by either the TCPM or TCPC. The \n\nTCPC \n\nshall \n\nignore writes \n\nto \n\nthis \n\nbit \n\nif \n\nTCPC_CONTROL.DebugAccessoryControl =0b."]
    #[inline(always)]
    #[must_use]
    pub fn debug_accessory_connected(
        &mut self,
    ) -> DebugAccessoryConnectedW<ConfigStandardOutputSpec> {
        DebugAccessoryConnectedW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn high_impedance_outputs(&mut self) -> HighImpedanceOutputsW<ConfigStandardOutputSpec> {
        HighImpedanceOutputsW::new(self, 7)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<ConfigStandardOutputSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Configure Standard Output Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_standard_output::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_standard_output::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigStandardOutputSpec;
impl crate::RegisterSpec for ConfigStandardOutputSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_standard_output::R`](R) reader structure"]
impl crate::Readable for ConfigStandardOutputSpec {}
#[doc = "`write(|w| ..)` method takes [`config_standard_output::W`](W) writer structure"]
impl crate::Writable for ConfigStandardOutputSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_STANDARD_OUTPUT to value 0x60"]
impl crate::Resettable for ConfigStandardOutputSpec {
    const RESET_VALUE: u32 = 0x60;
}
