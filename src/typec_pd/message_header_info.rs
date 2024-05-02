#[doc = "Register `MESSAGE_HEADER_INFO` reader"]
pub type R = crate::R<MessageHeaderInfoSpec>;
#[doc = "Register `MESSAGE_HEADER_INFO` writer"]
pub type W = crate::W<MessageHeaderInfoSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerRole {
    #[doc = "0: Sink"]
    B0 = 0,
    #[doc = "1: Source"]
    B1 = 1,
}
impl From<PowerRole> for bool {
    #[inline(always)]
    fn from(variant: PowerRole) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Power_Role` reader - "]
pub type PowerRoleR = crate::BitReader<PowerRole>;
impl PowerRoleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PowerRole {
        match self.bits {
            false => PowerRole::B0,
            true => PowerRole::B1,
        }
    }
    #[doc = "Sink"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == PowerRole::B0
    }
    #[doc = "Source"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == PowerRole::B1
    }
}
#[doc = "Field `Power_Role` writer - "]
pub type PowerRoleW<'a, REG> = crate::BitWriter<'a, REG, PowerRole>;
impl<'a, REG> PowerRoleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sink"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(PowerRole::B0)
    }
    #[doc = "Source"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(PowerRole::B1)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UsbPdSpecificationRevision {
    #[doc = "0: Revision1.0"]
    B00 = 0,
    #[doc = "1: Revision 2.0 10b - 11b:Reserved"]
    B01 = 1,
}
impl From<UsbPdSpecificationRevision> for u8 {
    #[inline(always)]
    fn from(variant: UsbPdSpecificationRevision) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UsbPdSpecificationRevision {
    type Ux = u8;
}
#[doc = "Field `USB_PD_Specification_Revision` reader - "]
pub type UsbPdSpecificationRevisionR = crate::FieldReader<UsbPdSpecificationRevision>;
impl UsbPdSpecificationRevisionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UsbPdSpecificationRevision> {
        match self.bits {
            0 => Some(UsbPdSpecificationRevision::B00),
            1 => Some(UsbPdSpecificationRevision::B01),
            _ => None,
        }
    }
    #[doc = "Revision1.0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == UsbPdSpecificationRevision::B00
    }
    #[doc = "Revision 2.0 10b - 11b:Reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == UsbPdSpecificationRevision::B01
    }
}
#[doc = "Field `USB_PD_Specification_Revision` writer - "]
pub type UsbPdSpecificationRevisionW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, UsbPdSpecificationRevision>;
impl<'a, REG> UsbPdSpecificationRevisionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Revision1.0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(UsbPdSpecificationRevision::B00)
    }
    #[doc = "Revision 2.0 10b - 11b:Reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(UsbPdSpecificationRevision::B01)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DataRole {
    #[doc = "0: UFP"]
    B0 = 0,
    #[doc = "1: DFP"]
    B1 = 1,
}
impl From<DataRole> for bool {
    #[inline(always)]
    fn from(variant: DataRole) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `Data_Role` reader - "]
pub type DataRoleR = crate::BitReader<DataRole>;
impl DataRoleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DataRole {
        match self.bits {
            false => DataRole::B0,
            true => DataRole::B1,
        }
    }
    #[doc = "UFP"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DataRole::B0
    }
    #[doc = "DFP"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DataRole::B1
    }
}
#[doc = "Field `Data_Role` writer - "]
pub type DataRoleW<'a, REG> = crate::BitWriter<'a, REG, DataRole>;
impl<'a, REG> DataRoleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UFP"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DataRole::B0)
    }
    #[doc = "DFP"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DataRole::B1)
    }
}
#[doc = "Field `Cable_Plug` reader - 0b: Message originated from Source, Sink, or \n\nDRP. 1b: Message originated \n\nfrom a Cable \n\nPlug"]
pub type CablePlugR = crate::BitReader;
#[doc = "Field `Cable_Plug` writer - 0b: Message originated from Source, Sink, or \n\nDRP. 1b: Message originated \n\nfrom a Cable \n\nPlug"]
pub type CablePlugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u32>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn power_role(&self) -> PowerRoleR {
        PowerRoleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn usb_pd_specification_revision(&self) -> UsbPdSpecificationRevisionR {
        UsbPdSpecificationRevisionR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn data_role(&self) -> DataRoleR {
        DataRoleR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0b: Message originated from Source, Sink, or \n\nDRP. 1b: Message originated \n\nfrom a Cable \n\nPlug"]
    #[inline(always)]
    pub fn cable_plug(&self) -> CablePlugR {
        CablePlugR::new(((self.bits >> 4) & 1) != 0)
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
    pub fn power_role(&mut self) -> PowerRoleW<MessageHeaderInfoSpec> {
        PowerRoleW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    #[must_use]
    pub fn usb_pd_specification_revision(
        &mut self,
    ) -> UsbPdSpecificationRevisionW<MessageHeaderInfoSpec> {
        UsbPdSpecificationRevisionW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn data_role(&mut self) -> DataRoleW<MessageHeaderInfoSpec> {
        DataRoleW::new(self, 3)
    }
    #[doc = "Bit 4 - 0b: Message originated from Source, Sink, or \n\nDRP. 1b: Message originated \n\nfrom a Cable \n\nPlug"]
    #[inline(always)]
    #[must_use]
    pub fn cable_plug(&mut self) -> CablePlugW<MessageHeaderInfoSpec> {
        CablePlugW::new(self, 4)
    }
    #[doc = "Bits 8:31 - Not used. Always write as 0 and ignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<MessageHeaderInfoSpec> {
        NotUsedW::new(self, 8)
    }
}
#[doc = "Message Header Information Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`message_header_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`message_header_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MessageHeaderInfoSpec;
impl crate::RegisterSpec for MessageHeaderInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`message_header_info::R`](R) reader structure"]
impl crate::Readable for MessageHeaderInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`message_header_info::W`](W) writer structure"]
impl crate::Writable for MessageHeaderInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MESSAGE_HEADER_INFO to value 0x02"]
impl crate::Resettable for MessageHeaderInfoSpec {
    const RESET_VALUE: u32 = 0x02;
}
