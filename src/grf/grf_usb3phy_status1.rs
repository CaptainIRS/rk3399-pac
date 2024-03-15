#[doc = "Register `GRF_USB3PHY_STATUS1` reader"]
pub type R = crate::R<GrfUsb3phyStatus1Spec>;
#[doc = "Register `GRF_USB3PHY_STATUS1` writer"]
pub type W = crate::W<GrfUsb3phyStatus1Spec>;
#[doc = "TCPC connect orientation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc0ConnOrientation {
    #[doc = "0: flip"]
    B0 = 0,
    #[doc = "1: flip"]
    B1 = 1,
}
impl From<Tcpc0ConnOrientation> for bool {
    #[inline(always)]
    fn from(variant: Tcpc0ConnOrientation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC0_CONN_ORIENTATION` reader - TCPC connect orientation"]
pub type Tcpc0ConnOrientationR = crate::BitReader<Tcpc0ConnOrientation>;
impl Tcpc0ConnOrientationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0ConnOrientation {
        match self.bits {
            false => Tcpc0ConnOrientation::B0,
            true => Tcpc0ConnOrientation::B1,
        }
    }
    #[doc = "flip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc0ConnOrientation::B0
    }
    #[doc = "flip"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc0ConnOrientation::B1
    }
}
#[doc = "TCPC connect present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc0ConnPresent {
    #[doc = "0: Connected"]
    B0 = 0,
    #[doc = "1: Connected"]
    B1 = 1,
}
impl From<Tcpc0ConnPresent> for bool {
    #[inline(always)]
    fn from(variant: Tcpc0ConnPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC0_CONN_PRESENT` reader - TCPC connect present"]
pub type Tcpc0ConnPresentR = crate::BitReader<Tcpc0ConnPresent>;
impl Tcpc0ConnPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0ConnPresent {
        match self.bits {
            false => Tcpc0ConnPresent::B0,
            true => Tcpc0ConnPresent::B1,
        }
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc0ConnPresent::B0
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc0ConnPresent::B1
    }
}
#[doc = "TCPC MUX CTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcpc0MuxCtrl {
    #[doc = "0: USB3.1 and DP 2 lanes"]
    D0 = 0,
    #[doc = "1: USB3.1 and DP 2 lanes"]
    D1 = 1,
    #[doc = "2: USB3.1 and DP 2 lanes"]
    D2 = 2,
    #[doc = "3: USB3.1 and DP 2 lanes"]
    D3 = 3,
}
impl From<Tcpc0MuxCtrl> for u8 {
    #[inline(always)]
    fn from(variant: Tcpc0MuxCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcpc0MuxCtrl {
    type Ux = u8;
}
#[doc = "Field `TCPC0_MUX_CTRL` reader - TCPC MUX CTRL"]
pub type Tcpc0MuxCtrlR = crate::FieldReader<Tcpc0MuxCtrl>;
impl Tcpc0MuxCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0MuxCtrl {
        match self.bits {
            0 => Tcpc0MuxCtrl::D0,
            1 => Tcpc0MuxCtrl::D1,
            2 => Tcpc0MuxCtrl::D2,
            3 => Tcpc0MuxCtrl::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Tcpc0MuxCtrl::D0
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Tcpc0MuxCtrl::D1
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Tcpc0MuxCtrl::D2
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Tcpc0MuxCtrl::D3
    }
}
#[doc = "TCPC active cable connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc0ActCableConnN {
    #[doc = "0: Active cable connected"]
    B0 = 0,
    #[doc = "1: Active cable connected"]
    B1 = 1,
}
impl From<Tcpc0ActCableConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc0ActCableConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC0_ACT_CABLE_CONN_N` reader - TCPC active cable connect"]
pub type Tcpc0ActCableConnNR = crate::BitReader<Tcpc0ActCableConnN>;
impl Tcpc0ActCableConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0ActCableConnN {
        match self.bits {
            false => Tcpc0ActCableConnN::B0,
            true => Tcpc0ActCableConnN::B1,
        }
    }
    #[doc = "Active cable connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc0ActCableConnN::B0
    }
    #[doc = "Active cable connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc0ActCableConnN::B1
    }
}
#[doc = "TCPC audio accessory connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc0AudioAccConnN {
    #[doc = "0: No audio accessory connected"]
    B0 = 0,
    #[doc = "1: No audio accessory connected"]
    B1 = 1,
}
impl From<Tcpc0AudioAccConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc0AudioAccConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC0_AUDIO_ACC_CONN_N` reader - TCPC audio accessory connect"]
pub type Tcpc0AudioAccConnNR = crate::BitReader<Tcpc0AudioAccConnN>;
impl Tcpc0AudioAccConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0AudioAccConnN {
        match self.bits {
            false => Tcpc0AudioAccConnN::B0,
            true => Tcpc0AudioAccConnN::B1,
        }
    }
    #[doc = "No audio accessory connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc0AudioAccConnN::B0
    }
    #[doc = "No audio accessory connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc0AudioAccConnN::B1
    }
}
#[doc = "TCPC debug accessory connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc0DbgAccConnN {
    #[doc = "0: No debug accessory connected"]
    B0 = 0,
    #[doc = "1: No debug accessory connected"]
    B1 = 1,
}
impl From<Tcpc0DbgAccConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc0DbgAccConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC0_DBG_ACC_CONN_N` reader - TCPC debug accessory connect"]
pub type Tcpc0DbgAccConnNR = crate::BitReader<Tcpc0DbgAccConnN>;
impl Tcpc0DbgAccConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc0DbgAccConnN {
        match self.bits {
            false => Tcpc0DbgAccConnN::B0,
            true => Tcpc0DbgAccConnN::B1,
        }
    }
    #[doc = "No debug accessory connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc0DbgAccConnN::B0
    }
    #[doc = "No debug accessory connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc0DbgAccConnN::B1
    }
}
#[doc = "Field `TCPC0_OUTS_TO_HIZ` reader - TCPC outs to hiz"]
pub type Tcpc0OutsToHizR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_OVERVOLTAGE_EN` reader - TCPC vbus overvoltage enable 1: enable"]
pub type Tcpc0VbusOvervoltageEnR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_VOLTAGE_EN` reader - TCPC vbus voltage enable 1: enable"]
pub type Tcpc0VbusVoltageEnR = crate::BitReader;
#[doc = "Field `TCPC0_VCONN_OVERCURRENT_EN` reader - TCPC vconn overcurrent enable 1: enable"]
pub type Tcpc0VconnOvercurrentEnR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_OVERCURRENT_EN` reader - TCPC vbus over current enable 1: enable"]
pub type Tcpc0VbusOvercurrentEnR = crate::BitReader;
#[doc = "Field `TCPC0_VBUS_OVERCURRENT_EN` writer - TCPC vbus over current enable 1: enable"]
pub type Tcpc0VbusOvercurrentEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "TCPC connect orientation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc1ConnOrientation {
    #[doc = "0: flip"]
    B0 = 0,
    #[doc = "1: flip"]
    B1 = 1,
}
impl From<Tcpc1ConnOrientation> for bool {
    #[inline(always)]
    fn from(variant: Tcpc1ConnOrientation) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC1_CONN_ORIENTATION` reader - TCPC connect orientation"]
pub type Tcpc1ConnOrientationR = crate::BitReader<Tcpc1ConnOrientation>;
impl Tcpc1ConnOrientationR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1ConnOrientation {
        match self.bits {
            false => Tcpc1ConnOrientation::B0,
            true => Tcpc1ConnOrientation::B1,
        }
    }
    #[doc = "flip"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc1ConnOrientation::B0
    }
    #[doc = "flip"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc1ConnOrientation::B1
    }
}
#[doc = "TCPC connect present\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc1ConnPresent {
    #[doc = "0: Connected"]
    B0 = 0,
    #[doc = "1: Connected"]
    B1 = 1,
}
impl From<Tcpc1ConnPresent> for bool {
    #[inline(always)]
    fn from(variant: Tcpc1ConnPresent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC1_CONN_PRESENT` reader - TCPC connect present"]
pub type Tcpc1ConnPresentR = crate::BitReader<Tcpc1ConnPresent>;
impl Tcpc1ConnPresentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1ConnPresent {
        match self.bits {
            false => Tcpc1ConnPresent::B0,
            true => Tcpc1ConnPresent::B1,
        }
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc1ConnPresent::B0
    }
    #[doc = "Connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc1ConnPresent::B1
    }
}
#[doc = "TCPC MUX CTRL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcpc1MuxCtrl {
    #[doc = "0: USB3.1 and DP 2 lanes"]
    D0 = 0,
    #[doc = "1: USB3.1 and DP 2 lanes"]
    D1 = 1,
    #[doc = "2: USB3.1 and DP 2 lanes"]
    D2 = 2,
    #[doc = "3: USB3.1 and DP 2 lanes"]
    D3 = 3,
}
impl From<Tcpc1MuxCtrl> for u8 {
    #[inline(always)]
    fn from(variant: Tcpc1MuxCtrl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcpc1MuxCtrl {
    type Ux = u8;
}
#[doc = "Field `TCPC1_MUX_CTRL` reader - TCPC MUX CTRL"]
pub type Tcpc1MuxCtrlR = crate::FieldReader<Tcpc1MuxCtrl>;
impl Tcpc1MuxCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1MuxCtrl {
        match self.bits {
            0 => Tcpc1MuxCtrl::D0,
            1 => Tcpc1MuxCtrl::D1,
            2 => Tcpc1MuxCtrl::D2,
            3 => Tcpc1MuxCtrl::D3,
            _ => unreachable!(),
        }
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d0(&self) -> bool {
        *self == Tcpc1MuxCtrl::D0
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d1(&self) -> bool {
        *self == Tcpc1MuxCtrl::D1
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d2(&self) -> bool {
        *self == Tcpc1MuxCtrl::D2
    }
    #[doc = "USB3.1 and DP 2 lanes"]
    #[inline(always)]
    pub fn is_d3(&self) -> bool {
        *self == Tcpc1MuxCtrl::D3
    }
}
#[doc = "TCPC active cable connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc1ActCableConnN {
    #[doc = "0: Active cable connected"]
    B0 = 0,
    #[doc = "1: Active cable connected"]
    B1 = 1,
}
impl From<Tcpc1ActCableConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc1ActCableConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC1_ACT_CABLE_CONN_N` reader - TCPC active cable connect"]
pub type Tcpc1ActCableConnNR = crate::BitReader<Tcpc1ActCableConnN>;
impl Tcpc1ActCableConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1ActCableConnN {
        match self.bits {
            false => Tcpc1ActCableConnN::B0,
            true => Tcpc1ActCableConnN::B1,
        }
    }
    #[doc = "Active cable connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc1ActCableConnN::B0
    }
    #[doc = "Active cable connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc1ActCableConnN::B1
    }
}
#[doc = "TCPC audio accessory connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc1AudioAccConnN {
    #[doc = "0: No audio accessory connected"]
    B0 = 0,
    #[doc = "1: No audio accessory connected"]
    B1 = 1,
}
impl From<Tcpc1AudioAccConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc1AudioAccConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC1_AUDIO_ACC_CONN_N` reader - TCPC audio accessory connect"]
pub type Tcpc1AudioAccConnNR = crate::BitReader<Tcpc1AudioAccConnN>;
impl Tcpc1AudioAccConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1AudioAccConnN {
        match self.bits {
            false => Tcpc1AudioAccConnN::B0,
            true => Tcpc1AudioAccConnN::B1,
        }
    }
    #[doc = "No audio accessory connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc1AudioAccConnN::B0
    }
    #[doc = "No audio accessory connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc1AudioAccConnN::B1
    }
}
#[doc = "TCPC debug accessory connect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcpc1DbgAccConnN {
    #[doc = "0: No debug accessory connected"]
    B0 = 0,
    #[doc = "1: No debug accessory connected"]
    B1 = 1,
}
impl From<Tcpc1DbgAccConnN> for bool {
    #[inline(always)]
    fn from(variant: Tcpc1DbgAccConnN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC1_DBG_ACC_CONN_N` reader - TCPC debug accessory connect"]
pub type Tcpc1DbgAccConnNR = crate::BitReader<Tcpc1DbgAccConnN>;
impl Tcpc1DbgAccConnNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcpc1DbgAccConnN {
        match self.bits {
            false => Tcpc1DbgAccConnN::B0,
            true => Tcpc1DbgAccConnN::B1,
        }
    }
    #[doc = "No debug accessory connected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcpc1DbgAccConnN::B0
    }
    #[doc = "No debug accessory connected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcpc1DbgAccConnN::B1
    }
}
#[doc = "Field `TCPC1_OUTS_TO_HIZ` reader - TCPC outs to hiz"]
pub type Tcpc1OutsToHizR = crate::BitReader;
#[doc = "Field `TCPC1_VBUS_OVERVOLTAGE_EN` reader - TCPC vbus overvoltage enable 1: enable"]
pub type Tcpc1VbusOvervoltageEnR = crate::BitReader;
#[doc = "Field `TCPC1_VBUS_VOLTAGE_EN` reader - TCPC vbus voltage enable 1: enable"]
pub type Tcpc1VbusVoltageEnR = crate::BitReader;
#[doc = "Field `TCPC1_VCONN_OVERCURRENT_EN` reader - TCPC vconn overcurrent enable 1: enable"]
pub type Tcpc1VconnOvercurrentEnR = crate::BitReader;
#[doc = "Field `TCPC1_VBUS_OVERCURRENT_EN` reader - TCPC vbus over current enable 1: enable"]
pub type Tcpc1VbusOvercurrentEnR = crate::BitReader;
#[doc = "Field `TCPC1_VBUS_OVERCURRENT_EN` writer - TCPC vbus over current enable 1: enable"]
pub type Tcpc1VbusOvercurrentEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CC dead battery indicator from IOMUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CcDeadBatteryN {
    #[doc = "0: No dead battery happen"]
    B0 = 0,
    #[doc = "1: No dead battery happen"]
    B1 = 1,
}
impl From<CcDeadBatteryN> for bool {
    #[inline(always)]
    fn from(variant: CcDeadBatteryN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC_DEAD_BATTERY_N` reader - CC dead battery indicator from IOMUX"]
pub type CcDeadBatteryNR = crate::BitReader<CcDeadBatteryN>;
impl CcDeadBatteryNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CcDeadBatteryN {
        match self.bits {
            false => CcDeadBatteryN::B0,
            true => CcDeadBatteryN::B1,
        }
    }
    #[doc = "No dead battery happen"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CcDeadBatteryN::B0
    }
    #[doc = "No dead battery happen"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CcDeadBatteryN::B1
    }
}
impl R {
    #[doc = "Bit 0 - TCPC connect orientation"]
    #[inline(always)]
    pub fn tcpc0_conn_orientation(&self) -> Tcpc0ConnOrientationR {
        Tcpc0ConnOrientationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCPC connect present"]
    #[inline(always)]
    pub fn tcpc0_conn_present(&self) -> Tcpc0ConnPresentR {
        Tcpc0ConnPresentR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - TCPC MUX CTRL"]
    #[inline(always)]
    pub fn tcpc0_mux_ctrl(&self) -> Tcpc0MuxCtrlR {
        Tcpc0MuxCtrlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - TCPC active cable connect"]
    #[inline(always)]
    pub fn tcpc0_act_cable_conn_n(&self) -> Tcpc0ActCableConnNR {
        Tcpc0ActCableConnNR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCPC audio accessory connect"]
    #[inline(always)]
    pub fn tcpc0_audio_acc_conn_n(&self) -> Tcpc0AudioAccConnNR {
        Tcpc0AudioAccConnNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TCPC debug accessory connect"]
    #[inline(always)]
    pub fn tcpc0_dbg_acc_conn_n(&self) -> Tcpc0DbgAccConnNR {
        Tcpc0DbgAccConnNR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TCPC outs to hiz"]
    #[inline(always)]
    pub fn tcpc0_outs_to_hiz(&self) -> Tcpc0OutsToHizR {
        Tcpc0OutsToHizR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TCPC vbus overvoltage enable 1: enable"]
    #[inline(always)]
    pub fn tcpc0_vbus_overvoltage_en(&self) -> Tcpc0VbusOvervoltageEnR {
        Tcpc0VbusOvervoltageEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCPC vbus voltage enable 1: enable"]
    #[inline(always)]
    pub fn tcpc0_vbus_voltage_en(&self) -> Tcpc0VbusVoltageEnR {
        Tcpc0VbusVoltageEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TCPC vconn overcurrent enable 1: enable"]
    #[inline(always)]
    pub fn tcpc0_vconn_overcurrent_en(&self) -> Tcpc0VconnOvercurrentEnR {
        Tcpc0VconnOvercurrentEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCPC vbus over current enable 1: enable"]
    #[inline(always)]
    pub fn tcpc0_vbus_overcurrent_en(&self) -> Tcpc0VbusOvercurrentEnR {
        Tcpc0VbusOvercurrentEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - TCPC connect orientation"]
    #[inline(always)]
    pub fn tcpc1_conn_orientation(&self) -> Tcpc1ConnOrientationR {
        Tcpc1ConnOrientationR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCPC connect present"]
    #[inline(always)]
    pub fn tcpc1_conn_present(&self) -> Tcpc1ConnPresentR {
        Tcpc1ConnPresentR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - TCPC MUX CTRL"]
    #[inline(always)]
    pub fn tcpc1_mux_ctrl(&self) -> Tcpc1MuxCtrlR {
        Tcpc1MuxCtrlR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - TCPC active cable connect"]
    #[inline(always)]
    pub fn tcpc1_act_cable_conn_n(&self) -> Tcpc1ActCableConnNR {
        Tcpc1ActCableConnNR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCPC audio accessory connect"]
    #[inline(always)]
    pub fn tcpc1_audio_acc_conn_n(&self) -> Tcpc1AudioAccConnNR {
        Tcpc1AudioAccConnNR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TCPC debug accessory connect"]
    #[inline(always)]
    pub fn tcpc1_dbg_acc_conn_n(&self) -> Tcpc1DbgAccConnNR {
        Tcpc1DbgAccConnNR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TCPC outs to hiz"]
    #[inline(always)]
    pub fn tcpc1_outs_to_hiz(&self) -> Tcpc1OutsToHizR {
        Tcpc1OutsToHizR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TCPC vbus overvoltage enable 1: enable"]
    #[inline(always)]
    pub fn tcpc1_vbus_overvoltage_en(&self) -> Tcpc1VbusOvervoltageEnR {
        Tcpc1VbusOvervoltageEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TCPC vbus voltage enable 1: enable"]
    #[inline(always)]
    pub fn tcpc1_vbus_voltage_en(&self) -> Tcpc1VbusVoltageEnR {
        Tcpc1VbusVoltageEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TCPC vconn overcurrent enable 1: enable"]
    #[inline(always)]
    pub fn tcpc1_vconn_overcurrent_en(&self) -> Tcpc1VconnOvercurrentEnR {
        Tcpc1VconnOvercurrentEnR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TCPC vbus over current enable 1: enable"]
    #[inline(always)]
    pub fn tcpc1_vbus_overcurrent_en(&self) -> Tcpc1VbusOvercurrentEnR {
        Tcpc1VbusOvercurrentEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - CC dead battery indicator from IOMUX"]
    #[inline(always)]
    pub fn cc_dead_battery_n(&self) -> CcDeadBatteryNR {
        CcDeadBatteryNR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - TCPC vbus over current enable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc0_vbus_overcurrent_en(&mut self) -> Tcpc0VbusOvercurrentEnW<GrfUsb3phyStatus1Spec> {
        Tcpc0VbusOvercurrentEnW::new(self, 11)
    }
    #[doc = "Bit 27 - TCPC vbus over current enable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc1_vbus_overcurrent_en(&mut self) -> Tcpc1VbusOvercurrentEnW<GrfUsb3phyStatus1Spec> {
        Tcpc1VbusOvercurrentEnW::new(self, 27)
    }
}
#[doc = "USB3PHY_STATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy_status1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy_status1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3phyStatus1Spec;
impl crate::RegisterSpec for GrfUsb3phyStatus1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3phy_status1::R`](R) reader structure"]
impl crate::Readable for GrfUsb3phyStatus1Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3phy_status1::W`](W) writer structure"]
impl crate::Writable for GrfUsb3phyStatus1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3PHY_STATUS1 to value 0"]
impl crate::Resettable for GrfUsb3phyStatus1Spec {
    const RESET_VALUE: u32 = 0;
}
