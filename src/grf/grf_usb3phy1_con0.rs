#[doc = "Register `GRF_USB3PHY1_CON0` reader"]
pub type R = crate::R<GrfUsb3phy1Con0Spec>;
#[doc = "Register `GRF_USB3PHY1_CON0` writer"]
pub type W = crate::W<GrfUsb3phy1Con0Spec>;
#[doc = "TypeC PHY connect direction\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypecConnDir {
    #[doc = "0: normal orientation"]
    B0 = 0,
    #[doc = "1: flip orientation"]
    B1 = 1,
}
impl From<TypecConnDir> for bool {
    #[inline(always)]
    fn from(variant: TypecConnDir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPEC_CONN_DIR` reader - TypeC PHY connect direction"]
pub type TypecConnDirR = crate::BitReader<TypecConnDir>;
impl TypecConnDirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TypecConnDir {
        match self.bits {
            false => TypecConnDir::B0,
            true => TypecConnDir::B1,
        }
    }
    #[doc = "normal orientation"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TypecConnDir::B0
    }
    #[doc = "flip orientation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TypecConnDir::B1
    }
}
#[doc = "Field `TYPEC_CONN_DIR` writer - TypeC PHY connect direction"]
pub type TypecConnDirW<'a, REG> = crate::BitWriter<'a, REG, TypecConnDir>;
impl<'a, REG> TypecConnDirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "normal orientation"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TypecConnDir::B0)
    }
    #[doc = "flip orientation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TypecConnDir::B1)
    }
}
#[doc = "Field `PIPE_DATA_BUS_WIDTH` reader - Pipe interface data bus width\n\n0: 32bit data bus width, only support 32bit\n\ndata bus width."]
pub type PipeDataBusWidthR = crate::FieldReader;
#[doc = "Field `PIPE_DATA_BUS_WIDTH` writer - Pipe interface data bus width\n\n0: 32bit data bus width, only support 32bit\n\ndata bus width."]
pub type PipeDataBusWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "force usb3 to usb2 enable control\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3tousb2En {
    #[doc = "1: force usb3 controller work as usb2."]
    B1 = 1,
    #[doc = "0: not force usb3 controller work as usb2."]
    B0 = 0,
}
impl From<Usb3tousb2En> for bool {
    #[inline(always)]
    fn from(variant: Usb3tousb2En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USB3TOUSB2_EN` reader - force usb3 to usb2 enable control"]
pub type Usb3tousb2EnR = crate::BitReader<Usb3tousb2En>;
impl Usb3tousb2EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3tousb2En {
        match self.bits {
            true => Usb3tousb2En::B1,
            false => Usb3tousb2En::B0,
        }
    }
    #[doc = "force usb3 controller work as usb2."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usb3tousb2En::B1
    }
    #[doc = "not force usb3 controller work as usb2."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usb3tousb2En::B0
    }
}
#[doc = "Field `USB3TOUSB2_EN` writer - force usb3 to usb2 enable control"]
pub type Usb3tousb2EnW<'a, REG> = crate::BitWriter<'a, REG, Usb3tousb2En>;
impl<'a, REG> Usb3tousb2EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "force usb3 controller work as usb2."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3tousb2En::B1)
    }
    #[doc = "not force usb3 controller work as usb2."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3tousb2En::B0)
    }
}
#[doc = "TCPC role trap\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TcpcRoleStrap {
    #[doc = "1: TCPC default as DFP"]
    B01 = 1,
    #[doc = "2: TCPC default as UFP"]
    B10 = 2,
    #[doc = "3: TCPC default as DRP"]
    B11 = 3,
}
impl From<TcpcRoleStrap> for u8 {
    #[inline(always)]
    fn from(variant: TcpcRoleStrap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TcpcRoleStrap {
    type Ux = u8;
}
#[doc = "Field `TCPC_ROLE_STRAP` reader - TCPC role trap"]
pub type TcpcRoleStrapR = crate::FieldReader<TcpcRoleStrap>;
impl TcpcRoleStrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TcpcRoleStrap> {
        match self.bits {
            1 => Some(TcpcRoleStrap::B01),
            2 => Some(TcpcRoleStrap::B10),
            3 => Some(TcpcRoleStrap::B11),
            _ => None,
        }
    }
    #[doc = "TCPC default as DFP"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == TcpcRoleStrap::B01
    }
    #[doc = "TCPC default as UFP"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == TcpcRoleStrap::B10
    }
    #[doc = "TCPC default as DRP"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == TcpcRoleStrap::B11
    }
}
#[doc = "Field `TCPC_ROLE_STRAP` writer - TCPC role trap"]
pub type TcpcRoleStrapW<'a, REG> = crate::FieldWriter<'a, REG, 2, TcpcRoleStrap>;
impl<'a, REG> TcpcRoleStrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TCPC default as DFP"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(TcpcRoleStrap::B01)
    }
    #[doc = "TCPC default as UFP"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(TcpcRoleStrap::B10)
    }
    #[doc = "TCPC default as DRP"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(TcpcRoleStrap::B11)
    }
}
#[doc = "dead_battery_sel\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadBatterySel {
    #[doc = "0: select external dead_battery_n from IOMUX"]
    B0 = 0,
    #[doc = "1: select internal bit7 of this register"]
    B1 = 1,
}
impl From<DeadBatterySel> for bool {
    #[inline(always)]
    fn from(variant: DeadBatterySel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEAD_BATTERY_SEL` reader - dead_battery_sel"]
pub type DeadBatterySelR = crate::BitReader<DeadBatterySel>;
impl DeadBatterySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeadBatterySel {
        match self.bits {
            false => DeadBatterySel::B0,
            true => DeadBatterySel::B1,
        }
    }
    #[doc = "select external dead_battery_n from IOMUX"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DeadBatterySel::B0
    }
    #[doc = "select internal bit7 of this register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DeadBatterySel::B1
    }
}
#[doc = "Field `DEAD_BATTERY_SEL` writer - dead_battery_sel"]
pub type DeadBatterySelW<'a, REG> = crate::BitWriter<'a, REG, DeadBatterySel>;
impl<'a, REG> DeadBatterySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select external dead_battery_n from IOMUX"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DeadBatterySel::B0)
    }
    #[doc = "select internal bit7 of this register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DeadBatterySel::B1)
    }
}
#[doc = "dead_battery_n\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadBatteryN {
    #[doc = "1: no dead battery"]
    B1 = 1,
    #[doc = "0: dead battery"]
    B0 = 0,
}
impl From<DeadBatteryN> for bool {
    #[inline(always)]
    fn from(variant: DeadBatteryN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEAD_BATTERY_N` reader - dead_battery_n"]
pub type DeadBatteryNR = crate::BitReader<DeadBatteryN>;
impl DeadBatteryNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeadBatteryN {
        match self.bits {
            true => DeadBatteryN::B1,
            false => DeadBatteryN::B0,
        }
    }
    #[doc = "no dead battery"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DeadBatteryN::B1
    }
    #[doc = "dead battery"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DeadBatteryN::B0
    }
}
#[doc = "Field `DEAD_BATTERY_N` writer - dead_battery_n"]
pub type DeadBatteryNW<'a, REG> = crate::BitWriter<'a, REG, DeadBatteryN>;
impl<'a, REG> DeadBatteryNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no dead battery"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DeadBatteryN::B1)
    }
    #[doc = "dead battery"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DeadBatteryN::B0)
    }
}
#[doc = "TypeC connect direction select\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TypecConnDirSel {
    #[doc = "0: select typec_conn_dir (bit0 of this register) to TypeC PHY"]
    B0 = 0,
    #[doc = "1: select TCPC ouput typec_con_dir to TypeC PHY"]
    B1 = 1,
}
impl From<TypecConnDirSel> for bool {
    #[inline(always)]
    fn from(variant: TypecConnDirSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TYPEC_CONN_DIR_SEL` reader - TypeC connect direction select"]
pub type TypecConnDirSelR = crate::BitReader<TypecConnDirSel>;
impl TypecConnDirSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TypecConnDirSel {
        match self.bits {
            false => TypecConnDirSel::B0,
            true => TypecConnDirSel::B1,
        }
    }
    #[doc = "select typec_conn_dir (bit0 of this register) to TypeC PHY"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TypecConnDirSel::B0
    }
    #[doc = "select TCPC ouput typec_con_dir to TypeC PHY"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TypecConnDirSel::B1
    }
}
#[doc = "Field `TYPEC_CONN_DIR_SEL` writer - TypeC connect direction select"]
pub type TypecConnDirSelW<'a, REG> = crate::BitWriter<'a, REG, TypecConnDirSel>;
impl<'a, REG> TypecConnDirSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select typec_conn_dir (bit0 of this register) to TypeC PHY"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TypecConnDirSel::B0)
    }
    #[doc = "select TCPC ouput typec_con_dir to TypeC PHY"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TypecConnDirSel::B1)
    }
}
#[doc = "TCPC Vbus On\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TcpcVbusOn {
    #[doc = "0: disable TCPC vbus supply"]
    B0 = 0,
    #[doc = "1: enable TCPC vbus supply"]
    B1 = 1,
}
impl From<TcpcVbusOn> for bool {
    #[inline(always)]
    fn from(variant: TcpcVbusOn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCPC_VBUS_ON` reader - TCPC Vbus On"]
pub type TcpcVbusOnR = crate::BitReader<TcpcVbusOn>;
impl TcpcVbusOnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TcpcVbusOn {
        match self.bits {
            false => TcpcVbusOn::B0,
            true => TcpcVbusOn::B1,
        }
    }
    #[doc = "disable TCPC vbus supply"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TcpcVbusOn::B0
    }
    #[doc = "enable TCPC vbus supply"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TcpcVbusOn::B1
    }
}
#[doc = "Field `TCPC_VBUS_ON` writer - TCPC Vbus On"]
pub type TcpcVbusOnW<'a, REG> = crate::BitWriter<'a, REG, TcpcVbusOn>;
impl<'a, REG> TcpcVbusOnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable TCPC vbus supply"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TcpcVbusOn::B0)
    }
    #[doc = "enable TCPC vbus supply"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TcpcVbusOn::B1)
    }
}
#[doc = "vbus valid select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VbusValidSel {
    #[doc = "0: use bvalid from usb2phy to usb3 controller"]
    B0 = 0,
    #[doc = "1: usb vbus_valid from TCPC to usb3 controller"]
    B1 = 1,
}
impl From<VbusValidSel> for bool {
    #[inline(always)]
    fn from(variant: VbusValidSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBUS_VALID_SEL` reader - vbus valid select"]
pub type VbusValidSelR = crate::BitReader<VbusValidSel>;
impl VbusValidSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VbusValidSel {
        match self.bits {
            false => VbusValidSel::B0,
            true => VbusValidSel::B1,
        }
    }
    #[doc = "use bvalid from usb2phy to usb3 controller"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VbusValidSel::B0
    }
    #[doc = "usb vbus_valid from TCPC to usb3 controller"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VbusValidSel::B1
    }
}
#[doc = "Field `VBUS_VALID_SEL` writer - vbus valid select"]
pub type VbusValidSelW<'a, REG> = crate::BitWriter<'a, REG, VbusValidSel>;
impl<'a, REG> VbusValidSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use bvalid from usb2phy to usb3 controller"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VbusValidSel::B0)
    }
    #[doc = "usb vbus_valid from TCPC to usb3 controller"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VbusValidSel::B1)
    }
}
#[doc = "cc1 overcurrent\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc1OvercurrentN {
    #[doc = "0: cc1 overcurrent"]
    B0 = 0,
    #[doc = "1: cc1 not overcurrent"]
    B1 = 1,
}
impl From<Cc1OvercurrentN> for bool {
    #[inline(always)]
    fn from(variant: Cc1OvercurrentN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC1_OVERCURRENT_N` reader - cc1 overcurrent"]
pub type Cc1OvercurrentNR = crate::BitReader<Cc1OvercurrentN>;
impl Cc1OvercurrentNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc1OvercurrentN {
        match self.bits {
            false => Cc1OvercurrentN::B0,
            true => Cc1OvercurrentN::B1,
        }
    }
    #[doc = "cc1 overcurrent"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cc1OvercurrentN::B0
    }
    #[doc = "cc1 not overcurrent"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cc1OvercurrentN::B1
    }
}
#[doc = "Field `CC1_OVERCURRENT_N` writer - cc1 overcurrent"]
pub type Cc1OvercurrentNW<'a, REG> = crate::BitWriter<'a, REG, Cc1OvercurrentN>;
impl<'a, REG> Cc1OvercurrentNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cc1 overcurrent"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1OvercurrentN::B0)
    }
    #[doc = "cc1 not overcurrent"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc1OvercurrentN::B1)
    }
}
#[doc = "cc2 overcurrent\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc2OvercurrentN {
    #[doc = "0: cc2 overcurrent"]
    B0 = 0,
    #[doc = "1: cc2 not overcurrent"]
    B1 = 1,
}
impl From<Cc2OvercurrentN> for bool {
    #[inline(always)]
    fn from(variant: Cc2OvercurrentN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC2_OVERCURRENT_N` reader - cc2 overcurrent"]
pub type Cc2OvercurrentNR = crate::BitReader<Cc2OvercurrentN>;
impl Cc2OvercurrentNR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc2OvercurrentN {
        match self.bits {
            false => Cc2OvercurrentN::B0,
            true => Cc2OvercurrentN::B1,
        }
    }
    #[doc = "cc2 overcurrent"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cc2OvercurrentN::B0
    }
    #[doc = "cc2 not overcurrent"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cc2OvercurrentN::B1
    }
}
#[doc = "Field `CC2_OVERCURRENT_N` writer - cc2 overcurrent"]
pub type Cc2OvercurrentNW<'a, REG> = crate::BitWriter<'a, REG, Cc2OvercurrentN>;
impl<'a, REG> Cc2OvercurrentNW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "cc2 overcurrent"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2OvercurrentN::B0)
    }
    #[doc = "cc2 not overcurrent"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cc2OvercurrentN::B1)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - TypeC PHY connect direction"]
    #[inline(always)]
    pub fn typec_conn_dir(&self) -> TypecConnDirR {
        TypecConnDirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Pipe interface data bus width\n\n0: 32bit data bus width, only support 32bit\n\ndata bus width."]
    #[inline(always)]
    pub fn pipe_data_bus_width(&self) -> PipeDataBusWidthR {
        PipeDataBusWidthR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - force usb3 to usb2 enable control"]
    #[inline(always)]
    pub fn usb3tousb2_en(&self) -> Usb3tousb2EnR {
        Usb3tousb2EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TCPC role trap"]
    #[inline(always)]
    pub fn tcpc_role_strap(&self) -> TcpcRoleStrapR {
        TcpcRoleStrapR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - dead_battery_sel"]
    #[inline(always)]
    pub fn dead_battery_sel(&self) -> DeadBatterySelR {
        DeadBatterySelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - dead_battery_n"]
    #[inline(always)]
    pub fn dead_battery_n(&self) -> DeadBatteryNR {
        DeadBatteryNR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TypeC connect direction select"]
    #[inline(always)]
    pub fn typec_conn_dir_sel(&self) -> TypecConnDirSelR {
        TypecConnDirSelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCPC Vbus On"]
    #[inline(always)]
    pub fn tcpc_vbus_on(&self) -> TcpcVbusOnR {
        TcpcVbusOnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - vbus valid select"]
    #[inline(always)]
    pub fn vbus_valid_sel(&self) -> VbusValidSelR {
        VbusValidSelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - cc1 overcurrent"]
    #[inline(always)]
    pub fn cc1_overcurrent_n(&self) -> Cc1OvercurrentNR {
        Cc1OvercurrentNR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - cc2 overcurrent"]
    #[inline(always)]
    pub fn cc2_overcurrent_n(&self) -> Cc2OvercurrentNR {
        Cc2OvercurrentNR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - TypeC PHY connect direction"]
    #[inline(always)]
    #[must_use]
    pub fn typec_conn_dir(&mut self) -> TypecConnDirW<GrfUsb3phy1Con0Spec> {
        TypecConnDirW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Pipe interface data bus width\n\n0: 32bit data bus width, only support 32bit\n\ndata bus width."]
    #[inline(always)]
    #[must_use]
    pub fn pipe_data_bus_width(&mut self) -> PipeDataBusWidthW<GrfUsb3phy1Con0Spec> {
        PipeDataBusWidthW::new(self, 1)
    }
    #[doc = "Bit 3 - force usb3 to usb2 enable control"]
    #[inline(always)]
    #[must_use]
    pub fn usb3tousb2_en(&mut self) -> Usb3tousb2EnW<GrfUsb3phy1Con0Spec> {
        Usb3tousb2EnW::new(self, 3)
    }
    #[doc = "Bits 4:5 - TCPC role trap"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc_role_strap(&mut self) -> TcpcRoleStrapW<GrfUsb3phy1Con0Spec> {
        TcpcRoleStrapW::new(self, 4)
    }
    #[doc = "Bit 6 - dead_battery_sel"]
    #[inline(always)]
    #[must_use]
    pub fn dead_battery_sel(&mut self) -> DeadBatterySelW<GrfUsb3phy1Con0Spec> {
        DeadBatterySelW::new(self, 6)
    }
    #[doc = "Bit 7 - dead_battery_n"]
    #[inline(always)]
    #[must_use]
    pub fn dead_battery_n(&mut self) -> DeadBatteryNW<GrfUsb3phy1Con0Spec> {
        DeadBatteryNW::new(self, 7)
    }
    #[doc = "Bit 8 - TypeC connect direction select"]
    #[inline(always)]
    #[must_use]
    pub fn typec_conn_dir_sel(&mut self) -> TypecConnDirSelW<GrfUsb3phy1Con0Spec> {
        TypecConnDirSelW::new(self, 8)
    }
    #[doc = "Bit 9 - TCPC Vbus On"]
    #[inline(always)]
    #[must_use]
    pub fn tcpc_vbus_on(&mut self) -> TcpcVbusOnW<GrfUsb3phy1Con0Spec> {
        TcpcVbusOnW::new(self, 9)
    }
    #[doc = "Bit 10 - vbus valid select"]
    #[inline(always)]
    #[must_use]
    pub fn vbus_valid_sel(&mut self) -> VbusValidSelW<GrfUsb3phy1Con0Spec> {
        VbusValidSelW::new(self, 10)
    }
    #[doc = "Bit 11 - cc1 overcurrent"]
    #[inline(always)]
    #[must_use]
    pub fn cc1_overcurrent_n(&mut self) -> Cc1OvercurrentNW<GrfUsb3phy1Con0Spec> {
        Cc1OvercurrentNW::new(self, 11)
    }
    #[doc = "Bit 12 - cc2 overcurrent"]
    #[inline(always)]
    #[must_use]
    pub fn cc2_overcurrent_n(&mut self) -> Cc2OvercurrentNW<GrfUsb3phy1Con0Spec> {
        Cc2OvercurrentNW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfUsb3phy1Con0Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "TypeC PHY/TCPD PHY/TCPC Control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_usb3phy1_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_usb3phy1_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfUsb3phy1Con0Spec;
impl crate::RegisterSpec for GrfUsb3phy1Con0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_usb3phy1_con0::R`](R) reader structure"]
impl crate::Readable for GrfUsb3phy1Con0Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_usb3phy1_con0::W`](W) writer structure"]
impl crate::Writable for GrfUsb3phy1Con0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_USB3PHY1_CON0 to value 0x19c8"]
impl crate::Resettable for GrfUsb3phy1Con0Spec {
    const RESET_VALUE: u32 = 0x19c8;
}
