#[doc = "Register `USB3_DEVTEN` reader"]
pub type R = crate::R<Usb3DevtenSpec>;
#[doc = "Register `USB3_DEVTEN` writer"]
pub type W = crate::W<Usb3DevtenSpec>;
#[doc = "Disconnect Detected Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dissconnevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Dissconnevten> for bool {
    #[inline(always)]
    fn from(variant: Dissconnevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISSCONNEVTEN` reader - Disconnect Detected Event Enable"]
pub type DissconnevtenR = crate::BitReader<Dissconnevten>;
impl DissconnevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dissconnevten {
        match self.bits {
            true => Dissconnevten::B1,
            false => Dissconnevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dissconnevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dissconnevten::B0
    }
}
#[doc = "Field `DISSCONNEVTEN` writer - Disconnect Detected Event Enable"]
pub type DissconnevtenW<'a, REG> = crate::BitWriter<'a, REG, Dissconnevten>;
impl<'a, REG> DissconnevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dissconnevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dissconnevten::B0)
    }
}
#[doc = "USB Reset Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrstevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Usbrstevten> for bool {
    #[inline(always)]
    fn from(variant: Usbrstevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRSTEVTEN` reader - USB Reset Event Enable"]
pub type UsbrstevtenR = crate::BitReader<Usbrstevten>;
impl UsbrstevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrstevten {
        match self.bits {
            true => Usbrstevten::B1,
            false => Usbrstevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbrstevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbrstevten::B0
    }
}
#[doc = "Field `USBRSTEVTEN` writer - USB Reset Event Enable"]
pub type UsbrstevtenW<'a, REG> = crate::BitWriter<'a, REG, Usbrstevten>;
impl<'a, REG> UsbrstevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrstevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrstevten::B0)
    }
}
#[doc = "Connection Done Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Connectdoneevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Connectdoneevten> for bool {
    #[inline(always)]
    fn from(variant: Connectdoneevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONNECTDONEEVTEN` reader - Connection Done Event Enable"]
pub type ConnectdoneevtenR = crate::BitReader<Connectdoneevten>;
impl ConnectdoneevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Connectdoneevten {
        match self.bits {
            true => Connectdoneevten::B1,
            false => Connectdoneevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Connectdoneevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Connectdoneevten::B0
    }
}
#[doc = "Field `CONNECTDONEEVTEN` writer - Connection Done Event Enable"]
pub type ConnectdoneevtenW<'a, REG> = crate::BitWriter<'a, REG, Connectdoneevten>;
impl<'a, REG> ConnectdoneevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Connectdoneevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Connectdoneevten::B0)
    }
}
#[doc = "USB/Link State Change Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ulstcngen {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Ulstcngen> for bool {
    #[inline(always)]
    fn from(variant: Ulstcngen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ULSTCNGEN` reader - USB/Link State Change Event Enable"]
pub type UlstcngenR = crate::BitReader<Ulstcngen>;
impl UlstcngenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ulstcngen {
        match self.bits {
            true => Ulstcngen::B1,
            false => Ulstcngen::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ulstcngen::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ulstcngen::B0
    }
}
#[doc = "Field `ULSTCNGEN` writer - USB/Link State Change Event Enable"]
pub type UlstcngenW<'a, REG> = crate::BitWriter<'a, REG, Ulstcngen>;
impl<'a, REG> UlstcngenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ulstcngen::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ulstcngen::B0)
    }
}
#[doc = "Resume/Remote Wakeup Detected Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wkupevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Wkupevten> for bool {
    #[inline(always)]
    fn from(variant: Wkupevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEVTEN` reader - Resume/Remote Wakeup Detected Event Enable"]
pub type WkupevtenR = crate::BitReader<Wkupevten>;
impl WkupevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wkupevten {
        match self.bits {
            true => Wkupevten::B1,
            false => Wkupevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wkupevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wkupevten::B0
    }
}
#[doc = "Field `WKUPEVTEN` writer - Resume/Remote Wakeup Detected Event Enable"]
pub type WkupevtenW<'a, REG> = crate::BitWriter<'a, REG, Wkupevten>;
impl<'a, REG> WkupevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wkupevten::B0)
    }
}
#[doc = "Hibernation Request Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hibernationreqevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Hibernationreqevten> for bool {
    #[inline(always)]
    fn from(variant: Hibernationreqevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIBERNATIONREQEVTEN` reader - Hibernation Request Event Enable"]
pub type HibernationreqevtenR = crate::BitReader<Hibernationreqevten>;
impl HibernationreqevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hibernationreqevten {
        match self.bits {
            true => Hibernationreqevten::B1,
            false => Hibernationreqevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hibernationreqevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hibernationreqevten::B0
    }
}
#[doc = "Field `HIBERNATIONREQEVTEN` writer - Hibernation Request Event Enable"]
pub type HibernationreqevtenW<'a, REG> = crate::BitWriter<'a, REG, Hibernationreqevten>;
impl<'a, REG> HibernationreqevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hibernationreqevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hibernationreqevten::B0)
    }
}
#[doc = "U3/L2-L1 Suspend Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U3l2l1suspen {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<U3l2l1suspen> for bool {
    #[inline(always)]
    fn from(variant: U3l2l1suspen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `U3L2L1SUSPEN` reader - U3/L2-L1 Suspend Event Enable"]
pub type U3l2l1suspenR = crate::BitReader<U3l2l1suspen>;
impl U3l2l1suspenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U3l2l1suspen {
        match self.bits {
            true => U3l2l1suspen::B1,
            false => U3l2l1suspen::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == U3l2l1suspen::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == U3l2l1suspen::B0
    }
}
#[doc = "Field `U3L2L1SUSPEN` writer - U3/L2-L1 Suspend Event Enable"]
pub type U3l2l1suspenW<'a, REG> = crate::BitWriter<'a, REG, U3l2l1suspen>;
impl<'a, REG> U3l2l1suspenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(U3l2l1suspen::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(U3l2l1suspen::B0)
    }
}
#[doc = "Start of (u)frame Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Softevten> for bool {
    #[inline(always)]
    fn from(variant: Softevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTEVTEN` reader - Start of (u)frame Event Enable"]
pub type SoftevtenR = crate::BitReader<Softevten>;
impl SoftevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softevten {
        match self.bits {
            true => Softevten::B1,
            false => Softevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Softevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Softevten::B0
    }
}
#[doc = "Field `SOFTEVTEN` writer - Start of (u)frame Event Enable"]
pub type SoftevtenW<'a, REG> = crate::BitWriter<'a, REG, Softevten>;
impl<'a, REG> SoftevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Softevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Softevten::B0)
    }
}
#[doc = "Erratic Error Event Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errticerrevten {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Errticerrevten> for bool {
    #[inline(always)]
    fn from(variant: Errticerrevten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRTICERREVTEN` reader - Erratic Error Event Enable"]
pub type ErrticerrevtenR = crate::BitReader<Errticerrevten>;
impl ErrticerrevtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errticerrevten {
        match self.bits {
            true => Errticerrevten::B1,
            false => Errticerrevten::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Errticerrevten::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Errticerrevten::B0
    }
}
#[doc = "Field `ERRTICERREVTEN` writer - Erratic Error Event Enable"]
pub type ErrticerrevtenW<'a, REG> = crate::BitWriter<'a, REG, Errticerrevten>;
impl<'a, REG> ErrticerrevtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Errticerrevten::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Errticerrevten::B0)
    }
}
#[doc = "Vendor Device Test LMP Received Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vendevtstrcvden {
    #[doc = "1: Disable this event"]
    B1 = 1,
    #[doc = "0: Disable this event"]
    B0 = 0,
}
impl From<Vendevtstrcvden> for bool {
    #[inline(always)]
    fn from(variant: Vendevtstrcvden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VENDEVTSTRCVDEN` reader - Vendor Device Test LMP Received Event"]
pub type VendevtstrcvdenR = crate::BitReader<Vendevtstrcvden>;
impl VendevtstrcvdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vendevtstrcvden {
        match self.bits {
            true => Vendevtstrcvden::B1,
            false => Vendevtstrcvden::B0,
        }
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vendevtstrcvden::B1
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vendevtstrcvden::B0
    }
}
#[doc = "Field `VENDEVTSTRCVDEN` writer - Vendor Device Test LMP Received Event"]
pub type VendevtstrcvdenW<'a, REG> = crate::BitWriter<'a, REG, Vendevtstrcvden>;
impl<'a, REG> VendevtstrcvdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vendevtstrcvden::B1)
    }
    #[doc = "Disable this event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vendevtstrcvden::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Disconnect Detected Event Enable"]
    #[inline(always)]
    pub fn dissconnevten(&self) -> DissconnevtenR {
        DissconnevtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Reset Event Enable"]
    #[inline(always)]
    pub fn usbrstevten(&self) -> UsbrstevtenR {
        UsbrstevtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Connection Done Event Enable"]
    #[inline(always)]
    pub fn connectdoneevten(&self) -> ConnectdoneevtenR {
        ConnectdoneevtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB/Link State Change Event Enable"]
    #[inline(always)]
    pub fn ulstcngen(&self) -> UlstcngenR {
        UlstcngenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resume/Remote Wakeup Detected Event Enable"]
    #[inline(always)]
    pub fn wkupevten(&self) -> WkupevtenR {
        WkupevtenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hibernation Request Event Enable"]
    #[inline(always)]
    pub fn hibernationreqevten(&self) -> HibernationreqevtenR {
        HibernationreqevtenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - U3/L2-L1 Suspend Event Enable"]
    #[inline(always)]
    pub fn u3l2l1suspen(&self) -> U3l2l1suspenR {
        U3l2l1suspenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Start of (u)frame Event Enable"]
    #[inline(always)]
    pub fn softevten(&self) -> SoftevtenR {
        SoftevtenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Erratic Error Event Enable"]
    #[inline(always)]
    pub fn errticerrevten(&self) -> ErrticerrevtenR {
        ErrticerrevtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Vendor Device Test LMP Received Event"]
    #[inline(always)]
    pub fn vendevtstrcvden(&self) -> VendevtstrcvdenR {
        VendevtstrcvdenR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disconnect Detected Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dissconnevten(&mut self) -> DissconnevtenW<Usb3DevtenSpec> {
        DissconnevtenW::new(self, 0)
    }
    #[doc = "Bit 1 - USB Reset Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbrstevten(&mut self) -> UsbrstevtenW<Usb3DevtenSpec> {
        UsbrstevtenW::new(self, 1)
    }
    #[doc = "Bit 2 - Connection Done Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn connectdoneevten(&mut self) -> ConnectdoneevtenW<Usb3DevtenSpec> {
        ConnectdoneevtenW::new(self, 2)
    }
    #[doc = "Bit 3 - USB/Link State Change Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ulstcngen(&mut self) -> UlstcngenW<Usb3DevtenSpec> {
        UlstcngenW::new(self, 3)
    }
    #[doc = "Bit 4 - Resume/Remote Wakeup Detected Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkupevten(&mut self) -> WkupevtenW<Usb3DevtenSpec> {
        WkupevtenW::new(self, 4)
    }
    #[doc = "Bit 5 - Hibernation Request Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hibernationreqevten(&mut self) -> HibernationreqevtenW<Usb3DevtenSpec> {
        HibernationreqevtenW::new(self, 5)
    }
    #[doc = "Bit 6 - U3/L2-L1 Suspend Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn u3l2l1suspen(&mut self) -> U3l2l1suspenW<Usb3DevtenSpec> {
        U3l2l1suspenW::new(self, 6)
    }
    #[doc = "Bit 7 - Start of (u)frame Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn softevten(&mut self) -> SoftevtenW<Usb3DevtenSpec> {
        SoftevtenW::new(self, 7)
    }
    #[doc = "Bit 9 - Erratic Error Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn errticerrevten(&mut self) -> ErrticerrevtenW<Usb3DevtenSpec> {
        ErrticerrevtenW::new(self, 9)
    }
    #[doc = "Bit 12 - Vendor Device Test LMP Received Event"]
    #[inline(always)]
    #[must_use]
    pub fn vendevtstrcvden(&mut self) -> VendevtstrcvdenW<Usb3DevtenSpec> {
        VendevtstrcvdenW::new(self, 12)
    }
}
#[doc = "Device Event Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb3_devten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb3_devten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usb3DevtenSpec;
impl crate::RegisterSpec for Usb3DevtenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb3_devten::R`](R) reader structure"]
impl crate::Readable for Usb3DevtenSpec {}
#[doc = "`write(|w| ..)` method takes [`usb3_devten::W`](W) writer structure"]
impl crate::Writable for Usb3DevtenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USB3_DEVTEN to value 0"]
impl crate::Resettable for Usb3DevtenSpec {
    const RESET_VALUE: u32 = 0;
}
